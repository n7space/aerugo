//! Module containing timer-related error structures and traits.

/// Timer configuration error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TimerConfigurationError {
    /// Invalid channel ID was provided.
    InvalidChannelId,
    /// Invalid clock source was selected for external clock signal.
    InvalidClockSourceForExternalClock,
}
