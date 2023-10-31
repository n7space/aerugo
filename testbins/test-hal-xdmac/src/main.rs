#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::hal::drivers::xdmac::Xdmac;
use aerugo::time::MillisDurationU32 as WatchdogDuration;
use aerugo::{
    hal::{drivers::nvic::NVIC, user_peripherals::PMC},
    Aerugo, InitApi, SystemHardwareConfig,
};
use calldwell::write_str;
use rt::entry;

mod xdmac_management;

fn configure_pmc(_pmc: &mut PMC) {}

#[entry]
fn main() -> ! {
    calldwell::start_session();
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig {
        watchdog_timeout: WatchdogDuration::secs(16),
    });

    let mut pmc = peripherals.pmc.take().unwrap();
    let _nvic = NVIC::new(peripherals.nvic.take().unwrap());
    let mut xdmac = Xdmac::new(peripherals.xdmac.take().unwrap());

    configure_pmc(&mut pmc);

    write_str("Performing XDMAC tests...");

    xdmac_management::perform_test(&mut xdmac);

    write_str("All XDMAC tests finished successfully.");

    aerugo.start();
}
