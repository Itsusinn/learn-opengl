use super::clamp;
use crate::GL;
use glow::HasContext;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev_float {
  data: u32,
}
impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev_float {
  fn from(other: (f32, f32, f32, f32)) -> Self {
    Self::new(other.0, other.1, other.2, other.3)
  }
}
impl u2_u10_u10_u10_rev_float {
  pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    let x = (clamp(x) * 1023f32).round() as u32;
    let y = (clamp(y) * 1023f32).round() as u32;
    let z = (clamp(z) * 1023f32).round() as u32;
    let w = (clamp(w) * 3f32).round() as u32;

    let mut c: u32 = 0;
    c |= w << 30;
    c |= z << 20;
    c |= y << 10;
    c |= x << 0;

    Self { data: c }
  }
  pub fn from_raw(data: u32) -> Self {
    Self { data }
  }

  pub fn x(&self) -> f32 {
    (1023 & self.data) as f32 / 1023f32
  }

  pub fn y(&self) -> f32 {
    ((1023 << 10 & self.data) >> 10) as f32 / 1023f32
  }

  pub fn z(&self) -> f32 {
    ((1023 << 20 & self.data) >> 20) as f32 / 1023f32
  }

  pub fn w(&self) -> f32 {
    ((0b11 << 30 & self.data) >> 30) as f32 / 3f32
  }

  pub fn set_x(&mut self, x: f32) {
    let x = (clamp(x) * 1023f32).round() as u32;
    let mut c: u32 = (3 << 30 | 1023 << 20 | 1023 << 10) & self.data;
    c |= x;
    self.data = c;
  }

  pub fn set_y(&mut self, y: f32) {
    let y = (clamp(y) * 1023f32).round() as u32;
    let mut c: u32 = (3 << 30 | 1023 << 20 | 1023) & self.data;
    c |= y << 10;
    self.data = c;
  }

  pub fn set_z(&mut self, z: f32) {
    let z = (clamp(z) * 1023f32).round() as u32;
    let mut c: u32 = (3 << 30 | 1023 << 10 | 1023) & self.data;
    c |= z << 20;
    self.data = c;
  }

  pub fn set_xyz(&mut self, x: f32, y: f32, z: f32) {
    let x = (clamp(x) * 1023f32).round() as u32;
    let y = (clamp(y) * 1023f32).round() as u32;
    let z = (clamp(z) * 1023f32).round() as u32;
    let mut c: u32 = (3 << 30) & self.data;
    c |= z << 20;
    c |= y << 10;
    c |= x << 0;
    self.data = c;
  }

  pub fn set_w(&mut self, w: f32) {
    let w = (clamp(w) * 3f32).round() as u32;
    let mut c: u32 = (1023 << 20 | 1023 << 10 | 1023) & self.data;
    c |= w << 30;
    self.data = c;
  }

  #[allow(unaligned_references)]
  pub fn raw_value(&self) -> &u32 {
    &self.data
  }
  pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
    GL.enable_vertex_attrib_array(location as u32);
    GL.vertex_attrib_pointer_f32(
      location as u32,
      4,
      glow::UNSIGNED_INT_2_10_10_10_REV,
      true,
      stride as i32,
      offset as i32,
    );
  }
}
