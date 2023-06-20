//! Generic task.

/// Module for Task handle.
mod task_handle;

pub use self::task_handle::TaskHandle;

/// Trait for generic task.
pub(crate) trait Task {
    /// Checks if task is ready for execution.
    ///
    /// Returns true if task is ready, false otherwise.
    fn is_ready(&self) -> bool;

    /// Executes task.
    fn execute(&self);
}

/// Task unique ID.
pub struct TaskId(u32);
