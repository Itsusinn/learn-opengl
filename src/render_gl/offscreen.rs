use std::sync::RwLock;

use crate::render_gl;
use crate::render_gl::debug::check_error;
use crate::render_gl::frame_buffer::FrameBuffer;
use crate::render_gl::{buffer, data};
use crate::resources::Resources;


pub struct Offscreen {
  pub frame_buffer: RwLock<FrameBuffer>,
  pub render: RwLock<Render>,
  gl: gl::Gl,
  res: Resources
}
impl Offscreen {
  pub fn new(gl: &gl::Gl,res: &Resources,width: i32, height: i32) -> anyhow::Result<Self> {
    let frame_buffer = FrameBuffer::new(&gl, width, height);
    let render = Render::new(&res, &gl, &frame_buffer)?;
    Ok(Self {
      frame_buffer: RwLock::new(frame_buffer),
      render: RwLock::new(render),
      gl: gl.clone(),
      res: res.clone(),
    })
  }
  pub fn resize(&self,width: i32, height: i32) -> anyhow::Result<()> {
    let mut frame_buffer = self.frame_buffer.write().unwrap();
    *frame_buffer = FrameBuffer::new(&self.gl, width, height);
    let mut render = self.render.write().unwrap();
    *render = Render::new(&self.res, &self.gl, &frame_buffer)?;
    Ok(())
  }
  pub fn bind(&self) {
    self.frame_buffer.read().unwrap().bind();
  }
  pub fn detach(&self) {
    self.frame_buffer.read().unwrap().detach();
  }
  pub fn render_output(&self) {
    self.render.read().unwrap().render(&self.gl);
  }
}

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
  #[location = 0]
  pos: data::f32_f32_f32,
  #[location = 1]
  tex: data::f32_f32,
}

pub struct Render {
  program: render_gl::Program,
  _vbo: buffer::ArrayBuffer,
  _ebo: buffer::ElementArrayBuffer,
  vao: buffer::VertexArray,
  texture_id: u32,
  gl: gl::Gl
}

impl Render {
  pub fn new(
    res: &Resources,
    gl: &gl::Gl,
    frame_buffer: &FrameBuffer,
  ) -> Result<Self, anyhow::Error> {
    let program = render_gl::Program::from_res(gl, res, "shaders/offscreen")?;

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
    // program.upload_texture_slot("frame", 0);
    Ok(Self {
      program,
      _vbo: vbo,
      _ebo: ebo,
      vao,
      texture_id: frame_buffer.texture_id,
      gl: gl.clone(),
    })
  }
  pub fn render(&self, gl: &gl::Gl) -> Option<()> {
    check_error(gl);
    self.program.set_used();
    self.vao.bind();
    unsafe {
      gl.ActiveTexture(gl::TEXTURE0);
      gl.BindTexture(gl::TEXTURE_2D, self.texture_id);
      gl.DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null())
    }
    self.vao.unbind();
    self.program.detach();
    check_error(gl);
    Some(())
  }
}
impl Drop for Render {
  fn drop(&mut self) {
    unsafe {
      self.gl.DeleteTextures(1, &self.texture_id);
    }
  }
}
