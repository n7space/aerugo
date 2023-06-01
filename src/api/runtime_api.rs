/// System runtime API.
///
/// This API can be used by the user in tasklet functions to interact with the system.
use core::ops::{Add, Sub};

use crate::execution_monitoring::ExecutionStats;
use crate::task::TaskId;

/// System runtime API.
pub trait RuntimeApi: ErrorType {
    /// Type for an instant in time.
    type Instant: Ord
        + Copy
        + Add<Self::Duration, Output = Self::Instant>
        + Sub<Self::Duration, Output = Self::Instant>
        + Sub<Self::Instant, Output = Self::Duration>;

    /// Type for a duration of time.
    type Duration;

    /// Gets current system time timestamp.
    fn get_system_time(&'static self) -> Self::Instant;

    /// Sets system time offset.
    ///
    /// * `offset` - Time offset.
    fn set_system_time_offset(&'static self, offset: Self::Duration);

    /// Returns an iterator to the list with IDs of registered tasklets.
    fn query_tasks(&'static self) -> core::slice::Iter<TaskId>;

    /// Returns execution statistics for given tasklet.
    ///
    /// * `task_id` - ID of the task to
    fn get_execution_statistics(&'static self, task_id: TaskId) -> ExecutionStats;
}

/// Runtime error
pub trait Error: core::fmt::Debug {}

/// Runtime error type trait
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}
