use std::any::Any;

use arcstr::ArcStr;

use crate::geom::camera::Camera;
pub trait Scene {
  fn render(&self, aspect: f32) -> Option<()>;
  fn get_camera(&mut self) -> &mut Camera;
  fn get_name(&self) -> ArcStr;
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}
