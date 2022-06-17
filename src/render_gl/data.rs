use glow::HasContext;
use na::Vector3;

use crate::GL;
extern crate vec_2_10_10_10;

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

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev_float {
  pub inner: vec_2_10_10_10::Vector,
}
impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev_float {
  fn from(other: (f32, f32, f32, f32)) -> Self {
    u2_u10_u10_u10_rev_float {
      inner: vec_2_10_10_10::Vector::new(other.0, other.1, other.2, other.3),
    }
  }
}
impl u2_u10_u10_u10_rev_float {
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

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct int8 {
  pub d0: i8,
}
impl int8 {
  pub fn new(d0: i8) -> int8 {
    int8 { d0 }
  }

  pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
    GL.enable_vertex_attrib_array(location as u32);
    GL.vertex_attrib_pointer_i32(location as u32, 1, glow::BYTE, stride as i32, offset as i32);
  }
}
impl From<i8> for int8 {
  fn from(other: i8) -> Self {
    int8::new(other)
  }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct int8_float {
  pub d0: i8,
}
impl int8_float {
  pub fn new(d0: i8) -> int8_float {
    int8_float { d0 }
  }
  // attribute
  pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
    GL.enable_vertex_attrib_array(location as u32);
    GL.vertex_attrib_pointer_f32(
      location as u32,
      1,
      glow::BYTE,
      true,
      stride as i32,
      offset as i32,
    );
  }
}
impl From<i8> for int8_float {
  fn from(other: i8) -> Self {
    int8_float::new(other)
  }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32 {
  pub d0: f32,
  pub d1: f32,
}
impl f32_f32 {
  pub fn new(d0: f32, d1: f32) -> f32_f32 {
    f32_f32 { d0, d1 }
  }

  pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
    GL.enable_vertex_attrib_array(location as u32);
    GL.vertex_attrib_pointer_f32(
      location as u32,
      2,
      glow::FLOAT,
      false,
      stride as i32,
      offset as i32,
    );
  }
}
impl From<(f32, f32)> for f32_f32 {
  fn from(other: (f32, f32)) -> Self {
    f32_f32::new(other.0, other.1)
  }
}
