//! Simple logging utility for the x86 target.

/// Alias for `log!` macro, which prints a message.
///
/// <div class="warning">Missing newline may prevent "flushing" the RTT, try using `logln!`
/// when output is not being flushed correctly!</div>
///
/// <div class="warning">Call `Aerugo::initialize` before using this function!</div>
pub use rtt_target::rprint as log;
/// Alias for `logln!` macro, which prints a message and adds newline at the end.
///
/// <div class="warning">Call `Aerugo::initialize` before using this function!</div>
pub use rtt_target::rprintln as logln;

/// Function used to initialize logging facilities. Should be called once, on init.
#[inline(never)]
pub fn init_log() {
    rtt_target::rtt_init_print!();
}
