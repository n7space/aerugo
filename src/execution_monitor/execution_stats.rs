//! Tasklet execution statistics.

use core::fmt;

use crate::execution_monitor::ExecutionData;
use crate::tasklet::TaskletId;
use crate::time::Duration;

/// Tasklet execution statistics.
#[derive(Copy, Clone)]
pub struct ExecutionStats {
    /// Tasklet ID.
    tasklet_id: TaskletId,
    /// Number of times tasklet was woken up.
    wake_count: u32,
    /// Number of times tasklet was executed after being woken up.
    execution_count: u32,
    /// Total execution time.
    total_execution_time: Duration,
    /// Shortest execution time.
    minimum_execution_time: Option<Duration>,
    /// Longes execution time.
    maximum_execution_time: Option<Duration>,
}

impl ExecutionStats {
    /// Creates new execution statistics.
    pub(crate) const fn new(tasklet_id: TaskletId) -> Self {
        Self {
            tasklet_id,
            wake_count: 0,
            execution_count: 0,
            total_execution_time: Duration::from_ticks(0),
            minimum_execution_time: None,
            maximum_execution_time: None,
        }
    }

    /// Returns ID of tasklet to which those statistics belong.
    pub fn tasklet_id(&self) -> &TaskletId {
        &self.tasklet_id
    }

    /// Returns wake count.
    pub fn wake_count(&self) -> u32 {
        self.wake_count
    }

    /// Returns execution count.
    pub fn execution_count(&self) -> u32 {
        self.execution_count
    }

    /// Returns total execution time.
    pub fn total_execution_time(&self) -> Duration {
        self.total_execution_time
    }

    /// Returns shortest execution time.
    pub fn minimum_execution_time(&self) -> Option<Duration> {
        self.minimum_execution_time
    }

    /// Returns longest execution time.
    pub fn maximum_execution_time(&self) -> Option<Duration> {
        self.maximum_execution_time
    }

    /// Returns average execution time.
    pub fn average_execution_time(&self) -> Option<Duration> {
        if self.execution_count > 0 {
            Some(self.total_execution_time / self.execution_count)
        } else {
            None
        }
    }

    /// Updates this statistics with new execution data.
    pub(crate) fn update(&mut self, execution_data: ExecutionData) {
        self.wake_count += 1;

        if execution_data.was_executed() {
            self.execution_count += 1;

            let execution_time = execution_data
                .execution_duration()
                .expect("No execution time set for executed tasklet");

            self.minimum_execution_time = Some(match self.minimum_execution_time {
                Some(time) => core::cmp::min(time, execution_time),
                None => execution_time,
            });

            self.maximum_execution_time = Some(match self.maximum_execution_time {
                Some(time) => core::cmp::max(time, execution_time),
                None => execution_time,
            });

            self.total_execution_time += execution_time;
        }
    }
}

impl fmt::Display for ExecutionStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "Tasklet #{} statistics", self.tasklet_id())?;
        writeln!(f, "Wake count: {}", self.wake_count())?;
        writeln!(f, "Execution count: {}", self.execution_count())?;
        if let Some(time) = self.minimum_execution_time() {
            writeln!(f, "Minimum execution time: {}", time)?;
        }
        if let Some(time) = self.maximum_execution_time() {
            writeln!(f, "Maximum execution time: {}", time)?;
        }
        if let Some(time) = self.average_execution_time() {
            writeln!(f, "Average execution time: {}", time)?;
        }

        Ok(())
    }
}
