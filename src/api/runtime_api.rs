//! System runtime API.
//!
//! This API can be used by the user in tasklet functions to interact with the system.

use critical_section::CriticalSection;

use crate::error::RuntimeError;
use crate::event::EventId;
use crate::execution_monitor::ExecutionStats;
use crate::tasklet::TaskletId;
use crate::time::{Duration, Instant};

/// System runtime API.
///
/// # Safety
/// This should only be exposed to the user in tasklet step function (crate::tasklet::StepFn) and
/// used only in the scope of that step function. If that interface is leaked then all functions
/// can be considered unsafe.
pub trait RuntimeApi {
    /// Emits event of given ID.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to emit.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    fn emit_event(&'static self, event_id: EventId) -> Result<(), RuntimeError>;

    /// Schedules event of given ID at the given absolute time.
    ///
    /// Time is counted from the system scheduler start.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to emit.
    /// * `time` - Absolute time when event should be emitted.
    ///
    /// # Return
    /// `bool` indicating if emit was successfully scheduled, `RuntimeError` if some error
    /// occurred.
    ///
    fn schedule_event(
        &'static self,
        event_id: EventId,
        time: Instant,
    ) -> Result<bool, RuntimeError>;

    /// Schedules event of given ID at given time counted from the system scheduler start.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to emit.
    /// * `time` - Time since the scheduler start when event should be emitted.
    ///
    /// # Return
    /// `bool` indicating if emit was successfully scheduled, `RuntimeError` if some error
    /// occurred.
    fn schedule_event_at(
        &'static self,
        event_id: EventId,
        time: Duration,
    ) -> Result<bool, RuntimeError>;

    /// Schedules event of given ID in time counting from current system time.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to emit.
    /// * `time` - Time since current system time when event should be emitted.
    ///
    /// # Return
    /// `bool` indicating if emit was successfully scheduled, `RuntimeError` if some error
    /// occurred.
    fn schedule_event_in(
        &'static self,
        event_id: EventId,
        time: Duration,
    ) -> Result<bool, RuntimeError>;

    /// Checks if event of given ID is scheduled to be emitted.
    ///
    /// If event was already scheduled at this time, that event will be rescheduled to the given time.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to check.
    ///
    /// # Return
    /// `bool` indicating if event was rescheduled, `RuntimeError` if some error occurred.
    fn is_event_scheduled(&'static self, event_id: EventId) -> Result<bool, RuntimeError>;

    /// Cancels event of given ID.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to cancel.
    ///
    /// # Return
    /// `bool` indicating if event was cancelled, `RuntimeError` if some error occurred.
    fn cancel_event(&'static self, event_id: EventId) -> Result<bool, RuntimeError>;

    /// Clears event queue.
    fn clear_event_queue(&'static self);

    /// Gets current system time timestamp.
    fn get_system_time(&'static self) -> Instant;

    /// Gets time elapsed since execution started.
    fn get_elapsed_time(&'static self) -> Duration;

    /// Sets system time offset.
    ///
    /// # Parameters
    /// * `offset` - Time offset.
    fn set_system_time_offset(&'static self, offset: Duration) -> Result<(), RuntimeError>;

    /// Returns time elapsed between system initialization and start of the scheduler.
    /// If called before scheduler's start, should return `None`.
    fn get_startup_duration(&'static self) -> Duration;

    /// Returns execution statistics for given tasklet.
    ///
    /// # Parameters
    /// * `task_id` - ID of the task to
    ///
    /// # Return
    /// Execution statistics for this tasklet.
    fn get_execution_statistics(&'static self, tasklet_id: &TaskletId) -> Option<ExecutionStats>;

    /// Returns an iterator to the list with IDs of registered tasklets.
    fn query_tasklets(&'static self) -> core::slice::Iter<TaskletId>;

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
        F: FnOnce(CriticalSection) -> R,
        Self: Sized;
}
