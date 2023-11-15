#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::hal::drivers::pmc::config::PeripheralId;
use aerugo::hal::drivers::spi::Spi;
use aerugo::hal::drivers::xdmac::Xdmac;
use aerugo::time::MillisDurationU32 as WatchdogDuration;
use aerugo::{
    hal::{drivers::nvic::NVIC, user_peripherals::PMC},
    Aerugo, InitApi, SystemHardwareConfig,
};
use calldwell::write_str;
use rt::entry;

mod config;

fn configure_pmc(pmc: &mut PMC) {
    pmc.enable_peripheral_clock(PeripheralId::XDMAC);
    pmc.enable_peripheral_clock(PeripheralId::SPI0);
}

#[entry]
fn main() -> ! {
    calldwell::start_session();
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig {
        watchdog_timeout: WatchdogDuration::secs(16),
    });

    let mut pmc = peripherals.pmc.take().unwrap();
    let spi = Spi::new(peripherals.spi_0.take().unwrap());
    let _nvic = NVIC::new(peripherals.nvic.take().unwrap());
    let _xdmac = Xdmac::new(peripherals.xdmac.take().unwrap());

    configure_pmc(&mut pmc);

    write_str("Performing SPI tests...");

    let _spi = config::perform_test(spi);

    write_str("All SPI tests done!");

    aerugo.start();
}
