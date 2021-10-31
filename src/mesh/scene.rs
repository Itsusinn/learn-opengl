use gl;

use crate::geom::camera::Camera;
pub trait Scene {
  fn render(&self, gl: &gl::Gl) -> Option<()>;
  fn get_camera(&mut self) -> &mut Camera;
}
