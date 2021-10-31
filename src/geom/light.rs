use na::Vector3;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Light {
  pub is_on: bool,
  pub ambient: Vector3<f32>,
  pub diffuse: Vector3<f32>,
  pub specular: Vector3<f32>,
}
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct DirectLight {
  pub light: Light,

  pub direction: Vector3<f32>,
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct PointLight {
  pub light: Light,

  pub position: Vector3<f32>,
  pub constant: f32,
  pub linear: f32,
  pub quadratic: f32,
}
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct SpotLight {
  pub light: Light,

  pub position: Vector3<f32>,
  pub direction: Vector3<f32>,

  pub constant: f32,
  pub linear: f32,
  pub quadratic: f32,
  pub cut_off: f32,
  pub outer_cut_off: f32,
}
