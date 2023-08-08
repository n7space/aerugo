//! System time manager.
//!
//! This module contains a system time manager. It's responsibility is to keep track of tasklets
//! and events that are based on time.

mod cyclic_execution;

use self::cyclic_execution::CyclicExecution;

use heapless::Vec;

use crate::aerugo::Aerugo;
use crate::api::InitError;
use crate::internal_cell::InternalCell;
use crate::tasklet::TaskletPtr;
use crate::time::MillisDurationU32;

/// List of cyclic executions registered in the system.
type CyclicExecutions = Vec<CyclicExecution, { Aerugo::TASKLET_COUNT }>;

/// System time manager.
///
/// This shouldn't be created by hand by the user or anywhere else in the code.
/// It should be used as a singleton (crate::aerugo::TIME_MANAGER) and shouldn't be directly accessed
/// by any other part of the system.
pub(crate) struct TimeManager {
    /// Registered cyclic executions.
    cyclic_executions: InternalCell<CyclicExecutions>,
}

impl TimeManager {
    /// Creates new time manager instance.
    ///
    /// # Safety
    /// This shouldn't be called more than once.
    pub(crate) const fn new() -> Self {
        TimeManager {
            cyclic_executions: InternalCell::new(CyclicExecutions::new()),
        }
    }

    /// Creates new cyclic execution and registers it in the manager.
    ///
    /// # Parameters
    /// * `tasklet`: Tasklet that will be executed
    /// * `period`: Period for execution, `None` if tasklet shall be executed without waits
    ///
    /// # Return
    /// Reference to the cyclic execution data if successful, `InitError` otherwise.
    pub(crate) unsafe fn create_cyclic_execution(
        &'static self,
        tasklet: TaskletPtr,
        period: Option<MillisDurationU32>,
    ) -> Result<&'static CyclicExecution, InitError> {
        let cyclic_execution = CyclicExecution::new(tasklet, period);

        match self.cyclic_executions.as_mut_ref().push(cyclic_execution) {
            Ok(_) => (),
            Err(_) => return Err(InitError::CyclicExecutionListFull),
        };

        unsafe { Ok(self.cyclic_executions.as_ref().last().unwrap()) }
    }

    /// Wakes all cyclic tasklets.
    pub(crate) fn wake_tasklets(&'static self) {
        for ce in unsafe { self.cyclic_executions.as_ref() } {
            ce.wake_tasklet();
        }
    }
}
