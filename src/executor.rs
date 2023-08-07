//! System scheduler.
//!
//! This module contains system executor, that schedules and runs tasklets.
//!
//! aerugo is build around an executor that run tasklets, which are fine-grained units of
//! computation. Executor is a cooperative scheduler, that doesn't support preemption.

use heapless::binary_heap::{BinaryHeap, Max};

use crate::aerugo::{error::RuntimeError, Aerugo, AERUGO};
use crate::api::{RuntimeApi, SystemApi};
use crate::arch::Mutex;
use crate::tasklet::{TaskletPtr, TaskletStatus};

/// Type for the tasklet execution queue
type TaskletQueue<const N: usize> = BinaryHeap<TaskletPtr, Max, N>;

/// System scheduler.
///
/// This shouldn't be created by hand by the user or anywhere else in the code.
/// It should be used as a singleton (crate::aerugo::EXECUTOR) and shouldn't be directly accessed
/// by any other part of the system. It's functionality shall be exposed for rest of the system
/// via system API in [Aerugo](crate::aerugo::Aerugo).
pub(crate) struct Executor {
    /// Tasklet queue.
    tasklet_queue: Mutex<TaskletQueue<{ Aerugo::TASKLET_COUNT }>>,
}

impl Executor {
    /// Creates new executor instance.
    ///
    /// # Safety
    /// This shouldn't be called more than once.
    pub(crate) const fn new() -> Self {
        Executor {
            tasklet_queue: Mutex::new(BinaryHeap::new()),
        }
    }

    /// Starts tasklet scheduling.
    pub(crate) fn run_scheduler(&'static self) -> ! {
        loop {
            self.execute_next_tasklet()
                .expect("Failure in tasklet execution");

            AERUGO.update();
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
    /// `RuntimeError` in case of an error, `Ok(bool)` otherwise indicating if tasklet was scheduled.
    #[allow(dead_code)]
    pub(crate) fn schedule_tasklet(
        &'static self,
        tasklet: &TaskletPtr,
    ) -> Result<bool, RuntimeError> {
        let tasklet_status = tasklet.get_status();

        if tasklet_status == TaskletStatus::Sleeping && tasklet.is_active() {
            self.add_tasklet_to_queue(tasklet.clone())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Executes the next tasklet from the queue.
    ///
    /// This sets `Waiting` status on the tasklet and then executes it. If there are more work to
    /// do (ex. there are still data in the queue) tasklet will be rescheduled, otherwise it will
    /// be put to sleep.
    ///
    /// # Returns
    /// `bool` indicating if tasklet was executed, `RuntimeError` otherwise.
    fn execute_next_tasklet(&'static self) -> Result<bool, RuntimeError> {
        if let Some(tasklet) = self.get_tasklet_for_execution() {
            if !tasklet.is_active() {
                tasklet.set_status(TaskletStatus::Sleeping);
                return Ok(false);
            }

            tasklet.set_status(TaskletStatus::Working);

            let executed = tasklet.execute();
            if executed {
                tasklet.set_last_execution_time(AERUGO.get_system_time());
            }

            self.try_reschedule_tasklet(tasklet)?;

            Ok(executed)
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
    /// `()` if successful, `RuntimeError` otherwise.
    fn try_reschedule_tasklet(&'static self, tasklet: TaskletPtr) -> Result<(), RuntimeError> {
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
    /// `()` if successful, `RuntimeError` otherwise.
    fn add_tasklet_to_queue(&'static self, tasklet: TaskletPtr) -> Result<(), RuntimeError> {
        self.tasklet_queue.lock(|q| {
            tasklet.set_status(TaskletStatus::Waiting);

            match q.push(tasklet) {
                Ok(_) => Ok(()),
                Err(_) => Err(RuntimeError::ExecutorTaskletQueueFull),
            }
        })
    }

    /// Returns next tasklet that is due for execution, or `None` if the execution queue is empty.
    fn get_tasklet_for_execution(&'static self) -> Option<TaskletPtr> {
        self.tasklet_queue.lock(|q| q.pop())
    }
}
