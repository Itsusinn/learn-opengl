pub(crate) mod f32_f32;
pub(crate) mod f32_f32_f32;
pub(crate) mod i8_float;
pub(crate) mod int8;
pub(crate) mod u2_u10_u10_u10_rev_float;

#[inline]
fn clamp(c: f32) -> f32 {
  if c < 0.0 {
    return 0.0;
  }
  if c > 1.0 {
    return 1.0;
  }
  c
}
