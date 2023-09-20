#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::{hal::drivers::pio::Port, Aerugo, InitApi, SystemHardwareConfig};
use rt::entry;

#[entry]
fn main() -> ! {
    calldwell::start_session();

    let (aerugo, peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    let port = Port::new(
        peripherals
            .pio_c
            .expect("PIO A missing from peripherals structure"),
    );

    let mut pins = port.into_pins();

    let _pc2 = pins[2].take().unwrap();

    aerugo.start();
}
