use na::Vector1;

use crate::geom::camera::Camera;
use crate::geom::light::{DirectLight, Light, PointLight};
use crate::render_gl;
use crate::render_gl::debug::check_error;
use crate::render_gl::{buffer, data, texture};
use crate::resources::Resources;
use na::Vector3;
lazy_static! {
  static ref POINT_LIGHT_POSITIONS: Vec<Vector3<f32>> = vec![
    Vector3::new(10.0, 0.0, 0.0),
    Vector3::new(0.0, 15.0, 0.0),
    Vector3::new(0.0, 0.0, 20.0),
    Vector3::new(0.0, -25.0, 0.0),
    Vector3::new(0.0, 0.0, -30.0),
    Vector3::new(-35.0, 0.0, 0.0)
  ];
  static ref LIGHT_COLORS: Vec<Vector3<f32>> = vec![
    Vector3::new(1.0, 0.0, 0.0),
    Vector3::new(0.5, 0.5, 0.0),
    Vector3::new(0.0, 1.0, 0.0),
    Vector3::new(0.0, 0.5, 0.5),
    Vector3::new(0.0, 0.0, 1.0),
    Vector3::new(0.5, 0.0, 0.5)
  ];
}

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
  #[location = 0]
  pos: data::f32_f32_f32,
  #[location = 1]
  tex: data::f32_f32,
  #[location = 2]
  normal: data::f32_f32_f32,
}

pub struct LightCube {
  program: render_gl::Program,
  light_program: render_gl::Program,
  _vbo: buffer::ArrayBuffer,
  _ebo: buffer::ElementArrayBuffer,
  vao: buffer::VertexArray,
  texture: Vec<texture::Texture>,
  pub camera: Camera,
}
fn gen_vertices() -> Vec<Vertex> {
  vec![
    //   2  1
    //  3  0
    Vertex {
      pos: (5.0, -5.0, 5.0).into(),
      tex: (1.0, 0.0).into(),
      normal: (0.0, 0.0, -1.0).into(),
    }, // bottom right
    Vertex {
      pos: (5.0, 5.0, 5.0).into(),
      tex: (1.0, 1.0).into(),
      normal: (0.0, 0.0, -1.0).into(),
    }, // top right
    Vertex {
      pos: (-5.0, 5.0, 5.0).into(),
      tex: (0.0, 1.0).into(),
      normal: (0.0, 0.0, -1.0).into(),
    }, // top left
    Vertex {
      pos: (-5.0, -5.0, 5.0).into(),
      tex: (0.0, 0.0).into(),
      normal: (0.0, 0.0, -1.0).into(),
    }, // bottom left
    //   2  1
    //  3  0
    Vertex {
      pos: (5.0, 5.0, 5.0).into(),
      tex: (1.0, 0.0).into(),
      normal: (0.0, 1.0, 0.0).into(),
    }, // bottom right
    Vertex {
      pos: (5.0, 5.0, -5.0).into(),
      tex: (1.0, 1.0).into(),
      normal: (0.0, 1.0, 0.0).into(),
    }, // top right
    Vertex {
      pos: (-5.0, 5.0, -5.0).into(),
      tex: (0.0, 1.0).into(),
      normal: (0.0, 1.0, 0.0).into(),
    }, // top left
    Vertex {
      pos: (-5.0, 5.0, 5.0).into(),
      tex: (0.0, 0.0).into(),
      normal: (0.0, 1.0, 0.0).into(),
    }, // bottom left
    //   2  1
    //  3  0
    Vertex {
      pos: (5.0, 5.0, -5.0).into(),
      tex: (1.0, 0.0).into(),
      normal: (0.0, 0.0, 1.0).into(),
    }, // bottom right
    Vertex {
      pos: (5.0, -5.0, -5.0).into(),
      tex: (1.0, 1.0).into(),
      normal: (0.0, 0.0, 1.0).into(),
    }, // top right
    Vertex {
      pos: (-5.0, -5.0, -5.0).into(),
      tex: (0.0, 1.0).into(),
      normal: (0.0, 0.0, 1.0).into(),
    }, // top left
    Vertex {
      pos: (-5.0, 5.0, -5.0).into(),
      tex: (0.0, 0.0).into(),
      normal: (0.0, 0.0, 1.0).into(),
    }, // bottom left
    //   2  1
    //  3  0
    Vertex {
      pos: (5.0, -5.0, -5.0).into(),
      tex: (1.0, 0.0).into(),
      normal: (0.0, -1.0, 0.0).into(),
    }, // bottom right
    Vertex {
      pos: (5.0, -5.0, 5.0).into(),
      tex: (1.0, 1.0).into(),
      normal: (0.0, -1.0, 0.0).into(),
    }, // top right
    Vertex {
      pos: (-5.0, -5.0, 5.0).into(),
      tex: (0.0, 1.0).into(),
      normal: (0.0, -1.0, 0.0).into(),
    }, // top left
    Vertex {
      pos: (-5.0, -5.0, -5.0).into(),
      tex: (0.0, 0.0).into(),
      normal: (0.0, -1.0, 0.0).into(),
    }, // bottom left
    //   2  1
    //  3  0
    Vertex {
      pos: (-5.0, -5.0, 5.0).into(),
      tex: (1.0, 0.0).into(),
      normal: (-1.0, 0.0, 0.0).into(),
    }, // bottom right
    Vertex {
      pos: (-5.0, 5.0, 5.0).into(),
      tex: (1.0, 1.0).into(),
      normal: (-1.0, 0.0, 0.0).into(),
    }, // top right
    Vertex {
      pos: (-5.0, 5.0, -5.0).into(),
      tex: (0.0, 1.0).into(),
      normal: (-1.0, 0.0, 0.0).into(),
    }, // top left
    Vertex {
      pos: (-5.0, -5.0, -5.0).into(),
      tex: (0.0, 0.0).into(),
      normal: (-1.0, 0.0, 0.0).into(),
    }, // bottom left
    //   2  1
    //  3  0
    Vertex {
      pos: (5.0, -5.0, -5.0).into(),
      tex: (1.0, 0.0).into(),
      normal: (1.0, 0.0, 0.0).into(),
    }, // bottom right
    Vertex {
      pos: (5.0, 5.0, -5.0).into(),
      tex: (1.0, 1.0).into(),
      normal: (1.0, 0.0, 0.0).into(),
    }, // top right
    Vertex {
      pos: (5.0, 5.0, 5.0).into(),
      tex: (0.0, 1.0).into(),
      normal: (1.0, 0.0, 0.0).into(),
    }, // top left
    Vertex {
      pos: (5.0, -5.0, 5.0).into(),
      tex: (0.0, 0.0).into(),
      normal: (1.0, 0.0, 0.0).into(),
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

impl LightCube {
  pub fn new(res: &Resources, gl: &gl::Gl) -> Result<LightCube, anyhow::Error> {
    let program = render_gl::Program::from_res(gl, res, "shaders/light_cube")?;
    let light_program = render_gl::Program::from_res(gl, res, "shaders/light")?;

    let dirLight = DirectLight {
      light: Light {
        is_on: true,
        ambient: Vector3::new(0.05, 0.05, 0.05),
        diffuse: Vector3::new(0.4, 0.4, 0.4),
        specular: Vector3::new(0.5, 0.5, 0.5),
      },
      direction: Vector3::new(-0.2, -1.0, -0.3),
    };
    let mut point_lights: Vec<PointLight> = Vec::new();
    for i in 0..6 {
      let point_light = PointLight {
        light: Light {
          is_on: true,
          ambient: 0.05 * LIGHT_COLORS.get(i).unwrap().clone(),
          diffuse: 0.8 * LIGHT_COLORS.get(i).unwrap().clone(),
          specular: LIGHT_COLORS.get(i).unwrap().clone(),
        },
        position: POINT_LIGHT_POSITIONS.get(i).unwrap().clone(),
        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,
      };
      point_lights.push(point_light);
    }

    let vertices: Vec<Vertex> = gen_vertices();
    let indices: Vec<u32> = gen_indices(&vertices);

    let vbo = buffer::ArrayBuffer::new(gl);
    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();
    let ebo = buffer::ElementArrayBuffer::new(gl);
    ebo.bind();
    ebo.static_draw_data(&indices);
    ebo.unbind();
    let vao = buffer::VertexArray::new(gl);

    vao.bind();
    vbo.bind();
    ebo.bind();
    Vertex::vertex_attrib_pointers(gl);
    // 注意这里有一个自动绑定机制
    vao.unbind();
    let texture0 = texture::Texture::from_res(&gl, &res, "textures/container.jpg")?;
    let texture1 = texture::Texture::from_res(&gl, res, "textures/awesomeface.png")?;
    //告诉OpenGL每个着色器采样器属于哪个纹理单元
    program.upload_texture_slot("texture0", 0);
    program.upload_texture_slot("texture1", 1);

    let light_vao = buffer::VertexArray::new(gl);
    Ok(LightCube {
      program,
      light_program,
      _vbo: vbo,
      _ebo: ebo,
      vao,
      texture: vec![texture0, texture1],
      camera: Camera::new(na::Point3::new(0.0, 0.0, 0.0)),
    })
  }
  pub fn render(&self, gl: &gl::Gl) -> Option<()> {
    check_error(gl);
    self.program.set_used();
    self.vao.bind();
    unsafe {
      // 绑定两个纹理到对应的纹理单元
      gl.ActiveTexture(gl::TEXTURE0);
      self.texture.get(0)?.bind();
      gl.ActiveTexture(gl::TEXTURE1);
      self.texture.get(1)?.bind();
      self
        .program
        .upload_mat4("vp_proj", &self.camera.get_pv_mat());
      gl.DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, std::ptr::null())
    }
    self.vao.unbind();
    self.program.detach();
    Some(())
  }
}
