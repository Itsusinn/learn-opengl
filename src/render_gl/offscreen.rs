use std::sync::RwLock;

use glow::HasContext;

use crate::render_gl::buffer;
use crate::render_gl::data::*;
use crate::render_gl::debug::check_error;
use crate::render_gl::frame_buffer::FrameBuffer;
use crate::resources::Resources;
use crate::{render_gl, GL};

pub struct OffScreen {
  pub frame_buffer: RwLock<FrameBuffer>,
  pub render: RwLock<Render>,
  res: Resources,
}
impl OffScreen {
  pub fn new(res: &Resources, width: i32, height: i32) -> anyhow::Result<Self> {
    let frame_buffer = FrameBuffer::new(width, height);
    let render = Render::new(&res, &frame_buffer)?;
    Ok(Self {
      frame_buffer: RwLock::new(frame_buffer),
      render: RwLock::new(render),
      res: res.clone(),
    })
  }
  pub fn resize(&self, width: i32, height: i32) -> anyhow::Result<()> {
    let mut frame_buffer = self.frame_buffer.write().unwrap();
    *frame_buffer = FrameBuffer::new(width, height);
    let mut render = self.render.write().unwrap();
    *render = Render::new(&self.res, &frame_buffer)?;
    Ok(())
  }
  pub fn bind(&self) {
    self.frame_buffer.read().unwrap().bind();
  }
  pub fn detach(&self) {
    self.frame_buffer.read().unwrap().detach();
  }
  pub fn render_output(&self) {
    self.render.read().unwrap().render();
  }
}

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
  #[location = 0]
  pos: f32_f32_f32,
  #[location = 1]
  tex: f32_f32,
}

pub struct Render {
  program: render_gl::Program,
  _vbo: buffer::ArrayBuffer,
  _ebo: buffer::ElementArrayBuffer,
  vao: buffer::VertexArray,
  texture: glow::Texture,
}

impl Render {
  pub fn new(res: &Resources, frame_buffer: &FrameBuffer) -> Result<Self, anyhow::Error> {
    let program = render_gl::Program::from_res(res, "shaders/offscreen")?;

    let vertices: Vec<Vertex> = vec![
      //   2  1
      //  3  0
      Vertex {
        pos: (1.0, -1.0, -0.5).into(),
        tex: (1.0, 0.0).into(),
      }, // bottom right
      Vertex {
        pos: (1.0, 1.0, -0.5).into(),
        tex: (1.0, 1.0).into(),
      }, // top right
      Vertex {
        pos: (-1.0, 1.0, -0.5).into(),
        tex: (0.0, 1.0).into(),
      }, // top left
      Vertex {
        pos: (-1.0, -1.0, -0.5).into(),
        tex: (0.0, 0.0).into(),
      }, // bottom left
    ];
    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];
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
    // program.upload_texture_slot("frame", 0);
    Ok(Self {
      program,
      _vbo: vbo,
      _ebo: ebo,
      vao,
      texture: frame_buffer.texture,
    })
  }
  pub fn render(&self) -> Option<()> {
    check_error();
    self.program.set_used();
    self.vao.bind();
    unsafe {
      GL.active_texture(glow::TEXTURE0);
      GL.bind_texture(glow::TEXTURE_2D, Some(self.texture));
      GL.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
    }
    self.vao.unbind();
    self.program.detach();
    check_error();
    Some(())
  }
}
