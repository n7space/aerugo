use aerugo::RuntimeApi;
use lsm6dso::config::fifo::data::FifoWord;

use crate::{telemetry::Telemetry, IMU_STORAGE, UART_WRITER_STORAGE};

#[derive(Default)]
pub struct TaskTransmitImuDataContext;

pub fn task_transmit_imu_data(
    _: (),
    _: &mut TaskTransmitImuDataContext,
    _: &'static dyn RuntimeApi,
) {
    let imu = unsafe { IMU_STORAGE.as_mut().unwrap() };
    for _ in 0..imu.get_fifo_status().unwrap().stored_words {
        match imu.get_next_fifo_word().unwrap() {
            FifoWord::Gyroscope(angular_rate) => {
                Telemetry::new_gyroscope_data(angular_rate)
                    .write_ccsds_packet(unsafe { UART_WRITER_STORAGE.as_mut().unwrap() });
            }
            FifoWord::Accelerometer(linear_acceleration) => {
                Telemetry::new_accelerometer_data(linear_acceleration)
                    .write_ccsds_packet(unsafe { UART_WRITER_STORAGE.as_mut().unwrap() });
            }
            _ => {}
        }
    }
}
