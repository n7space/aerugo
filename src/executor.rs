//! System scheduler.
//!
//! This module contains system executor, that schedules and runs tasklets.
//!
//! aerugo is build around an executor that run tasklets, which are fine-grained units of
//! computation. Executor is a cooperative scheduler, that doesn't support preemption.

use heapless::binary_heap::{BinaryHeap, Max};

use crate::aerugo::Aerugo;
use crate::error::SystemError;
use crate::mutex::Mutex;
use crate::tasklet::{TaskletPtr, TaskletStatus};
use crate::time_source::TimeSource;

/// Type for the tasklet execution queue
type TaskletQueue<const N: usize> = BinaryHeap<TaskletPtr, Max, N>;

/// System scheduler.
///
/// This shouldn't be created by hand by the user or anywhere else in the code.
/// It should be used as a singleton (crate::aerugo::EXECUTOR) and shouldn't be directly accessed
/// by any other part of the system. It's functionality shall be exposed for rest of the system
/// via system API in [Aerugo].
pub(crate) struct Executor {
    /// Tasklet queue.
    tasklet_queue: Mutex<TaskletQueue<{ Aerugo::TASKLET_COUNT }>>,
    /// Time source.
    time_source: &'static TimeSource,
}

impl Executor {
    /// Creates new executor instance.
    ///
    /// # Safety
    /// This shouldn't be called more than once.
    pub(crate) const fn new(time_source: &'static TimeSource) -> Self {
        Executor {
            tasklet_queue: Mutex::new(BinaryHeap::new()),
            time_source,
        }
    }

    /// Executes the next tasklet from the queue.
    ///
    /// This sets `Waiting` status on the tasklet and then executes it. If there are more work to
    /// do (ex. there are still data in the queue) tasklet will be rescheduled, otherwise it will
    /// be put to sleep.
    ///
    /// # Returns
    /// Value indicating if tasklet was executed, `SystemError` otherwise.
    pub(crate) fn execute_next_tasklet(&'static self) -> Result<bool, SystemError> {
        if let Some(tasklet) = self.get_tasklet_for_execution() {
            if !tasklet.is_active() {
                tasklet.set_status(TaskletStatus::Sleeping);
                return Ok(false);
            }

            tasklet.set_status(TaskletStatus::Working);

            let executed = tasklet.execute();
            if executed {
                let system_time = self.time_source.system_time();
                tasklet.set_last_execution_time(system_time);
            }

            self.try_reschedule_tasklet(tasklet)?;

            Ok(executed)
        } else {
            Ok(false)
        }
    }

    /// Schedules given task for execution.
    ///
    /// If given task is not already waiting for execution it is put to the execution queue.
    ///
    /// # Parameters
    /// * `tasklet` - Tasklet to schedule.
    ///
    /// # Return
    /// Value indicating if tasklet was scheduled if successful, `SystemError` otherwise.
    #[allow(dead_code)]
    pub(crate) fn schedule_tasklet(
        &'static self,
        tasklet: &TaskletPtr,
    ) -> Result<bool, SystemError> {
        let tasklet_status = tasklet.get_status();

        if tasklet_status == TaskletStatus::Sleeping && tasklet.is_active() {
            self.add_tasklet_to_queue(tasklet.clone())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Schedules tasklet if there is more work to do, or sets it sleeping otherwise.
    ///
    /// # Parameters
    /// * `tasklet` - Tasklet to reschedule
    ///
    /// # Return
    /// `()` if successful, `SystemError` otherwise.
    fn try_reschedule_tasklet(&'static self, tasklet: TaskletPtr) -> Result<(), SystemError> {
        if tasklet.has_work() {
            self.add_tasklet_to_queue(tasklet)?;
        } else {
            tasklet.set_status(TaskletStatus::Sleeping);
        }

        Ok(())
    }

    /// Adds given tasklet to the execution queue.
    ///
    /// This marks tasklet as waiting.
    ///
    /// # Return
    /// `()` if successful, `SystemError` otherwise.
    fn add_tasklet_to_queue(&'static self, tasklet: TaskletPtr) -> Result<(), SystemError> {
        self.tasklet_queue.lock(|q| {
            tasklet.set_status(TaskletStatus::Waiting);

            match q.push(tasklet) {
                Ok(_) => Ok(()),
                Err(_) => Err(SystemError::ExecutorTaskletQueueFull),
            }
        })
    }

    /// Returns next tasklet that is due for execution, or `None` if the execution queue is empty.
    fn get_tasklet_for_execution(&'static self) -> Option<TaskletPtr> {
        self.tasklet_queue.lock(|q| q.pop())
    }
}

#[cfg(any(doc, test))]
mod tests {
    use super::*;

    use crate::boolean_condition::{BooleanConditionSet, BooleanConditionSetType};
    use crate::tasklet::{Tasklet, TaskletConfig};
    use crate::tests::{MockConditionSet, MockDataProvider, MockRuntimeApi};

    /// @SRS{ROS-FUN-RTOS-050}
    /// @SRS{ROS-FUN-RTOS-060}
    /// @SRS{ROS-FUN-RTOS-070}
    /// @SRS{ROS-FUN-RTOS-080}
    #[cfg_attr(not(doc), test)]
    #[allow(non_upper_case_globals)]
    fn req_tasklet_execution_state() {
        static mut mock_data_provider: MockDataProvider = MockDataProvider::new();

        static mock_condition_set: MockConditionSet<0> = MockConditionSet::new();
        let _ = mock_condition_set
            .storage
            .set(BooleanConditionSet::new(BooleanConditionSetType::And));

        static mock_runtime_api: MockRuntimeApi = MockRuntimeApi {};

        static mut tasklet_context: () = ();
        static mut tasklet_config: TaskletConfig = TaskletConfig {
            name: "TestTasklet",
            priority: 0,
        };
        static tasklet: Tasklet<(), (), 0> = Tasklet::new(
            unsafe { tasklet_config },
            |_, _, _| {},
            unsafe { &mut tasklet_context },
            &mock_condition_set.storage,
            &mock_runtime_api,
        );

        let subscribe_result = unsafe { tasklet.subscribe(&mock_data_provider) };
        assert!(subscribe_result.is_ok());

        static time_source: TimeSource = TimeSource::new();
        unsafe { time_source.set_system_start() };

        static executor: Executor = Executor::new(&time_source);

        // Unscheduled tasklet is `Sleeping`.
        assert_eq!(tasklet.get_status(), TaskletStatus::Sleeping);

        let schedule_result = executor.schedule_tasklet(&tasklet.ptr());
        assert!(schedule_result.is_ok());

        // Scheduled tasklet is `Waiting`.
        assert_eq!(tasklet.get_status(), TaskletStatus::Waiting);

        unsafe {
            mock_data_provider.set_data_waiting(true);
        }

        // Tasklet that is being executed is `Working`.
        let execution_result = executor.execute_next_tasklet();
        assert!(execution_result.is_ok());
        assert!(execution_result.unwrap());
    }
}
