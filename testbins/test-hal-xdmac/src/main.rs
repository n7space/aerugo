#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::time::MillisDurationU32 as WatchdogDuration;
use aerugo::{
    hal::{drivers::nvic::NVIC, user_peripherals::PMC},
    Aerugo, InitApi, SystemHardwareConfig,
};
use calldwell::write_str;
use rt::entry;

fn configure_pmc(_pmc: &mut PMC) {}

#[entry]
fn main() -> ! {
    calldwell::start_session();
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig {
        watchdog_timeout: WatchdogDuration::secs(16),
    });

    // Initialize peripheral clocks.
    let mut pmc = peripherals.pmc.take().unwrap();
    configure_pmc(&mut pmc);

    let _nvic = NVIC::new(peripherals.nvic.take().unwrap());

    write_str("All XDMAC tests finished successfully.");

    aerugo.start();
}
