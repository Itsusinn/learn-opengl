use crate::geom::camera::Camera;
pub trait Scene {
  fn render(&self, aspect: f32) -> Option<()>;
  fn get_camera(&mut self) -> &mut Camera;
  fn get_name(&self) -> arcstr::ArcStr;
  fn render_window(&mut self, _: &egui::CtxRef) {}
}
