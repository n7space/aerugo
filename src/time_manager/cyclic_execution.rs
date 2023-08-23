//! Cyclic execution for tasklets.
//!
//! This module contains a structure which holds information about cyclic execution of tasklets.

use crate::aerugo::AERUGO;
use crate::api::SystemApi;
use crate::data_provider::DataProvider;
use crate::tasklet::TaskletPtr;
use crate::time::MillisDurationU32;

/// Cyclic execution information.
pub(crate) struct CyclicExecution {
    /// Tasklet subscribed for cyclic execution.
    tasklet: TaskletPtr,
    /// Period of cyclic execution.
    period: Option<MillisDurationU32>,
}

impl CyclicExecution {
    /// Creates new instance.
    pub(crate) fn new(tasklet: TaskletPtr, period: Option<MillisDurationU32>) -> Self {
        CyclicExecution { tasklet, period }
    }

    /// Wakes thet stored tasklet.
    pub(crate) fn wake_tasklet(&self) {
        AERUGO.wake_tasklet(&self.tasklet);
    }
}

impl DataProvider<()> for CyclicExecution {
    /// Returns `Some()`.
    fn get_data(&self) -> Option<()> {
        Some(())
    }

    fn data_waiting(&self) -> bool {
        // TODO: This will be changed when period is implemented
        match self.period {
            Some(_) => todo!(),
            None => true,
        }
    }
}
