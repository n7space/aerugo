//! Possible system runtime errors.

use crate::event::EventId;

/// System runtime error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RuntimeError {
    /// Enqueued too many tasklets.
    ExecutorTaskletQueueFull,
    /// Enqueued data to a full data queue.
    DataQueueFull,
    /// Event with given ID was not found.
    EventNotFound(EventId),
    /// Enqueued event to a full event queue.
    EventQueueFull,
    /// Tried to perform an operation before system initialization.
    SystemNotInitialized,
    /// Tried to initialize system more than once.
    SystemAlreadyInitialized,
    /// Tried to set user time offset more than once.
    UserTimeOffsetAlreadySet,
    /// Tried to set system time start more than once.
    SystemTimeStartAlreadySet,
}
