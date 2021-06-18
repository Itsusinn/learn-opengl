use std::{io, path::PathBuf};

use stb_image::image::{self,Image, LoadResult};
use thiserror::Error;

use crate::resources::Resources;

#[derive(Debug,Error)]
pub enum Error {
   #[error("I/O 错误")]
   IO(#[from] io::Error),
   #[error("纹理加载错误 ,原因:{0}")]
   LoadError(String)
}
pub struct  Texture {
    id : u32,
    gl : gl::Gl
}
impl Texture {
    pub fn new(gl: &gl::Gl,path:PathBuf) -> Result<Texture,Error>{
        let result = image::load(path);
        if let LoadResult::Error(msg) = result {
            return Err(Error::LoadError(msg));
        }
        let mut id = 0u32;
        unsafe  {
            gl.GenTextures(1, &mut id);
            gl.BindTexture(gl::TEXTURE_2D, id);
        }
        match result {
            LoadResult::ImageF32(data) => {
                unsafe {
                    gl.TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGB as i32,
                        data.width as i32,data.height as i32,
                        0, gl::RGB, gl::UNSIGNED_BYTE,
                        data.data.as_ptr() as *const gl::types::GLvoid
                    );
                }
            }
            LoadResult::ImageU8(data) => {
                unsafe {
                    gl.TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGB as i32,
                        data.width as i32,data.height as i32,
                        0, gl::RGB, gl::UNSIGNED_BYTE,
                        data.data.as_ptr() as *const gl::types::GLvoid
                    );
                }
            },
            _=> { panic!("不可达的代码") }
        }
        unsafe  {
            gl.GenerateMipmap(gl::TEXTURE_2D);
        }
        Ok(Texture{
            id,
            gl: gl.clone()
        })
    }
    pub fn from_res(gl: &gl::Gl,res:&Resources,name:&str) -> Result<Texture,Error>{
        let mut  full_path= res.get_root_path().clone();
        full_path.push(name);
        Self::new(gl, full_path)
    }
    pub fn bind(&self){
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
       unsafe {
           self.gl.DeleteTextures(1,&mut self.id)
        }
    }
 }