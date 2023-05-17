/// System runtime API.
///
/// This API can be used by the user in tasklet functions to interact with the system.

/// System runtime API.
pub trait RuntimeApi: ErrorType {}

/// Runtime error
pub trait Error: core::fmt::Debug {}

/// Runtime error type trait
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}
