use critical_section::CriticalSection;

use crate::api::{RuntimeApi, RuntimeError};
use crate::event::EventId;
use crate::execution_monitoring::ExecutionStats;
use crate::tasklet::TaskletId;
use crate::{Duration, Instant};

pub(crate) struct MockRuntimeApi;

impl RuntimeApi for MockRuntimeApi {
    fn emit_event(&'static self, _event_id: EventId) -> Result<(), RuntimeError> {
        todo!()
    }

    fn cancel_event(&'static self, _event_id: EventId) -> Result<(), RuntimeError> {
        todo!()
    }

    fn clear_event_queue(&'static self) {
        todo!()
    }

    fn get_system_time(&'static self) -> Instant {
        todo!()
    }

    fn set_system_time_offset(&'static self, _offset: Duration) -> Result<(), RuntimeError> {
        todo!()
    }

    /// Returns time elapsed between system initialization and start of the scheduler.
    /// If called before [`Aerugo::start`](crate::Aerugo::start), returns `None`.
    fn get_startup_time(&'static self) -> Option<Duration> {
        todo!()
    }

    /// Returns time elapsed since scheduler's start.
    /// If called before [`Aerugo::start`](crate::Aerugo::start), returns `None`.
    fn get_time_since_startup(&'static self) -> Option<Instant> {
        todo!()
    }

    fn query_tasks(&'static self) -> core::slice::Iter<TaskletId> {
        todo!()
    }

    fn get_execution_statistics(&'static self, _task_id: TaskletId) -> ExecutionStats {
        todo!()
    }

    fn execute_critical<F, R>(_f: F) -> R
    where
        F: FnOnce(CriticalSection) -> R,
    {
        todo!()
    }
}
