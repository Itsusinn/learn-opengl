use glow::HasContext;

use crate::render_gl::debug;
use crate::GL;

pub struct FrameBuffer {
  pub width: i32,
  pub height: i32,
  // fbo
  pub inner: glow::Framebuffer,
  pub texture: glow::Texture,
  pub rbo: glow::Renderbuffer,
}
impl Drop for FrameBuffer {
  fn drop(&mut self) {
    unsafe {
      GL.delete_framebuffer(self.inner);
    }
  }
}
impl FrameBuffer {
  pub fn new(width: i32, height: i32) -> Self {
    let fbo = unsafe { GL.create_framebuffer().unwrap() };
    unsafe {
      GL.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
    }
    let texture = unsafe { GL.create_texture().unwrap() };
    // 生成空白纹理并attach到FBO上
    unsafe {
      GL.bind_texture(glow::TEXTURE_2D, Some(texture));
      GL.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MIN_FILTER,
        glow::LINEAR as i32,
      );
      GL.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MAG_FILTER,
        glow::LINEAR as i32,
      );
      GL.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        glow::RGB as i32,
        width,
        height,
        0,
        glow::RGB,
        glow::UNSIGNED_BYTE,
        None,
      );
      GL.bind_texture(glow::TEXTURE_2D, None);

      GL.framebuffer_texture_2d(
        glow::FRAMEBUFFER,
        glow::COLOR_ATTACHMENT0,
        glow::TEXTURE_2D,
        Some(texture),
        0,
      );
    }
    // 生成render buffer以缓冲深度和模板信息
    let rbo = unsafe { GL.create_renderbuffer().unwrap() };
    unsafe {
      GL.bind_renderbuffer(glow::RENDERBUFFER, Some(rbo));
      // 通过glRenderbufferStorage API给RBO创建、初始化存储空间
      GL.renderbuffer_storage(glow::RENDERBUFFER, glow::DEPTH_COMPONENT32, width, height);
      // glFramebufferRenderbuffer API 将指定的RBO关联到GPU当前的FBO上。
      GL.framebuffer_renderbuffer(
        glow::FRAMEBUFFER,
        glow::DEPTH_ATTACHMENT,
        glow::RENDERBUFFER,
        Some(rbo),
      );
      if GL.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
        println!("帧缓冲创建失败");
        debug::check_error();
      }
      GL.bind_renderbuffer(glow::RENDERBUFFER, None);
      GL.bind_framebuffer(glow::FRAMEBUFFER, None);
      debug::check_error();
    }
    Self {
      inner: fbo,
      width,
      height,
      texture,
      rbo,
    }
  }

  pub fn bind(&self) {
    unsafe {
      GL.bind_framebuffer(glow::FRAMEBUFFER, Some(self.inner));
    }
  }
  pub fn detach(&self) {
    unsafe {
      GL.bind_framebuffer(glow::FRAMEBUFFER, None);
    }
  }
}
