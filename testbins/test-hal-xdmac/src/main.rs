#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::hal::drivers::pmc::config::PeripheralId;
use aerugo::hal::drivers::uart::Uart;
use aerugo::hal::drivers::xdmac::Xdmac;
use aerugo::time::MillisDurationU32 as WatchdogDuration;
use aerugo::{
    hal::{drivers::nvic::NVIC, user_peripherals::PMC},
    Aerugo, InitApi, SystemHardwareConfig,
};
use calldwell::write_str;
use rt::entry;

mod channel_management;
mod transfer_config;
mod xdmac_management;

fn configure_pmc(pmc: &mut PMC) {
    pmc.enable_peripheral_clock(PeripheralId::XDMAC);
    pmc.enable_peripheral_clock(PeripheralId::UART0);
}

#[entry]
fn main() -> ! {
    calldwell::start_session();
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig {
        watchdog_timeout: WatchdogDuration::secs(16),
    });

    let mut pmc = peripherals.pmc.take().unwrap();
    let _nvic = NVIC::new(peripherals.nvic.take().unwrap());
    let _uart = Uart::new(peripherals.uart_0.take().unwrap());
    let mut xdmac = Xdmac::new(peripherals.xdmac.take().unwrap());

    configure_pmc(&mut pmc);

    write_str("Performing XDMAC tests...");

    xdmac_management::perform_test(&mut xdmac);
    channel_management::perform_test(&mut xdmac);
    transfer_config::perform_test();

    write_str("All XDMAC tests finished successfully.");

    aerugo.start();
}
