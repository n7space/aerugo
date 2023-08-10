#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_rtt_target;
extern crate rtt_target;

use core::str::from_utf8;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use rtt_target::{rprint, rprintln, rtt_init_default, set_print_channel, DownChannel};

#[entry]
fn main() -> ! {
    let rtt = rtt_init_default!();
    set_print_channel(rtt.up.0);
    let mut input: DownChannel = rtt.down.0;
    let mut input_received = false;

    rprintln!("Hello from RTT!");

    while !input_received {
        let mut buffer: [u8; 32] = [0; 32];
        let received = input.read(&mut buffer);
        if received > 0 {
            rprintln!(
                "Received '{}'",
                from_utf8(&buffer).expect("Invalid string received!")
            );
            input_received = true;
        }
    }

    rprintln!("Running unit tests...");

    for i in 0..10 {
        rprint!("Test {}: ", i);
        for _ in 0..100000 {
            nop();
        }
        rprintln!("DONE!");
    }

    rprintln!("All tests executed!");

    loop {}
}
