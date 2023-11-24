#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate lsm6dso;
extern crate panic_rtt_target;

use aerugo::{
    hal::drivers::{pio::Port, spi::Spi},
    logln, Aerugo, InitApi, SystemHardwareConfig,
};
use lsm6dso::{
    config::fifo::{
        AccelerometerBatchingRate, DataRateChangeBatching, FifoConfig, FifoMode,
        FifoWatermarkThreshold, GyroscopeBatchingRate, StopOnWatermarkThreshold,
    },
    LSM6DSO,
};
use rt::entry;

mod hardware_config;
mod tasklets;

#[entry]
fn main() -> ! {
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());
    logln!("Hello, world! Aerugo initialized! Initializing hardware...");

    let mut pmc = peripherals.pmc.take().unwrap();
    let spi = Spi::new(peripherals.spi_0.take().unwrap());
    let port_d = Port::new(peripherals.pio_d.take().unwrap());

    hardware_config::configure_pmc(&mut pmc);
    let _lsm_pins = hardware_config::configure_pio(port_d);
    let lsm_spi = hardware_config::configure_spi(spi);

    let mut lsm: LSM6DSO<_, 32> = LSM6DSO::new(lsm_spi).unwrap();

    logln!("LSM id: {:2X?}", lsm.id());
    logln!("Is LSM alive? {:?}", lsm.is_alive());
    logln!("Current LSM config: {:#?}", lsm.get_fifo_config());

    let test_config = FifoConfig {
        watermark_threshold: FifoWatermarkThreshold::new(123).unwrap(),
        odr_change_batched: DataRateChangeBatching::Enabled,
        stop_on_watermark: StopOnWatermarkThreshold::Yes,
        gyroscope_batching_rate: GyroscopeBatchingRate::Batch26Hz,
        accelerometer_batching_rate: AccelerometerBatchingRate::Batch417Hz,
        mode: FifoMode::Fifo,
    };
    lsm.set_fifo_config(test_config).unwrap();
    let read_config = lsm.get_fifo_config();
    logln!("New LSM config: {:#?}", read_config);
    assert_eq!(read_config.unwrap(), test_config);

    aerugo.start();
}
