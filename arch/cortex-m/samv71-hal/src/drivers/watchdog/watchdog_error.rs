//! Module containing watchdog error types.

/// Possible watchdog errors
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WatchdogError {
    /// Tried to configure watchdog more than once.
    WatchdogAlreadyConfigured,
}
