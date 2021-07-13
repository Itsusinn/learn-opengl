use std::time::Instant;
use std::sync::RwLock;

lazy_static!{
    static ref PREVIOUS_TIME:RwLock<f32> = RwLock::new(0.0);
    static ref DELTA_TIME:RwLock<f32> = RwLock::new(0.0);
    static ref START:Instant = Instant::now();
}

pub fn update(){
    let now = START.elapsed().as_secs_f32();
    let mut pervious = PREVIOUS_TIME.write().unwrap();
    let mut delta = DELTA_TIME.write().unwrap();
    *delta = now - *pervious;
    *pervious = now;
}
pub fn get_delta() -> f32{
    *DELTA_TIME.read().unwrap()
}
pub fn get_now() -> f32 {
    START.elapsed().as_secs_f32()
}