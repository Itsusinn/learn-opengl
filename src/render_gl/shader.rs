use gl::types::*;
use std::ffi::{CStr, CString};
use crate::resources::Resources;
use crate::resources;
use thiserror::Error;
use na::{Matrix4, Vector2, Vector3, Vector4};

#[derive(Debug,Error)]
pub enum Error {
   #[error("资源加载失败 {}", name)]
   ResourceLoad { name: String,#[source] inner: resources::Error },
   #[error("Can not determine shader type for resource {}", name)]
   CanNotDetermineShaderTypeForResource { name: String },
   #[error( "着色器编译失败: {}", message)]
   CompileError { message: String },
   #[error("着色程序链接失败 : {}", message)]
   LinkError { message: String },
}

pub struct Program {
   gl: gl::Gl,
   id: GLuint,
}
impl Program {
   pub fn from_res(
      gl:&gl::Gl,
      res:&Resources,
      name:&str
   ) -> Result<Program, Error> {
      const POSSIBLE_EXT: [&str; 2] = [".vert",".frag"];
      let shaders = POSSIBLE_EXT.iter()
         .map(|file_extension| {
            Shader::from_res(
               gl,res,
               &format!("{}{}", name, file_extension)
            )
         })
         .collect::<Result<Vec<Shader>, Error>>()?;
      Ok(Program::from_shaders(gl,&shaders[..])?)
   }

   pub fn upload_texture_slot(&self, name: &str, slot: i32) {
      self.set_used();
      let name = CString::new(name).unwrap();
      unsafe {
         let location = self.gl.GetUniformLocation(self.id, name.as_ptr());
         self.gl.Uniform1i(location, slot);
      }
  }
   pub fn upload_mat4(&self,name: &str, mat4: &Matrix4<f32>) {
      self.set_used();
      let name = CString::new(name).unwrap();
      unsafe {
         let location = self.gl.GetUniformLocation(self.id, name.as_ptr());
         self.gl.UniformMatrix4fv(location,1, gl::FALSE, mat4.as_ptr());
      }
   }
   pub fn upload_vec2(&self, name: &str, vec2: &Vector2<f32>) {
      self.set_used();
      let name = CString::new(name).unwrap();
      unsafe {
         let location = self.gl.GetUniformLocation(self.id, name.as_ptr());
         self.gl.Uniform2fv(location, 1, vec2.as_ptr());
      }
  }

   pub fn upload_vec3(&self, name: &str, vec3: &Vector3<f32>) {
      self.set_used();
      let name = CString::new(name).unwrap();
      unsafe {
         let location = self.gl.GetUniformLocation(self.id, name.as_ptr());
         self.gl.Uniform3fv(location, 1, vec3.as_ptr());
      }
  }

   pub fn upload_vec4(&self, name: &str, vec4: &Vector4<f32>) {
      self.set_used();
      let name = CString::new(name).unwrap();
      unsafe  {
         let location = self.gl.GetUniformLocation(self.id, name.as_ptr());
         self.gl.Uniform4fv(location, 1, vec4.as_ptr());
      }
  }

   pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, Error> {
      let program_id = unsafe { gl.CreateProgram() };
      for shader in shaders {
         unsafe { gl.AttachShader(program_id, shader.id) };
      }
      unsafe { gl.LinkProgram(program_id) }
      for shader in shaders {
         unsafe { gl.DetachShader(program_id, shader.id) };
      }
      let mut success: gl::types::GLint = 1;
      unsafe {
         gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
      }

      if success == 0 {
         let mut len: gl::types::GLint = 0;
         unsafe {
            gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
         }

         let error = create_whitespace_cstring_with_len(len as usize);

         unsafe {
            gl.GetProgramInfoLog(
               program_id,
               len,
               std::ptr::null_mut(),
               error.as_ptr() as *mut gl::types::GLchar,
            );
         }

         return Err(
            Error::LinkError { message:error.to_string_lossy().into_owned() }
         );
      }
      Ok(Program {
         gl: gl.clone(),
         id: program_id,
      })
   }
   pub fn id(&self) -> GLuint {
      self.id
   }
   pub fn set_used(&self) {
      unsafe { self.gl.UseProgram(self.id) }
   }
   pub fn detach(&self){
      unsafe { self.gl.UseProgram(0) }
   }
}

impl Drop for Program {
   fn drop(&mut self) {
      unsafe { self.gl.DeleteProgram(self.id) }
   }
}

pub struct Shader {
   gl: gl::Gl,
   id: gl::types::GLuint,
}

impl Shader {
   pub fn from_res(
      gl:&gl::Gl,
      res:&Resources,
      name:&str
   ) -> Result<Shader, Error> {
      const POSSIBLE_EXT:[(&str,GLenum);2] = [
         (".vert",gl::VERTEX_SHADER),
         (".frag",gl::FRAGMENT_SHADER)
      ];

      let shader_kind = POSSIBLE_EXT.iter()
         .find(|&&(file_extension,_)| {
            name.ends_with(file_extension)
         })
         .map(|&(_,kind) | kind)
         .ok_or_else( || Error::CanNotDetermineShaderTypeForResource {
            name: format!("无法判断给定resource的渲染器类型 {}", name)
         })?;

      let source = res.load_cstring(name)
         .map_err( |e| Error::ResourceLoad {
            name: format!("Resources {:?} {}",&res.get_root_path(),name),
            inner: e
         })?;

      Shader::from_source(gl,&source,shader_kind)
   }

   pub fn id(&self) -> GLuint {
      self.id
   }

   pub fn from_source(
      gl: &gl::Gl,
      source: &CStr,
      kind: gl::types::GLenum,
   ) -> Result<Shader, Error> {
      let id = shader_from_source(&gl, source, kind)?;
      Ok(Shader { gl: gl.clone(), id })
   }

   pub fn from_vert_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, Error> {
      Shader::from_source(gl, source, gl::VERTEX_SHADER)
   }

   pub fn from_frag_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, Error> {
      Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
   }
}

impl Drop for Shader {
   fn drop(&mut self) {
      unsafe { self.gl.DeleteShader(self.id) }
   }
}

fn shader_from_source(
   gl: &gl::Gl,
   source: &CStr,
   kind: gl::types::GLenum,
) -> Result<gl::types::GLuint, Error> {
   let id = unsafe { gl.CreateShader(kind) };

   unsafe {
      gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
      gl.CompileShader(id);
   }

   let mut success: gl::types::GLint = 1;
   unsafe {
      gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
   }

   if success == 0 {
      let mut len: gl::types::GLint = 0;
      unsafe {
         gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
      }

      let error = create_whitespace_cstring_with_len(len as usize);

      unsafe {
         gl.GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar, );
      }
      return Err(
         Error::CompileError { message:error.to_string_lossy().into_owned() }
      );
   }
   Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
   // allocate buffer of correct size
   let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
   // fill it with len spaces
   buffer.extend([b' '].iter().cycle().take(len));
   // convert buffer to CString
   unsafe { CString::from_vec_unchecked(buffer) }
}
