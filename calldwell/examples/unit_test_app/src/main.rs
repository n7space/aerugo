#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_rtt_target;
extern crate rtt_target;

use core::{cell::RefCell, str::from_utf8};
use cortex_m::{asm::nop, interrupt as irq, interrupt::Mutex};
use cortex_m_rt::entry;
use rtt_target::{rprint, rprintln, rtt_init_default, set_print_channel, DownChannel};

static STDIN: Mutex<RefCell<Option<DownChannel>>> = Mutex::new(RefCell::new(None));

#[inline(never)]
fn init_tests() {
    irq::free(|cs| {
        let rtt = rtt_init_default!();
        set_print_channel(rtt.up.0);
        STDIN.borrow(cs).replace(Some(rtt.down.0));
    });
}

fn read_stdin(buffer: &mut [u8]) -> usize {
    irq::free(|cs| {
        let mut stream_ref = STDIN.borrow(cs).borrow_mut();
        let stream = stream_ref.as_mut().unwrap();
        stream.read(buffer)
    })
}

#[entry]
fn main() -> ! {
    init_tests();
    let mut input_received = false;

    rprintln!("Hello from RTT!");

    while !input_received {
        let mut buffer: [u8; 32] = [0; 32];
        let received = read_stdin(&mut buffer);
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
