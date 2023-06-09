//! Generic task.

use crate::time::TimerInstantU64;

/// Task status.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TaskStatus {
    /// Task is inactive.
    Sleeping,
    /// Task is waiting for being executed.
    Waiting,
    /// Task is being executed.
    Working,
}

/// Trait for generic task.
pub(crate) trait Task {
    /// Returns task name.
    fn get_name(&self) -> &'static str;

    /// Returns task status.
    fn get_status(&self) -> TaskStatus;

    /// Sets task status.
    ///
    /// * `status` - New task status.
    fn set_status(&self, status: TaskStatus);

    /// Returns last execution time.
    fn get_last_execution_time(&self) -> TimerInstantU64<1_000_000>;

    /// Sets last execution time.
    ///
    /// * `time` - Last execution time.
    fn set_last_execution_time(&self, time: TimerInstantU64<1_000_000>);

    /// Checks if there are any more data for the tasklet to process.
    fn has_work(&self) -> bool;

    /// Executes task.
    fn execute(&self);
}

/// Task unique ID.
pub struct TaskId(u32);
