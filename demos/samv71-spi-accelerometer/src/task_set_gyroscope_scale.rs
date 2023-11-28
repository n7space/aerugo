use aerugo::{logln, RuntimeApi};
pub use lsm6dso::config::control::GyroscopeScale;

#[derive(Default)]
pub struct TaskSetGyroscopeScaleContext {}

pub fn task_set_gyroscope_scale(
    scale: GyroscopeScale,
    _: &mut TaskSetGyroscopeScaleContext,
    _: &'static dyn RuntimeApi,
) {
    logln!("Set gyroscope scale: {:?}", scale);
}
