//! Module containing timer-related error structures and traits.

/// Timer configuration error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TimerConfigurationError {
    /// Invalid clock source was selected.
    InvalidClockSource,
}
