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
    config::control::{
        AccelerometerConfig, AccelerometerDataRate, AccelerometerOutputSelection,
        AccelerometerScale,
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
    logln!("Pre-reboot LSM config: {:#?}", lsm.get_fifo_config());
    lsm.software_reset().unwrap();
    lsm.reboot_memory_content().unwrap();

    let accel_cfg_a = AccelerometerConfig {
        data_rate: AccelerometerDataRate::Rate208Hz,
        scale: AccelerometerScale::Scale4g,
        output_selection: AccelerometerOutputSelection::FirstStageFilter,
    };
    let accel_cfg_b = AccelerometerConfig {
        data_rate: AccelerometerDataRate::Rate3333Hz,
        scale: AccelerometerScale::Scale8g,
        output_selection: AccelerometerOutputSelection::LPF2SecondFilter,
    };

    logln!("Original config: {:#?}", lsm.get_accelerometer_config());
    logln!("Register value: {:#02X?}", lsm.get_reg());
    lsm.set_accelerometer_config(accel_cfg_a).unwrap();
    logln!("New config A: {:#?}", lsm.get_accelerometer_config());
    logln!("Register value: {:#02X?}", lsm.get_reg());
    lsm.set_accelerometer_config(accel_cfg_b).unwrap();
    logln!("New config B: {:#?}", lsm.get_accelerometer_config());
    logln!("Register value: {:#02X?}", lsm.get_reg());

    aerugo.start();
}
