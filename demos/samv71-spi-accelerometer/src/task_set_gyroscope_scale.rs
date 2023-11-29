use aerugo::{logln, RuntimeApi};
use lsm6dso::config::control::GyroscopeConfig;
pub use lsm6dso::config::control::GyroscopeScale;

use crate::IMU_STORAGE;

#[derive(Default)]
pub struct TaskSetGyroscopeScaleContext {}

pub fn task_set_gyroscope_scale(
    scale: GyroscopeScale,
    _: &mut TaskSetGyroscopeScaleContext,
    _: &'static dyn RuntimeApi,
) {
    // This is safe, because it's a single-core system and IMU_STORAGE is never accessed from any IRQ
    let imu = unsafe { IMU_STORAGE.as_mut().unwrap() };

    // Read old config, update it, verify if it's updated successfully.
    let gyroscope_config = GyroscopeConfig {
        scale,
        ..imu.get_gyroscope_config().unwrap()
    };
    imu.set_gyroscope_config(gyroscope_config).unwrap();
    assert_eq!(gyroscope_config, imu.get_gyroscope_config().unwrap());

    logln!("Gyroscope scale set to {:?}", scale);
}
