use crate::GL;
use glow::HasContext;

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
