//! Generic task.

/// Trait for generic task.
pub(crate) trait Task {
    /// Returns task name.
    fn get_name(&self) -> &'static str;

    /// Executes task.
    fn execute(&self);
}

/// Task unique ID.
pub struct TaskId(u32);
