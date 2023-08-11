//! System runtime API.
//!
//! This API can be used by the user in tasklet functions to interact with the system.

use bare_metal::CriticalSection;

use crate::api::RuntimeError;
use crate::event::EventId;
use crate::execution_monitoring::ExecutionStats;
use crate::tasklet::TaskletId;

/// System runtime API.
pub trait RuntimeApi {
    /// Emits event of given ID.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to emit.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    fn emit_event(&'static self, event_id: EventId) -> Result<(), RuntimeError>;

    /// Cancels event of given ID.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to cancel.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    fn cancel_event(&'static self, event_id: EventId) -> Result<(), RuntimeError>;

    /// Gets current system time timestamp.
    fn get_system_time(&'static self) -> crate::time::TimerInstantU64<1_000_000>;

    /// Sets system time offset.
    ///
    /// # Parameters
    /// * `offset` - Time offset.
    fn set_system_time_offset(&'static self, offset: crate::time::TimerDurationU64<1_000_000>);

    /// Returns an iterator to the list with IDs of registered tasklets.
    fn query_tasks(&'static self) -> core::slice::Iter<TaskletId>;

    /// Returns execution statistics for given tasklet.
    ///
    /// # Parameters
    /// * `task_id` - ID of the task to
    ///
    /// # Return
    /// Execution statistics for this tasklet.
    fn get_execution_statistics(&'static self, task_id: TaskletId) -> ExecutionStats;

    /// Enters critical section
    fn enter_critical()
    where
        Self: Sized;

    /// Exits critical section
    fn exit_critical()
    where
        Self: Sized;

    /// Executes closure `f` in an interrupt-free context.
    ///
    /// # Generic Parameters
    /// * `F` - Closure type.
    /// * `R` - Closure return type.
    ///
    /// # Parameters
    /// * `f` - Closure to execute.
    ///
    /// # Return
    /// Closure result.
    fn execute_critical<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
        Self: Sized;
}
