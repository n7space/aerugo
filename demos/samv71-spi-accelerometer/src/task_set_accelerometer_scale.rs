use aerugo::{logln, RuntimeApi};
use lsm6dso::config::control::AccelerometerConfig;
pub use lsm6dso::config::control::AccelerometerScale;

use crate::{telemetry::Telemetry, IMU_STORAGE, UART_WRITER_STORAGE};

#[derive(Default)]
pub struct TaskSetAccelerometerScaleContext {}

pub fn task_set_accelerometer_scale(
    scale: AccelerometerScale,
    _: &mut TaskSetAccelerometerScaleContext,
    _: &'static dyn RuntimeApi,
) {
    // This is safe, because it's a single-core system and IMU_STORAGE is never accessed from any IRQ
    let imu = unsafe { IMU_STORAGE.as_mut().unwrap() };

    // Read old config, update it, verify if it's updated successfully.
    let accelerometer_config = AccelerometerConfig {
        scale,
        ..imu.get_accelerometer_config().unwrap()
    };
    imu.set_accelerometer_config(accelerometer_config).unwrap();
    assert_eq!(
        accelerometer_config,
        imu.get_accelerometer_config().unwrap()
    );

    Telemetry::new_set_accelerometer_scale_confirmation()
        .write_ccsds_packet(unsafe { UART_WRITER_STORAGE.as_mut().unwrap() });

    logln!("Accelerometer scale set to {:?}", scale);
}
