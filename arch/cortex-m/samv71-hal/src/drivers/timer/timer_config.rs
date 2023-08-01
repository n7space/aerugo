//! Module containing configuration structures for Timer

/// External clock signal source.
///
/// This represents available source clock signals for channel's external clock inputs.
pub enum ExternalClockSource {
    /// Timer clock (default source)
    TCLKx,
    /// Channel 0 output A
    TIOA0,
    /// Channel 1 output A
    TIOA1,
    /// Channel 2 output A
    TIOA2,
}

/// External clock signal selection configuration structure.
/// This can be used to configure external clock inputs for all channels at once,
/// to reduce amount of writes performed to timer registers.
pub type ExternalClockSignalSelectionConfig = [ExternalClockSource; 3];
