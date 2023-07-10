//! Possible system errors.

use super::Aerugo;

use crate::api::{init_api, runtime_api};

/// System initialization error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InitError {
    /// Error occurring when trying to initialize one storage twice.
    StorageAlreadyInitialized,
}

impl init_api::Error for InitError {}

impl init_api::ErrorType for Aerugo {
    type Error = InitError;
}

/// System runtime error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RuntimeError {
    /// Error occurring when trying to enqueue too many tasklets.
    ExecutorTaskletQueueFull,
}

impl runtime_api::Error for RuntimeError {}

impl runtime_api::ErrorType for Aerugo {
    type Error = RuntimeError;
}
