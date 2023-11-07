//! Data from tasklet execution.

use crate::tasklet::TaskletId;
use crate::time::{Duration, Instant};

/// This structure holds information about an execution of a tasklet.
pub(crate) struct ExecutionData {
    /// Tasklet ID.
    tasklet_id: TaskletId,
    /// Whether tasklet was executed or only woken up.
    executed: bool,
    /// Timestamp for the start of the execution.
    execution_start: Option<Instant>,
    /// Timestamp for the end of the execution.
    execution_end: Option<Instant>,
}

impl ExecutionData {
    /// Creates new instance.
    pub(crate) const fn new(tasklet_id: TaskletId) -> Self {
        Self {
            tasklet_id,
            executed: false,
            execution_start: None,
            execution_end: None,
        }
    }

    /// Returns tasklet ID.
    pub(crate) fn tasklet_id(&self) -> &TaskletId {
        &self.tasklet_id
    }

    /// Returns whether tasklet was executed or just woken up.
    pub(crate) fn was_executed(&self) -> bool {
        self.executed
    }

    /// Marks that tasklet was executed.
    pub(crate) fn set_executed(&mut self) {
        self.executed = true;
    }

    /// Saves timestamp for the start of the execution.
    pub(crate) fn set_execution_start(&mut self, execution_start: Instant) {
        self.execution_start.replace(execution_start);
    }

    /// Saves timestamp for the end of the execution.
    pub(crate) fn set_execution_end(&mut self, execution_end: Instant) {
        self.execution_end.replace(execution_end);
    }

    /// Calculates the execution duration. Returns `None` if tasklet wasn't executed.
    pub(crate) fn execution_duration(&self) -> Option<Duration> {
        match (self.execution_start, self.execution_end) {
            (Some(start), Some(end)) => Some(end - start),
            (_, _) => None,
        }
    }
}
