use std::sync::RwLock;
use std::time::Instant;

use once_cell::sync::Lazy;

static PREVIOUS_TIME: Lazy<RwLock<f32>> = Lazy::new(||{ RwLock::new(0.0) });
static DELTA_TIME: Lazy<RwLock<f32>> = Lazy::new(||{  RwLock::new(0.0) });
static START: Lazy<Instant> = Lazy::new(||{ Instant::now()});

pub fn update() {
  let now = START.elapsed().as_secs_f32();
  let mut pervious = PREVIOUS_TIME.write().unwrap();
  let mut delta = DELTA_TIME.write().unwrap();
  *delta = now - *pervious;
  *pervious = now;
}
pub fn get_delta() -> f32 {
  *DELTA_TIME.read().unwrap()
}
pub fn get_now() -> f32 {
  START.elapsed().as_secs_f32()
}
