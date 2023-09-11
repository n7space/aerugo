//! Possible system initialization errors.

use crate::event::EventId;

/// System initialization error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InitError {
    /// Initialized one storage twice.
    StorageAlreadyInitialized,
    /// Set more than one condition set to tasklet.
    TaskletAlreadyConditioned,
    /// Subscribed tasklet to more than one data provider.
    TaskletAlreadySubscribed,
    /// Tasklet list was full when tried to register.
    TaskletListFull,
    /// Event already exists in the system.
    EventAlreadyExists(EventId),
    /// Event with given ID was not found.
    EventNotFound(EventId),
    /// Event list was full when tried to create a new one.
    EventListFull,
    /// Event set was full when tried to add another event.
    EventSetFull,
    /// Event set list was full when tried to create a new one.
    EventSetListFull,
    /// Cyclic execution list was full when tried to create a new one.
    CyclicExecutionListFull,
}
