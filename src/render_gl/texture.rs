use std::{io, path::PathBuf};

use another::any_as_u8_slice;
use glow::HasContext;
use stb_image::image::{self, LoadResult};
use thiserror::Error;

use crate::{resources::Resources, GL};

#[derive(Debug, Error)]
pub enum Error {
  #[error("I/O 错误")]
  IO(#[from] io::Error),
  #[error("纹理加载错误 ,原因:{0}")]
  LoadError(String),
}
pub struct Texture {
  inner: glow::Texture,
}
impl Texture {
  pub fn new(path: PathBuf) -> Result<Texture, Error> {
    image::stbi_set_flip_vertically_on_load(true);
    let result = image::load(path);
    if let LoadResult::Error(msg) = result {
      return Err(Error::LoadError(msg));
    }
    let texture = unsafe { GL.create_texture().unwrap() };
    unsafe {
      GL.bind_texture(glow::TEXTURE_2D, Some(texture));
      GL.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
      GL.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
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
    }
    match result {
      LoadResult::ImageF32(data) => unsafe {
        upload_texture_data(
          data.width,
          data.height,
          data.depth,
          any_as_u8_slice(data.data.as_slice()).to_vec(),
        );
      },
      LoadResult::ImageU8(data) => unsafe {
        upload_texture_data(data.width, data.height, data.depth, data.data);
      },
      _ => {
        panic!("不可达的代码")
      }
    }
    unsafe {
      GL.generate_mipmap(glow::TEXTURE_2D);
    }
    Ok(Texture { inner: texture })
  }
  pub fn from_res(res: &Resources, name: &str) -> Result<Texture, Error> {
    let mut full_path = res.get_root_path().clone();
    full_path.push(name);
    Self::new(full_path)
  }
  pub fn bind(&self) {
    unsafe {
      GL.bind_texture(glow::TEXTURE_2D, Some(self.inner));
    }
  }
  pub fn detach(&self) {
    unsafe {
      GL.bind_texture(glow::TEXTURE_2D, None);
    }
  }
}
impl Drop for Texture {
  fn drop(&mut self) {
    unsafe {
      GL.delete_texture(self.inner);
    }
  }
}

unsafe fn upload_texture_data(width: usize, height: usize, channels: usize, pixels: Vec<u8>) {
  match channels {
    3 => {
      GL.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        glow::RGB as i32,
        width as i32,
        height as i32,
        0,
        glow::RGB,
        glow::UNSIGNED_BYTE,
        Some(&pixels),
      );
    }
    4 => {
      GL.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        glow::RGBA as i32,
        width as i32,
        height as i32,
        0,
        glow::RGBA,
        glow::UNSIGNED_BYTE,
        Some(&pixels),
      );
    }
    _ => {
      panic!("不支持的图片通道数")
    }
  }
}
