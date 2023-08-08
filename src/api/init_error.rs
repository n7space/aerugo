//! Possible system initialization errors.

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
    /// Cyclic execution list was full when tried to create a new one.
    CyclicExecutionListFull,
}
