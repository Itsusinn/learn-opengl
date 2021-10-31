use crate::time;
use na::{Matrix4, Point3, Unit, Vector3};
use sdl2::keyboard::Keycode;

use super::input;

lazy_static! {
  static ref Y_AXIS: Unit<Vector3<f32>> = Vector3::y_axis();
}

pub struct Camera {
  // 摄像机的位置
  eye: Point3<f32>,
  // 摄像机的看向
  toward: Vector3<f32>,
  // 摄像机朝上的方向
  up: Vector3<f32>,
  // 俯仰角
  pitch: f32,
  // 偏航角
  yaw: f32,
  // 近平面距离
  znear: f32,
  // 远平面距离
  zfar: f32,
  // 视域(角度)
  fov: f32,
  enable: bool,
}
impl Camera {
  pub fn new(eye: Point3<f32>) -> Self {
    // 格拉姆—施密特正交化(Gram-Schmidt Process)。 <https://en.wikipedia.org/wiki/Gram-Schmidt_process>
    let toward = Vector3::new(0.0, 0.0, -1.0);
    let right = Y_AXIS.cross(&toward);
    let up = toward.cross(&right).normalize();
    Camera {
      eye,
      toward,
      up,
      pitch: 0.0,
      yaw: -90.0,
      znear: 1.0,
      zfar: 10000.0,
      fov: 3.14 / 4.0,
      enable: false,
    }
  }
  pub fn move_forward_and_backward(&mut self, distance: f32) {
    let change = Vector3::new(self.toward.x, 0.0, self.toward.z).normalize();
    let delta = change * distance;
    self.eye += delta
  }
  pub fn move_left_and_right(&mut self, distance: f32) {
    let right = Y_AXIS.cross(&self.toward).normalize();
    let delta = right * distance;
    self.eye += delta;
  }
  pub fn move_upward_and_downward(&mut self, distance: f32) {
    self.eye.y += distance;
  }
  pub fn turn_right_and_left(&mut self, angle: f32) {
    self.yaw += angle;
    self.toward.x = self.pitch.to_radians().cos() * self.yaw.to_radians().cos();
    self.toward.z = self.pitch.to_radians().cos() * self.yaw.to_radians().sin();
    self.gs_process();
  }
  pub fn turn_up_and_down(&mut self, angle: f32) {
    self.pitch += angle;
    if self.pitch >= 89.0 {
      self.pitch = 89.0
    } else if self.pitch <= -89.0 {
      self.pitch = -89.0
    }
    self.toward.x = self.pitch.to_radians().cos() * self.yaw.to_radians().cos();
    self.toward.y = self.pitch.to_radians().sin();
    self.toward.z = self.pitch.to_radians().cos() * self.yaw.to_radians().sin();
    self.gs_process()
  }
  // Gram-Schmidt Process
  fn gs_process(&mut self) {
    let right = Y_AXIS.cross(&self.toward);
    self.up = self.toward.cross(&right).normalize();
  }
  pub fn handle_sdl_input(&mut self) {
    let (dx, dy) = input::fetch_motion();
    if !self.enable {
      return;
    }
    let rate = time::get_delta() * 10.0;
    if dx != 0 {
      self.turn_right_and_left((dx as f32) / 10.0);
    }
    if dy != 0 {
      self.turn_up_and_down((dy as f32) / 10.0);
    }
    if input::get_key(Keycode::W, false) {
      self.move_forward_and_backward(rate);
    }
    if input::get_key(Keycode::S, false) {
      self.move_forward_and_backward(-rate);
    }
    if input::get_key(Keycode::A, false) {
      self.move_left_and_right(rate);
    }
    if input::get_key(Keycode::D, false) {
      self.move_left_and_right(-rate);
    }
    if input::get_key(Keycode::Space, false) {
      self.move_upward_and_downward(rate);
    }
    if input::get_key(Keycode::LShift, false) {
      self.move_upward_and_downward(-rate);
    }
  }
  pub fn get_pv_mat(&self) -> Matrix4<f32> {
    let proj_mat = Matrix4::new_perspective(16.0 / 9.0, self.fov, self.znear, self.zfar);
    let view_mat = Matrix4::look_at_rh(&self.eye, &(&self.eye + &self.toward), &self.up);
    proj_mat * view_mat
  }
  pub fn disable(&mut self) {
    self.enable = false;
  }
  pub fn enable(&mut self) {
    self.enable = true;
  }
}
