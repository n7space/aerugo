//! Cyclic execution manager.
//!
//! This module contains cyclic execution manager. It's responsibility is to keep track of tasklets
//! that should be executed periodically.

use crate::aerugo::Aerugo;
use crate::cyclic_execution::CyclicExecution;
use crate::error::SystemError;
use crate::internal_list::InternalList;
use crate::tasklet::TaskletPtr;
use crate::time::Duration;
use crate::time_source::TimeSource;

/// List of cyclic executions registered in the system.
type CyclicExecutions = InternalList<CyclicExecution, { Aerugo::TASKLET_COUNT }>;

/// Cyclic execution manager.
///
/// This shouldn't be created by hand by the user or anywhere else in the code.
/// It should be used as a singleton (crate::aerugo::CYCLIC_EXECUTION_MANAGER) and shouldn't be directly accessed
/// by any other part of the system.
pub(crate) struct CyclicExecutionManager {
    /// Registered cyclic executions.
    cyclic_executions: CyclicExecutions,
    /// Time source.
    time_source: &'static TimeSource,
}

impl CyclicExecutionManager {
    /// Creates new cyclic execution manager instance.
    ///
    /// # Safety
    /// This shouldn't be called more than once.
    pub(crate) const fn new(time_source: &'static TimeSource) -> Self {
        CyclicExecutionManager {
            cyclic_executions: CyclicExecutions::new(),
            time_source,
        }
    }

    /// Creates new cyclic execution and registers it in the manager.
    ///
    /// # Parameters
    /// * `tasklet`: Tasklet that will be executed
    /// * `period`: Period for execution, `None` if tasklet shall be executed without waits
    ///
    /// # Return
    /// Reference to the cyclic execution data if successful, `SystemError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of cyclic executions.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn create_cyclic_execution(
        &'static self,
        tasklet: TaskletPtr,
        period: Option<Duration>,
    ) -> Result<&'static CyclicExecution, SystemError> {
        let cyclic_execution = CyclicExecution::new(tasklet, period);

        match self.cyclic_executions.add(cyclic_execution) {
            Ok(_) => (),
            Err(_) => return Err(SystemError::CyclicExecutionListFull),
        };

        Ok(self.cyclic_executions.last().unwrap())
    }

    /// Wakes all cyclic tasklets.
    pub(crate) fn wake_tasklets(&'static self) {
        for ce in &self.cyclic_executions {
            let system_time = self.time_source.system_time();
            ce.wake_if_should_execute(system_time);
        }
    }
}

unsafe impl Sync for CyclicExecutionManager {}
