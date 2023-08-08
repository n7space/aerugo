//! Possible system runtime errors.

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
