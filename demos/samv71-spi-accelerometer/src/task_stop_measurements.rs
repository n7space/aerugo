use aerugo::{logln, EventId, RuntimeApi};
use lsm6dso::config::control::{
    AccelerometerConfig, AccelerometerDataRate, GyroscopeConfig, GyroscopeDataRate,
};

use crate::IMU_STORAGE;

#[derive(Default)]
pub struct TaskStopMeasurementsContext {}

pub fn task_stop_measurements(
    _: EventId,
    _: &mut TaskStopMeasurementsContext,
    _: &'static dyn RuntimeApi,
) {
    // This is safe, because it's a single-core system and IMU_STORAGE is never accessed from any IRQ
    let imu = unsafe { IMU_STORAGE.as_mut().unwrap() };

    let accelerometer_config = AccelerometerConfig {
        data_rate: AccelerometerDataRate::PowerDown,
        ..imu.get_accelerometer_config().unwrap()
    };
    let gyroscope_config = GyroscopeConfig {
        data_rate: GyroscopeDataRate::PowerDown,
        ..imu.get_gyroscope_config().unwrap()
    };

    imu.set_accelerometer_config(accelerometer_config).unwrap();
    imu.set_gyroscope_config(gyroscope_config).unwrap();

    assert_eq!(
        accelerometer_config,
        imu.get_accelerometer_config().unwrap()
    );
    assert_eq!(gyroscope_config, imu.get_gyroscope_config().unwrap());

    logln!("Measurements stopped");
}
