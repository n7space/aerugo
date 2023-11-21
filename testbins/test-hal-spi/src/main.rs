#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::hal::drivers::pio::pin::Peripheral;
use aerugo::hal::drivers::pio::Port;
use aerugo::hal::drivers::pmc::config::PeripheralId;
use aerugo::hal::drivers::spi::Spi;
use aerugo::hal::drivers::xdmac::Xdmac;
use aerugo::hal::user_peripherals::PIOD;
use aerugo::time::MillisDurationU32 as WatchdogDuration;
use aerugo::{
    hal::{drivers::nvic::NVIC, user_peripherals::PMC},
    Aerugo, InitApi, SystemHardwareConfig,
};
use calldwell::write_str;
use rt::entry;

mod comms;
mod config;
mod lsm6dso;

fn configure_pmc(pmc: &mut PMC) {
    pmc.enable_peripheral_clock(PeripheralId::XDMAC);
    pmc.enable_peripheral_clock(PeripheralId::SPI0);
}

fn configure_pio(port: Port<PIOD>) {
    let mut pins = port.into_pins();

    let _miso_pin = pins[20].take().unwrap().into_peripheral_pin(Peripheral::B);
    let _mosi_pin = pins[21].take().unwrap().into_peripheral_pin(Peripheral::B);
    let _sck_pin = pins[22].take().unwrap().into_peripheral_pin(Peripheral::B);
    let _cs_pin = pins[25].take().unwrap().into_peripheral_pin(Peripheral::B);
    let _int1_pin = pins[28].take().unwrap().into_input_pin();
}

#[entry]
fn main() -> ! {
    calldwell::start_session();
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig {
        watchdog_timeout: WatchdogDuration::secs(16),
    });

    let mut pmc = peripherals.pmc.take().unwrap();
    let spi = Spi::new(peripherals.spi_0.take().unwrap());
    let mut nvic = NVIC::new(peripherals.nvic.take().unwrap());
    let mut xdmac = Xdmac::new(peripherals.xdmac.take().unwrap());
    let portd = Port::new(peripherals.pio_d.take().unwrap());

    configure_pmc(&mut pmc);
    configure_pio(portd);

    write_str("Performing SPI tests...");

    let spi = config::perform_test(spi);
    let spi = comms::perform_test(spi, &mut nvic);
    lsm6dso::perform_test(spi, &mut xdmac, &mut nvic);

    write_str("All SPI tests done!");

    aerugo.start();
}
