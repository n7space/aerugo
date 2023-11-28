use aerugo::{logln, RuntimeApi};
pub use lsm6dso::config::control::AccelerometerScale;

#[derive(Default)]
pub struct TaskSetAccelerometerScaleContext {}

pub fn task_set_accelerometer_scale(
    scale: AccelerometerScale,
    _: &mut TaskSetAccelerometerScaleContext,
    _: &'static dyn RuntimeApi,
) {
    logln!("Set accelerometer scale: {:?}", scale);
}
