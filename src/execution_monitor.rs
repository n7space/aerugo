//! Tasklet execution monitoring.

mod execution_data;
mod execution_stats;

pub use self::execution_stats::ExecutionStats;

pub(crate) use self::execution_data::ExecutionData;

use core::cell::{OnceCell, UnsafeCell};

use heapless::Vec;

use crate::aerugo::Aerugo;
use crate::error::SystemError;
use crate::event::Event;
use crate::tasklet::TaskletId;
use crate::time::Duration;

/// Monitor for tasklet execution.
///
/// Stores execution statistics for tasklets in the system.
pub(crate) struct ExecutionMonitor {
    /// Tasklet execution statistics .
    execution_stats: UnsafeCell<Vec<ExecutionStats, { Aerugo::TASKLET_COUNT }>>,
    /// Tasklet execution time exceeded maximum event.
    time_exceeded_event: OnceCell<(&'static Event, Duration)>,
}

/// This is safe on single-threaded platform when `ExecutionMonitor` is not available from the IRQ
/// context.
///
/// In this implementation `ExecutionMonitor` is used only by `Aerugo` in
/// [run](crate::aerugo::Aerugo::run) and via `RuntimeApi` in
/// [get_execution_statistics](crate::api::RuntimeApi::get_execution_statistics), both of which are
/// not accessible from the IRQ context.
unsafe impl Sync for ExecutionMonitor {}

impl ExecutionMonitor {
    /// Creates new ExecutionMonitor instance.
    pub(crate) const fn new() -> Self {
        Self {
            execution_stats: UnsafeCell::new(Vec::new()),
            time_exceeded_event: OnceCell::new(),
        }
    }

    /// Sets an event that should be emitted when tasklet execution time exceeds maximum.
    ///
    /// # Parameter
    /// * `event` - Event to be emitted
    pub(crate) unsafe fn set_time_exceeded_event(
        &'static self,
        event: &'static Event,
        time: Duration,
    ) -> Result<(), SystemError> {
        match self.time_exceeded_event.set((event, time)) {
            Ok(_) => Ok(()),
            Err(_) => Err(SystemError::TimeExceededEventAlreadySet),
        }
    }

    /// Returns execution statistics for tasklet of given ID.
    ///
    /// # Parameters
    /// * `tasklet_id` - Tasklet ID.
    ///
    /// # Return
    /// `ExecutionStats` if that tasklet was is present in the system and was woken at least once.
    /// `None` otherwise.
    ///
    /// # Safety
    /// This is marked as unsafe because it accesses the execution statistics list. This is
    /// considered safe on single-threaded platform if `ExecutionMonitor` is not available
    /// from the IRQ context.
    pub(crate) unsafe fn get_stats(
        &'static self,
        tasklet_id: &TaskletId,
    ) -> Option<ExecutionStats> {
        // This is safe, because system is single-threaded and interrupt doesn't have access to the `ExecutionMonitor`
        let execution_stats = &(*self.execution_stats.get());

        execution_stats
            .iter()
            .find(|stats| stats.tasklet_id() == tasklet_id)
            .copied()
    }

    /// Updates execution statistics with new data.
    ///
    /// # Parameters
    /// * `execution_data` - Data from the latest execution.
    ///
    /// # Safety
    /// This is marked as unsafe because it accesses the execution statistics list. This is
    /// considered safe on single-threaded platform if `ExecutionMonitor` is not available
    /// from the IRQ context.
    pub(crate) unsafe fn update(&'static self, execution_data: ExecutionData) {
        if let Some(event) = self.time_exceeded_event.get() {
            if let Some(execution_duration) = execution_data.execution_duration() {
                if execution_duration > event.1 {
                    event.0.emit();
                }
            }
        }

        let tasklet_id = execution_data.tasklet_id();

        let mut execution_stats = self.take_or_create_stats(tasklet_id);
        execution_stats.update(execution_data);

        self.add_stats(execution_stats)
            .expect("Failed to update execution stats");
    }

    /// Adds execution statistics to the list.
    ///
    /// # Parameters
    /// * `stats` - Execution stats.
    ///
    /// # Return
    /// `()` if successful, `SystemError` otherwise.
    ///
    /// # Safety
    /// This is marked as unsafe because it accesses the execution statistics list. This is
    /// considered safe on single-threaded platform if `ExecutionMonitor` is not available
    /// from the IRQ context.
    unsafe fn add_stats(&'static self, stats: ExecutionStats) -> Result<(), SystemError> {
        let execution_stats = &mut (*self.execution_stats.get());

        match execution_stats.push(stats) {
            Ok(_) => Ok(()),
            Err(_) => Err(SystemError::ExecutionStatsListFull),
        }
    }

    /// Takes executions statistics for tasklet of given ID or creates new if not present.
    ///
    /// # Parameters
    /// * `tasklet_id` - Tasklet ID.
    ///
    /// # Return
    /// `ExecutionStats` for tasklet of given ID.
    ///
    /// # Safety
    /// This is marked as unsafe because it accesses the execution statistics list. This is
    /// considered safe on single-threaded platform if `ExecutionMonitor` is not available
    /// from the IRQ context.
    unsafe fn take_or_create_stats(&'static self, tasklet_id: &TaskletId) -> ExecutionStats {
        let execution_stats = &mut (*self.execution_stats.get());

        match execution_stats
            .iter()
            .position(|stats| stats.tasklet_id() == tasklet_id)
        {
            Some(position) => execution_stats.remove(position),
            None => ExecutionStats::new(*tasklet_id),
        }
    }
}
