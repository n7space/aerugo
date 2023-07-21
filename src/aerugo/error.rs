//! Possible system errors.

use super::Aerugo;

use crate::api::{init_api, runtime_api};

/// System initialization error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InitError {
    /// Initialized one storage twice.
    StorageAlreadyInitialized,
    /// Subscribed tasklet to more than one data provider.
    TaskletAlreadySubscribed,
    /// Tasklet list was full when tried to register.
    TaskletListFull,
}

impl init_api::Error for InitError {}

impl init_api::ErrorType for Aerugo {
    type Error = InitError;
}

/// System runtime error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RuntimeError {
    /// Enqueued too many tasklets.
    ExecutorTaskletQueueFull,
    /// Enqueued data to a full data queue.
    DataQueueFull,
    /// Tried to perform an operation before system initialization.
    SystemNotInitialized,
    /// Tried to initialize system more than once
    SystemAlreadyInitialized,
}

impl runtime_api::Error for RuntimeError {}

impl runtime_api::ErrorType for Aerugo {
    type Error = RuntimeError;
}
