use arcstr::ArcStr;
use gl;

use crate::geom::camera::Camera;
pub trait Scene {
  fn render(&self, gl: &gl::Gl, fov:f32) -> Option<()>;
  fn get_camera(&mut self) -> &mut Camera;
  fn get_name(&self) -> ArcStr;
}
