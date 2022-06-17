use arcstr::ArcStr;
use glow::HasContext;
use na::{Matrix4, Vector4};

use super::scene::Scene;
use crate::geom::camera::Camera;
use crate::render_gl::debug::check_error;
use crate::render_gl::{buffer, data, texture};
use crate::resources::Resources;
use crate::{render_gl, time, GL};

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
  #[location = 0]
  pos: data::f32_f32_f32,
  #[location = 1]
  tex: data::f32_f32,
  #[location = 2]
  nor: data::f32_f32_f32,
}

pub struct Cube {
  program: render_gl::Program,
  _vbo: buffer::ArrayBuffer,
  _ebo: buffer::ElementArrayBuffer,
  vao: buffer::VertexArray,
  texture: Vec<texture::Texture>,
  camera: Camera,
  ambient: na::Vector4<f32>,
}
fn gen_vertices() -> Vec<Vertex> {
  vec![
    // 正面
    //  2  1
    //  3  0
    Vertex {
      pos: (5.0, -5.0, 5.0).into(),
      tex: (1.0, 0.0).into(),
      nor: (0.0, 0.0, 1.0).into(),
    }, // bottom right
    Vertex {
      pos: (5.0, 5.0, 5.0).into(),
      tex: (1.0, 1.0).into(),
      nor: (0.0, 0.0, 1.0).into(),
    }, // top right
    Vertex {
      pos: (-5.0, 5.0, 5.0).into(),
      tex: (0.0, 1.0).into(),
      nor: (0.0, 0.0, 1.0).into(),
    }, // top left
    Vertex {
      pos: (-5.0, -5.0, 5.0).into(),
      tex: (0.0, 0.0).into(),
      nor: (0.0, 0.0, 1.0).into(),
    }, // bottom left
    // 顶面
    //  2  1
    //  3  0
    Vertex {
      pos: (5.0, 5.0, 5.0).into(),
      tex: (1.0, 0.0).into(),
      nor: (0.0, 1.0, 0.0).into(),
    }, // bottom right
    Vertex {
      pos: (5.0, 5.0, -5.0).into(),
      tex: (1.0, 1.0).into(),
      nor: (0.0, 1.0, 0.0).into(),
    }, // top right
    Vertex {
      pos: (-5.0, 5.0, -5.0).into(),
      tex: (0.0, 1.0).into(),
      nor: (0.0, 1.0, 0.0).into(),
    }, // top left
    Vertex {
      pos: (-5.0, 5.0, 5.0).into(),
      tex: (0.0, 0.0).into(),
      nor: (0.0, 1.0, 0.0).into(),
    }, // bottom left
    // 背面
    //  2  1
    //  3  0
    Vertex {
      pos: (5.0, 5.0, -5.0).into(),
      tex: (1.0, 0.0).into(),
      nor: (0.0, 0.0, -1.0).into(),
    }, // bottom right
    Vertex {
      pos: (5.0, -5.0, -5.0).into(),
      tex: (1.0, 1.0).into(),
      nor: (0.0, 0.0, -1.0).into(),
    }, // top right
    Vertex {
      pos: (-5.0, -5.0, -5.0).into(),
      tex: (0.0, 1.0).into(),
      nor: (0.0, 0.0, -1.0).into(),
    }, // top left
    Vertex {
      pos: (-5.0, 5.0, -5.0).into(),
      tex: (0.0, 0.0).into(),
      nor: (0.0, 0.0, -1.0).into(),
    }, // bottom left
    // 底面
    //  2  1
    //  3  0
    Vertex {
      pos: (5.0, -5.0, -5.0).into(),
      tex: (1.0, 0.0).into(),
      nor: (0.0, -1.0, 0.0).into(),
    }, // bottom right
    Vertex {
      pos: (5.0, -5.0, 5.0).into(),
      tex: (1.0, 1.0).into(),
      nor: (0.0, -1.0, 0.0).into(),
    }, // top right
    Vertex {
      pos: (-5.0, -5.0, 5.0).into(),
      tex: (0.0, 1.0).into(),
      nor: (0.0, -1.0, 0.0).into(),
    }, // top left
    Vertex {
      pos: (-5.0, -5.0, -5.0).into(),
      tex: (0.0, 0.0).into(),
      nor: (0.0, -1.0, 0.0).into(),
    }, // bottom left
    // 左侧面
    //  2  1
    //  3  0
    Vertex {
      pos: (-5.0, -5.0, 5.0).into(),
      tex: (1.0, 0.0).into(),
      nor: (-1.0, 0.0, 0.0).into(),
    }, // bottom right
    Vertex {
      pos: (-5.0, 5.0, 5.0).into(),
      tex: (1.0, 1.0).into(),
      nor: (-1.0, 0.0, 0.0).into(),
    }, // top right
    Vertex {
      pos: (-5.0, 5.0, -5.0).into(),
      tex: (0.0, 1.0).into(),
      nor: (-1.0, 0.0, 0.0).into(),
    }, // top left
    Vertex {
      pos: (-5.0, -5.0, -5.0).into(),
      tex: (0.0, 0.0).into(),
      nor: (-1.0, 0.0, 0.0).into(),
    }, // bottom left
    // 右侧面
    //  2  1
    //  3  0
    Vertex {
      pos: (5.0, -5.0, -5.0).into(),
      tex: (1.0, 0.0).into(),
      nor: (1.0, 0.0, 0.0).into(),
    }, // bottom right
    Vertex {
      pos: (5.0, 5.0, -5.0).into(),
      tex: (1.0, 1.0).into(),
      nor: (1.0, 0.0, 0.0).into(),
    }, // top right
    Vertex {
      pos: (5.0, 5.0, 5.0).into(),
      tex: (0.0, 1.0).into(),
      nor: (1.0, 0.0, 0.0).into(),
    }, // top left
    Vertex {
      pos: (5.0, -5.0, 5.0).into(),
      tex: (0.0, 0.0).into(),
      nor: (1.0, 0.0, 0.0).into(),
    }, // bottom left
  ]
}
fn gen_indices(vertices: &Vec<Vertex>) -> Vec<u32> {
  let num = (vertices.len() / 4) as u32;
  let mut res = Vec::<u32>::new();
  for i in 0..num {
    let mut append = vec![
      0 + 4 * i,
      1 + 4 * i,
      2 + 4 * i,
      0 + 4 * i,
      2 + 4 * i,
      3 + 4 * i,
    ];
    res.append(&mut append)
  }
  res
}

impl Cube {
  pub fn new(res: &Resources) -> Result<Cube, anyhow::Error> {
    let program = render_gl::Program::from_res(res, "shaders/phong")?;

    let vertices: Vec<Vertex> = gen_vertices();
    let indices: Vec<u32> = gen_indices(&vertices);

    let vbo = buffer::ArrayBuffer::new();
    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();
    let ebo = buffer::ElementArrayBuffer::new();
    ebo.bind();
    ebo.static_draw_data(&indices);
    ebo.unbind();
    let vao = buffer::VertexArray::new();

    vao.bind();
    vbo.bind();
    ebo.bind();
    Vertex::vertex_attrib_pointers();
    // 注意这里有一个自动绑定机制
    vao.unbind();
    let texture0 = texture::Texture::from_res(&res, "textures/container.jpg")?;
    //告诉OpenGL每个着色器采样器属于哪个纹理单元
    program.upload_texture_slot("texture0", 0);

    Ok(Cube {
      program,
      _vbo: vbo,
      _ebo: ebo,
      vao,
      texture: vec![texture0],
      camera: Camera::new(na::Point3::new(0.0, 0.0, 0.0)),
      ambient: Vector4::<f32>::new(1.0, 1.0, 1.0, 1.0),
    })
  }
  pub fn update(&mut self, ambient: &na::Vector4<f32>) -> Option<()> {
    self.ambient = ambient.clone();
    Some(())
  }
}
impl Scene for Cube {
  fn render(&self, aspect: f32) -> Option<()> {
    check_error();
    self.program.set_used();
    self.vao.bind();
    let model_mat = na::Matrix4::<f32>::identity();
    let view_mat = self.camera.get_view_mat();
    let proj_mat = self.camera.get_proj_mat(aspect);
    let mvp_mat = proj_mat * view_mat * model_mat;

    let nor_mat = (view_mat * model_mat)
      .fixed_resize::<3, 3>(0.0)
      .try_inverse()
      .unwrap()
      .transpose();
    unsafe {
      // 绑定纹理到对应的纹理单元
      GL.active_texture(glow::TEXTURE0);
      self.texture.get(0)?.bind();
      self.program.upload_mat4("MVPMat", &mvp_mat);
      // self.program.upload_mat3("NormalMat", &nor_mat.try_into().unwrap());
      self.program.upload_vec4("Ambient", &self.ambient);
      GL.draw_elements(glow::TRIANGLES, 36, glow::UNSIGNED_INT, 0);
    }
    self.vao.unbind();
    self.program.detach();
    Some(())
  }

  fn get_camera(&mut self) -> &mut Camera {
    &mut self.camera
  }

  fn get_name(&self) -> ArcStr {
    ArcStr::from("phong")
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
  fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
    self
  }
}
