#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

#[entry]
fn main() -> ! {
    asm::nop();

    debug::exit(debug::EXIT_SUCCESS);

    #[allow(clippy::empty_loop)]
    loop {}
}
