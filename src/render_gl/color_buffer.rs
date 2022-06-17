use glow::HasContext;

use crate::GL;

pub struct ColorBuffer {
  pub color: na::Vector4<f32>,
}

impl ColorBuffer {
  ///  构造方法
  pub fn from_color(color: na::Vector3<f32>) -> ColorBuffer {
    ColorBuffer {
      color: color.fixed_resize::<4, 1>(1.0f32),
    }
  }

  pub fn update_color(&mut self, color: na::Vector3<f32>) {
    self.color = color.fixed_resize::<4, 1>(1.0f32);
  }

  pub fn clear(&self) {
    let color = self.color;
    unsafe {
      GL.clear_color(color.x, color.y, color.z, color.w);
    }
  }
}
