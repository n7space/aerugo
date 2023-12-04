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
    config::{
        control::{
            AccelerometerConfig, AccelerometerDataRate, AccelerometerOutputSelection,
            AccelerometerScale, AccelerometerTestMode, GyroscopeConfig, GyroscopeDataRate,
            GyroscopeScale, GyroscopeTestMode,
        },
        fifo::config::{
            AccelerometerBatchingRate, DataRateChangeBatching, FifoConfig, FifoMode,
            FifoWatermarkThreshold, GyroscopeBatchingRate, StopOnWatermarkThreshold,
        },
    },
    LSM6DSO,
};
use rt::entry;

use crate::tasklets::init_system;

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
    lsm.software_reset().unwrap();
    lsm.reboot_memory_content().unwrap();

    let fifo_config = FifoConfig {
        watermark_threshold: FifoWatermarkThreshold::new(50).unwrap(),
        odr_change_batched: DataRateChangeBatching::Enabled,
        stop_on_watermark: StopOnWatermarkThreshold::No,
        gyroscope_batching_rate: GyroscopeBatchingRate::Batch12_5Hz,
        accelerometer_batching_rate: AccelerometerBatchingRate::Batch12_5Hz,
        mode: FifoMode::Fifo,
    };
    lsm.set_fifo_config(fifo_config).unwrap();
    logln!("New LSM FIFO config: {:#?}", lsm.get_fifo_config());

    let accelerometer_config = AccelerometerConfig {
        data_rate: AccelerometerDataRate::Rate12_5Hz,
        scale: AccelerometerScale::Scale4g,
        output_selection: AccelerometerOutputSelection::FirstStageFilter,
    };

    let gyroscope_config = GyroscopeConfig {
        data_rate: GyroscopeDataRate::Rate12_5Hz,
        scale: GyroscopeScale::Scale500dps,
    };

    lsm.set_accelerometer_test_mode(AccelerometerTestMode::Negative)
        .unwrap();
    lsm.set_gyroscope_test_mode(GyroscopeTestMode::Positive)
        .unwrap();

    logln!(
        "Accelerometer mode: {:?}, gyroscope mode: {:?}",
        lsm.get_accelerometer_test_mode(),
        lsm.get_gyroscope_test_mode()
    );

    lsm.set_accelerometer_config(accelerometer_config).unwrap();
    lsm.set_gyroscope_config(gyroscope_config).unwrap();
    logln!(
        "New LSM accelerometer config: {:#?}",
        lsm.get_accelerometer_config()
    );
    logln!(
        "New LSM gyroscope config: {:#?}",
        lsm.get_gyroscope_config()
    );

    init_system(aerugo, lsm);

    logln!("System is starting!");
    aerugo.start();
}
