#![no_std]
#![no_main]

extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_rtt_target;
extern crate rtt_target;

use core::str::from_utf8;

use calldwell::{with_rtt_in, with_rtt_out};
use core::fmt::Write;
use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut input_buffer: [u8; 128] = [0; 128];
    calldwell::initialize();

    with_rtt_out(|o, _| o.write_str("Hello from SAMV71!"));

    let read_bytes = with_rtt_in(|i, _| i.read(&mut input_buffer));

    match read_bytes {
        Ok(amount) => with_rtt_out(|o, _| {
            write!(
                o.writer(),
                "Received {} bytes: {}",
                amount,
                from_utf8(&input_buffer[..amount]).expect("Invalid string received!")
            )
            .unwrap();
        }),
        Err(e) => with_rtt_out(|o, _| {
            write!(
                o.writer(),
                "Error occurred while receiving message: {:?}",
                e
            )
            .unwrap();
        }),
    }

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
