use aerugo::{logln, EventId, RuntimeApi};
use lsm6dso::config::control::{AccelerometerConfig, GyroscopeConfig};

use crate::{task_set_data_output_rate::IMU_DATA_RATE_CONFIG, IMU_STORAGE};

#[derive(Default)]
pub struct TaskStartMeasurementsContext {}

pub fn task_start_measurements(
    _: EventId,
    _: &mut TaskStartMeasurementsContext,
    _: &'static dyn RuntimeApi,
) {
    // This is safe, because it's a single-core system and IMU_STORAGE is never accessed from any IRQ
    let imu = unsafe { IMU_STORAGE.as_mut().unwrap() };
    let data_rate_config = match IMU_DATA_RATE_CONFIG.lock(|config| config.borrow().clone()) {
        Some(config) => config,
        None => {
            logln!("Measurements cannot be started, data rate was not set yet");
            return;
        }
    };

    let accelerometer_config = AccelerometerConfig {
        data_rate: data_rate_config.accelerometer,
        ..imu.get_accelerometer_config().unwrap()
    };
    let gyroscope_config = GyroscopeConfig {
        data_rate: data_rate_config.gyroscope,
        ..imu.get_gyroscope_config().unwrap()
    };

    imu.set_accelerometer_config(accelerometer_config).unwrap();
    imu.set_gyroscope_config(gyroscope_config).unwrap();

    assert_eq!(
        accelerometer_config,
        imu.get_accelerometer_config().unwrap()
    );
    assert_eq!(gyroscope_config, imu.get_gyroscope_config().unwrap());

    logln!(
        "Measurements started with {:?} data rate",
        data_rate_config.accelerometer
    );
}
