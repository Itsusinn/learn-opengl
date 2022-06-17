use std::{io, path::PathBuf};

use glow::HasContext;
use thiserror::Error;

use image::io::Reader as ImageReader;
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


    let img = ImageReader::open(path)?.decode().unwrap();

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

    unsafe {
      match img {
        image::DynamicImage::ImageRgb8(_)
        |image::DynamicImage::ImageRgb16(_)
        |image::DynamicImage::ImageRgb32F(_)  => {
          upload_texture_data(img.width(), img.height(), 3, img.as_bytes())
        },
        image::DynamicImage::ImageRgba8(_)
        |image::DynamicImage::ImageRgba16(_)
        |image::DynamicImage::ImageRgba32F(_) => {
          upload_texture_data(img.width(), img.height(), 4, img.as_bytes())
        },
        _ => unimplemented!(),
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

unsafe fn upload_texture_data(width: u32, height: u32, channels: i32, pixels: &[u8]) {
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
