#![no_std]
#![no_main]

extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate rtt_target;

use calldwell::with_rtt_out;
use core::fmt::Write;
use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    calldwell::start_test_session();

    with_rtt_out(|o, _| o.write_str("Running tests..."));

    for i in 0..10 {
        with_rtt_out(|o, _| write!(o.writer(), "Starting test #{}", i).unwrap());

        for _ in 0..100000 {
            asm::nop();
        }

        with_rtt_out(|o, _| write!(o.writer(), "Test #{} finished!", i).unwrap());
    }

    with_rtt_out(|o, _| o.write_str("All tests done!"));

    loop {}
}
