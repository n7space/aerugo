//! Module containing HAL error types.

/// HAL initialization error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HalError {
    /// Error indicating that the requested operation was called before HAL creation.
    HalNotInitialized,
    /// Error indicating that HAL was tried to be created twice.
    HalAlreadyInitialized,
    /// Error indicating that system was tried to be initialized twice.
    HardwareAlreadyInitialized,
}
