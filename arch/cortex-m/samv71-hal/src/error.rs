//! Module containing HAL error types.

/// HAL initialization error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HalError {
    /// Error indicating that HAL has already been created once.
    HalAlreadyCreated,
}
