//! Simple logging utility for the x86 target.

/// Alias for `log!` macro, which prints a message.
pub use std::print as log;
/// Alias for `logln!` macro, which prints a message and adds newline at the end.
pub use std::println as logln;

/// Function used to initialize logging facilities. Should be called once, on init.
pub fn init_log() {
    // No-op on x86
}
