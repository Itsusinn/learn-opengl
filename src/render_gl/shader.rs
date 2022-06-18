use crate::resources;
use crate::resources::Resources;
use crate::GL;
use glow::HasContext;
use na::{Matrix3, Matrix4, Vector2, Vector3, Vector4, Point3};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("资源加载失败 {}", name)]
  ResourceLoad {
    name: String,
    #[source]
    inner: resources::Error,
  },
  #[error("Can not determine shader type for resource {}", name)]
  CanNotDetermineShaderTypeForResource { name: String },
  #[error("着色器编译失败: {}", message)]
  CompileError { message: String },
  #[error("着色程序链接失败 : {}", message)]
  LinkError { message: String },
}

pub struct Program {
  inner: glow::Program,
}
impl Program {
  pub fn from_res(res: &Resources, name: &str) -> Result<Program, Error> {
    const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];
    let shaders = POSSIBLE_EXT
      .iter()
      .map(|file_extension| Shader::from_res(res, &format!("{}{}", name, file_extension)))
      .collect::<Result<Vec<Shader>, Error>>()?;
    Ok(Program::from_shaders(&shaders[..])?)
  }

  pub fn upload_texture_slot(&self, name: &str, slot: i32)  -> Option<()> {
    self.set_used();
    unsafe {
      let location = GL.get_uniform_location(self.inner, name)?;
      GL.uniform_1_i32(Some(&location), slot);
      Some(())
    }
  }
  pub fn upload_mat4(&self, name: &str, mat4: &Matrix4<f32>) -> Option<()>  {
    self.set_used();
    unsafe {
      let location = GL.get_uniform_location(self.inner, name)?;
      GL.uniform_matrix_4_f32_slice(Some(&location), false, mat4.as_slice());
      Some(())
    }
  }
  pub fn upload_mat3(&self, name: &str, mat3: &Matrix3<f32>) -> Option<()> {
    self.set_used();
    unsafe {
      let location = GL.get_uniform_location(self.inner, name)?;
      GL.uniform_matrix_3_f32_slice(Some(&location), false, mat3.as_slice());
      Some(())
    }
  }
  pub fn upload_vec2(&self, name: &str, vec2: &Vector2<f32>)  -> Option<()> {
    self.set_used();
    unsafe {
      let location = GL.get_uniform_location(self.inner, name)?;
      GL.uniform_2_f32(Some(&location), vec2.x, vec2.y);
      Some(())
    }
  }

  pub fn upload_vec3(&self, name: &str, vec3: &Vector3<f32>) -> Option<()>  {
    self.set_used();
    unsafe {
      let location = GL.get_uniform_location(self.inner, name)?;
      GL.uniform_3_f32(Some(&location), vec3.x, vec3.y, vec3.z);
      Some(())
    }
  }
  pub fn upload_point3(&self, name: &str, point3: &Point3<f32>) -> Option<()>  {
    self.upload_vec3(name, &point3.coords)
  }
  pub fn upload_vec4(&self, name: &str, vec4: &Vector4<f32>) -> Option<()>  {
    self.set_used();
    unsafe {
      let location = GL.get_uniform_location(self.inner, name)?;
      GL.uniform_4_f32(Some(&location), vec4.x, vec4.y, vec4.z, vec4.w);
      Some(())
    }
  }

  pub fn from_shaders(shaders: &[Shader]) -> Result<Program, Error> {
    let program = unsafe { GL.create_program().unwrap() };
    for shader in shaders {
      unsafe { GL.attach_shader(program, shader.inner) };
    }
    unsafe {
      GL.link_program(program);
    }
    for shader in shaders {
      unsafe {
        GL.detach_shader(program, shader.inner);
      }
    }

    unsafe {
      if !GL.get_program_link_status(program) {
        let info = GL.get_program_info_log(program);
        return Err(Error::LinkError { message: info });
      }
    }

    Ok(Program { inner: program })
  }
  pub fn set_used(&self) {
    unsafe {
      GL.use_program(Some(self.inner));
    }
  }
  pub fn detach(&self) {
    unsafe {
      GL.use_program(None);
    }
  }
}

impl Drop for Program {
  fn drop(&mut self) {
    unsafe {
      GL.delete_program(self.inner);
    }
  }
}

pub struct Shader {
  inner: glow::Shader,
}

impl Shader {
  pub fn from_res(res: &Resources, name: &str) -> Result<Shader, Error> {
    const POSSIBLE_EXT: [(&str, u32); 2] = [
      (".vert", glow::VERTEX_SHADER),
      (".frag", glow::FRAGMENT_SHADER),
    ];

    let shader_kind = POSSIBLE_EXT
      .iter()
      .find(|&&(file_extension, _)| name.ends_with(file_extension))
      .map(|&(_, kind)| kind)
      .ok_or_else(|| Error::CanNotDetermineShaderTypeForResource {
        name: format!("无法判断给定resource的渲染器类型 {}", name),
      })?;

    let source = res.load_string(name).map_err(|e| Error::ResourceLoad {
      name: format!("Resources {:?} {}", &res.get_root_path(), name),
      inner: e,
    })?;

    Shader::from_source(&source, shader_kind,name)
  }

  pub fn from_source(source: &str, kind: u32,name: &str) -> Result<Shader, Error> {
    let inner = shader_from_source(source, kind,name)?;
    Ok(Shader { inner })
  }

  pub fn from_vert_source(source: &str,name: &str) -> Result<Shader, Error> {
    Shader::from_source(source, glow::VERTEX_SHADER,name)
  }

  pub fn from_frag_source(source: &str,name: &str) -> Result<Shader, Error> {
    Shader::from_source(source, glow::FRAGMENT_SHADER,name)
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    unsafe { GL.delete_shader(self.inner) }
  }
}

fn shader_from_source(source: &str, shader_type: u32,name:&str) -> Result<glow::Shader, Error> {
  let shader = unsafe { GL.create_shader(shader_type).unwrap() };

  unsafe {
    GL.shader_source(shader, source);
    GL.compile_shader(shader);
  };

  let success = unsafe { GL.get_shader_compile_status(shader) };

  if !success {
    let info = unsafe { GL.get_shader_info_log(shader) };
    return Err(Error::CompileError { message: format!("{}{}",name,info) });
  }
  Ok(shader)
}
