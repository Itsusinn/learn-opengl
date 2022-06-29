use crate::GL;
use glow::HasContext;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_float {
  pub d0: i8,
}
impl i8_float {
  pub fn new(d0: i8) -> i8_float {
    i8_float { d0 }
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
impl From<i8> for i8_float {
  fn from(other: i8) -> Self {
    i8_float::new(other)
  }
}
