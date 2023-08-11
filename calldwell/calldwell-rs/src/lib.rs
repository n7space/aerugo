//! Library providing software testing utilities for microcontrollers.
//! Also provides a panic handler that works over RTT.
//! Part of Calldwell testing framework.

#![cfg(all(target_arch = "arm", target_os = "none"))]
#![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]

mod streams;

use core::cell::RefCell;

use critical_section::Mutex;
use rtt_target::rtt_init;
use streams::{DownStream, Idle, UpStream};

/// RTT channel acting as standard input.
static CALLDWELL_RTT_IN: Mutex<RefCell<Option<DownStream<Idle>>>> = Mutex::new(RefCell::new(None));
/// RTT channel acting as standard output.
static CALLDWELL_RTT_OUT: Mutex<RefCell<Option<UpStream<Idle>>>> = Mutex::new(RefCell::new(None));

/// Initializes Calldwell's I/O. Call as soon as possible in the program,
/// to make Calldwell's RTT facilities available.
///
/// This function runs in a critical section.
#[inline(never)]
pub fn initialize() {
    critical_section::with(|cs| {
        let channels = rtt_init! {
            up: {
                0: {
                    size: 1024
                    name: "CalldwellStdout"
                }
            }
            down: {
                0: {
                    size: 1024
                    name: "CalldwellStdin"
                }
            }
        };

        let rtt_in = DownStream::new(channels.down.0);
        let rtt_out = UpStream::new(channels.up.0);
        CALLDWELL_RTT_IN.borrow(cs).replace(Some(rtt_in));
        CALLDWELL_RTT_OUT.borrow(cs).replace(Some(rtt_out));
    });
}
