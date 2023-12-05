use core::cell::RefCell;

use aerugo::{logln, Mutex, RuntimeApi};
use lsm6dso::config::{
    control::{AccelerometerDataRate, GyroscopeDataRate},
    fifo::config::{AccelerometerBatchingRate, FifoConfig, GyroscopeBatchingRate},
};

use crate::{telemetry::Telemetry, IMU_STORAGE, UART_WRITER_STORAGE};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutputDataRate {
    Odr12_5Hz,
    Odr26Hz,
    Odr52Hz,
    Odr104Hz,
    Odr208Hz,
    Odr416Hz,
    Odr833Hz,
    Odr1667Hz,
    Odr3333Hz,
    Odr6667Hz,
}

impl From<OutputDataRate> for AccelerometerDataRate {
    fn from(value: OutputDataRate) -> Self {
        match value {
            OutputDataRate::Odr12_5Hz => Self::Rate12_5Hz,
            OutputDataRate::Odr26Hz => Self::Rate26Hz,
            OutputDataRate::Odr52Hz => Self::Rate52Hz,
            OutputDataRate::Odr104Hz => Self::Rate104Hz,
            OutputDataRate::Odr208Hz => Self::Rate208Hz,
            OutputDataRate::Odr416Hz => Self::Rate416Hz,
            OutputDataRate::Odr833Hz => Self::Rate833Hz,
            OutputDataRate::Odr1667Hz => Self::Rate1667Hz,
            OutputDataRate::Odr3333Hz => Self::Rate3333Hz,
            OutputDataRate::Odr6667Hz => Self::Rate6667Hz,
        }
    }
}

impl From<OutputDataRate> for GyroscopeDataRate {
    fn from(value: OutputDataRate) -> Self {
        match value {
            OutputDataRate::Odr12_5Hz => Self::Rate12_5Hz,
            OutputDataRate::Odr26Hz => Self::Rate26Hz,
            OutputDataRate::Odr52Hz => Self::Rate52Hz,
            OutputDataRate::Odr104Hz => Self::Rate104Hz,
            OutputDataRate::Odr208Hz => Self::Rate208Hz,
            OutputDataRate::Odr416Hz => Self::Rate416Hz,
            OutputDataRate::Odr833Hz => Self::Rate833Hz,
            OutputDataRate::Odr1667Hz => Self::Rate1667Hz,
            OutputDataRate::Odr3333Hz => Self::Rate3333Hz,
            OutputDataRate::Odr6667Hz => Self::Rate6667Hz,
        }
    }
}

impl From<OutputDataRate> for AccelerometerBatchingRate {
    fn from(value: OutputDataRate) -> Self {
        match value {
            OutputDataRate::Odr12_5Hz => Self::Batch12_5Hz,
            OutputDataRate::Odr26Hz => Self::Batch26Hz,
            OutputDataRate::Odr52Hz => Self::Batch52Hz,
            OutputDataRate::Odr104Hz => Self::Batch104Hz,
            OutputDataRate::Odr208Hz => Self::Batch208Hz,
            OutputDataRate::Odr416Hz => Self::Batch416Hz,
            OutputDataRate::Odr833Hz => Self::Batch833Hz,
            OutputDataRate::Odr1667Hz => Self::Batch1667Hz,
            OutputDataRate::Odr3333Hz => Self::Batch3333Hz,
            OutputDataRate::Odr6667Hz => Self::Batch6667Hz,
        }
    }
}

impl From<OutputDataRate> for GyroscopeBatchingRate {
    fn from(value: OutputDataRate) -> Self {
        match value {
            OutputDataRate::Odr12_5Hz => Self::Batch12_5Hz,
            OutputDataRate::Odr26Hz => Self::Batch26Hz,
            OutputDataRate::Odr52Hz => Self::Batch52Hz,
            OutputDataRate::Odr104Hz => Self::Batch104Hz,
            OutputDataRate::Odr208Hz => Self::Batch208Hz,
            OutputDataRate::Odr416Hz => Self::Batch416Hz,
            OutputDataRate::Odr833Hz => Self::Batch833Hz,
            OutputDataRate::Odr1667Hz => Self::Batch1667Hz,
            OutputDataRate::Odr3333Hz => Self::Batch3333Hz,
            OutputDataRate::Odr6667Hz => Self::Batch6667Hz,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImuDataRateConfiguration {
    pub accelerometer: AccelerometerDataRate,
    pub gyroscope: GyroscopeDataRate,
}

/// This storage is used to keep IMU data rate configuration created by task_set_data_output_rate
/// tasklet.
///
/// Mutex is used here as an example. Similarly to [`IMU_STORAGE`], this could simply be a
/// `static mut`, as it's never accessed from other threads. However, due to the fact that this
/// will usually be accessed either to replace or clone the value, it's fine to use short critical
/// section to guard access to this.
pub static IMU_DATA_RATE_CONFIG: Mutex<RefCell<Option<ImuDataRateConfiguration>>> =
    Mutex::new(RefCell::new(None));

#[derive(Default)]
pub struct TaskSetDataOutputRateContext {}

pub fn task_set_data_output_rate(
    output_rate: OutputDataRate,
    _: &mut TaskSetDataOutputRateContext,
    _: &'static dyn RuntimeApi,
) {
    // This is safe, because it's a single-core system and IMU_STORAGE is never accessed from any IRQ
    let imu = unsafe { IMU_STORAGE.as_mut().unwrap() };

    // Read old config, update it, verify if it's updated successfully.
    let fifo_config = FifoConfig {
        gyroscope_batching_rate: output_rate.into(),
        accelerometer_batching_rate: output_rate.into(),
        ..imu.get_fifo_config().unwrap()
    };
    imu.set_fifo_config(fifo_config).unwrap();
    assert_eq!(fifo_config, imu.get_fifo_config().unwrap());

    // Gyroscope and accelerometer data rates must be stored in some global memory, as they are used
    // to start and stop measurements.
    IMU_DATA_RATE_CONFIG.lock(|config| {
        config.borrow_mut().replace(ImuDataRateConfiguration {
            accelerometer: output_rate.into(),
            gyroscope: output_rate.into(),
        })
    });

    Telemetry::new_set_data_output_rate_confirmation()
        .write_ccsds_packet(unsafe { UART_WRITER_STORAGE.as_mut().unwrap() });

    logln!("Data output rate set to {:?}", output_rate)
}
