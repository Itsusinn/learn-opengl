use crate::GL;
use glow::HasContext;
use nalgebra::Vector3;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
  pub d0: f32,
  pub d1: f32,
  pub d2: f32,
}
impl f32_f32_f32 {
  pub fn new(d0: f32, d1: f32, d2: f32) -> f32_f32_f32 {
    f32_f32_f32 { d0, d1, d2 }
  }

  pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
    GL.enable_vertex_attrib_array(location as u32);
    GL.vertex_attrib_pointer_f32(
      location as u32,
      3,
      glow::FLOAT,
      false,
      stride as i32,
      offset as i32,
    );
  }
}
impl From<(f32, f32, f32)> for f32_f32_f32 {
  fn from(other: (f32, f32, f32)) -> Self {
    f32_f32_f32::new(other.0, other.1, other.2)
  }
}

impl Into<Vector3<f32>> for f32_f32_f32 {
  fn into(self) -> Vector3<f32> {
    Vector3::new(self.d0, self.d1, self.d2)
  }
}
