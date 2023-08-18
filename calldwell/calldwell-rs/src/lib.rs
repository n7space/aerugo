//! Library providing software testing utilities for microcontrollers.
//! Also provides a panic handler that works over RTT.
//! Part of Calldwell testing framework.

#![cfg(all(target_arch = "arm", target_os = "none"))]
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

mod streams;

use core::panic;
use core::{cell::RefCell, str::from_utf8};

use core::fmt::Write;
use critical_section::{CriticalSection, Mutex};
use rtt_target::rtt_init;
use streams::{DownStream, UpStream};

/// RTT channel acting as standard input.
static RTT_IN: Mutex<RefCell<Option<DownStream>>> = Mutex::new(RefCell::new(None));
/// RTT channel acting as standard output.
static RTT_OUT: Mutex<RefCell<Option<UpStream>>> = Mutex::new(RefCell::new(None));

/// Initializes Calldwell's test framework and connects to test host via RTT.
/// Should be called as soon as possible in the program.
///
/// Will panic on error caused by miscommunication with test host.
pub fn start_test_session() {
    initialize();

    with_rtt_out(|o, _| o.write_str("calldwell-rs started"));

    let mut input_buffer: [u8; 16] = [0; 16];
    let read_bytes = with_rtt_in(|i, _| i.read(&mut input_buffer));

    with_rtt_out(|o, _| match read_bytes {
        Ok(amount) => write!(
            o.writer(),
            "{}:{}",
            amount,
            from_utf8(&input_buffer[..amount])
                .expect("invalid string received from test host during handshake")
        )
        .unwrap(),
        Err(e) => {
            write!(o.writer(), "{:?}", e).unwrap();
            panic!("a ReceptionError occurred while handshaking with test host")
        }
    });
}

/// Initializes Calldwell's I/O. Calldwell host should hook to this function and
/// wait until it finishes before starting RTT communication facilities.
///
/// This function runs in a critical section.
#[inline(never)]
fn initialize() {
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
        RTT_IN.borrow(cs).replace(Some(rtt_in));
        RTT_OUT.borrow(cs).replace(Some(rtt_out));
    });
}

/// Calls provided functor with mutable Calldwell input stream reference.
/// Creates a critical section, and passes it to the functor as argument
/// along with the stream.
///
/// # Generic Parameters
/// * `F` - Functor type, must accept two arguments: `&mut DownStream` and `CriticalSection`.
///         Can return a value of any type.
/// * `T` - Type of value returned from functor `f`.
///
/// # Parameters
/// * `f` - Functor to call
///
/// # Returns
/// Value returned from functor `f`.
pub fn with_rtt_in<F, T>(f: F) -> T
where
    F: FnOnce(&mut DownStream, CriticalSection) -> T,
{
    critical_section::with(|cs| {
        let mut rtt_in_ref = RTT_IN.borrow(cs).borrow_mut();
        let rtt_in = rtt_in_ref
            .as_mut()
            .expect("you must initialize Calldwell before using data streams");

        f(rtt_in, cs)
    })
}

/// Calls provided functor with mutable Calldwell output stream reference.
/// Creates a critical section, and passes it to the functor as argument
/// along with the stream.
///
/// # Generic Parameters
/// * `F` - Functor type, must accept two arguments: `&mut UpStream` and `CriticalSection`.
///         Can return a value of any type.
/// * `T` - Type of value returned from functor `f`.
///
/// # Parameters
/// * `f` - Functor to call
///
/// # Returns
/// Value returned from functor `f`.
pub fn with_rtt_out<F, T>(f: F) -> T
where
    F: FnOnce(&mut UpStream, CriticalSection) -> T,
{
    critical_section::with(|cs| {
        let mut rtt_out_ref = RTT_OUT.borrow(cs).borrow_mut();
        let rtt_out = rtt_out_ref
            .as_mut()
            .expect("you must initialize Calldwell before using data streams");

        f(rtt_out, cs)
    })
}

/// Calls provided functor with mutable Calldwell input and output stream reference.
/// Creates a critical section, and passes it to the functor as argument
/// along with the stream.
///
/// # Generic Parameters
/// * `F` - Functor type, must accept three arguments: `&mut DownStream`, `&mut UpStream` and `CriticalSection`.
///         Can return a value of any type.
/// * `T` - Type of value returned from functor `f`.
///
/// # Parameters
/// * `f` - Functor to call
///
/// # Returns
/// Value returned from functor `f`.
pub fn with_rtt_in_out<F, T>(f: F) -> T
where
    F: FnOnce(&mut DownStream, &mut UpStream, CriticalSection) -> T,
{
    critical_section::with(|cs| {
        let mut rtt_in_ref = RTT_IN.borrow(cs).borrow_mut();
        let rtt_in = rtt_in_ref
            .as_mut()
            .expect("you must initialize Calldwell before using data streams");

        let mut rtt_out_ref = RTT_OUT.borrow(cs).borrow_mut();
        let rtt_out = rtt_out_ref
            .as_mut()
            .expect("you must initialize Calldwell before using data streams");

        f(rtt_in, rtt_out, cs)
    })
}
