#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::{
    hal::drivers::pio::{port::IntoPins, Port},
    Aerugo, InitApi, SystemHardwareConfig,
};
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

    let pins = port.into_pins();

    let _is_high = pins.pc11.state();

    aerugo.start();
}
