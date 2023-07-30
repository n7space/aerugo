//! Generic task.

/// Task status.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TaskletStatus {
    /// Task is inactive.
    Sleeping,
    /// Task is waiting for being executed.
    Waiting,
    /// Task is being executed.
    Working,
}
