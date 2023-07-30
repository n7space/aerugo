//! Unit of computation in the system.
//!
//! Tasklet is a fine-grained units of computation, that execute a processing step in a finite
//! amount of time.
//!
//! Tasklet should be thought of as a small building block, which processes a given type of data,
//! one element at the time.
//! Instead
//! one tasklet is expected to perform one specific operation over received data. A bunch of
//! tasklets can create

mod tasklet_config;
mod tasklet_handle;
mod tasklet_ptr;
mod tasklet_status;
mod tasklet_storage;
mod tasklet_vtable;

pub(crate) use self::tasklet_ptr::TaskletPtr;
pub(crate) use self::tasklet_status::TaskletStatus;
pub(crate) use self::tasklet_vtable::{tasklet_vtable, TaskletVTable};

pub use self::tasklet_config::TaskletConfig;
pub use self::tasklet_handle::TaskletHandle;
pub use self::tasklet_storage::TaskletStorage;

use core::cell::OnceCell;

use crate::aerugo::error::InitError;
use crate::arch::Mutex;
use crate::boolean_condition::BooleanConditionSet;
use crate::data_provider::DataProvider;
use crate::data_receiver::DataReceiver;
use crate::internal_cell::InternalCell;
use crate::time::TimerInstantU64;

/// Type of function that is executed by the tasklet in its step.
pub(crate) type StepFn<T, C> = fn(T, &mut C);

/// Tasklet unique ID.
pub struct TaskletId(u32);

/// Tasklet structure.
///
/// # Generic Parameters
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
/// * `COND_COUNT` - Number of conditions.
#[repr(C)]
pub(crate) struct Tasklet<T: 'static, C: 'static, const COND_COUNT: usize> {
    /// Tasklet name.
    name: &'static str,
    /// Tasklet priority.
    priority: u8,
    /// Tasklet status.
    status: Mutex<TaskletStatus>,
    /// Last execution time.
    last_execution_time: Mutex<TimerInstantU64<1_000_000>>,
    /// Step function.
    step_fn: StepFn<T, C>,
    /// Context data.
    context: InternalCell<&'static mut C>,
    /// Source of the data.
    data_provider: OnceCell<&'static dyn DataProvider<T>>,
    /// Condition set.
    _condition_set: OnceCell<&'static BooleanConditionSet<COND_COUNT>>,
}

impl<T, C, const COND_COUNT: usize> Tasklet<T, C, COND_COUNT> {
    /// Creates new `Tasklet`.
    pub(crate) fn new(
        config: TaskletConfig,
        step_fn: StepFn<T, C>,
        context: &'static mut C,
    ) -> Self {
        Tasklet {
            name: config.name,
            priority: config.priority,
            status: Mutex::new(TaskletStatus::Sleeping),
            last_execution_time: Mutex::new(TimerInstantU64::<1_000_000>::from_ticks(0)),
            step_fn,
            context: InternalCell::new(context),
            data_provider: OnceCell::new(),
            _condition_set: OnceCell::new(),
        }
    }

    /// Returns task name.
    pub(crate) fn get_name(&self) -> &'static str {
        self.name
    }

    /// Returns task priority.
    pub(crate) fn get_priority(&self) -> u8 {
        self.priority
    }

    /// Returns task status.
    pub(crate) fn get_status(&self) -> TaskletStatus {
        self.status.lock(|s| *s)
    }

    /// Sets task status.
    ///
    /// # Parameters
    /// * `status` - New task status.
    pub(crate) fn set_status(&self, status: TaskletStatus) {
        self.status.lock(|s| *s = status)
    }

    /// Returns last execution time.
    pub(crate) fn get_last_execution_time(&self) -> TimerInstantU64<1_000_000> {
        self.last_execution_time.lock(|t| *t)
    }

    /// Sets last execution time.
    ///
    /// # Parameters
    /// * `time` - Last execution time.
    pub(crate) fn set_last_execution_time(&self, time: TimerInstantU64<1_000_000>) {
        self.last_execution_time.lock(|t| *t = time)
    }

    /// Checks if this tasklet is ready for execution.
    pub(crate) fn is_ready(&self) -> bool {
        match self.data_provider.get() {
            Some(dp) => dp.data_ready(),
            None => false,
        }
    }

    /// Executes task.
    pub(crate) fn execute(&self) {
        let value = match self.data_provider.get() {
            Some(dp) => dp.get_data(),
            None => return,
        };

        if let Some(val) = value {
            // SAFETY: This is safe, because this field is only accessed here, and given tasklet can
            // be executed only once at a given time.
            let context = unsafe { self.context.as_mut_ref() };
            (self.step_fn)(val, context)
        }
    }

    /// Creates pointer to this tasklet.
    pub(crate) fn ptr(&'static self) -> TaskletPtr {
        TaskletPtr::new::<T, C, COND_COUNT>(self)
    }
}

impl<T, C, const COND_COUNT: usize> DataReceiver<T> for Tasklet<T, C, COND_COUNT> {
    unsafe fn subscribe(
        &self,
        data_provider: &'static dyn DataProvider<T>,
    ) -> Result<(), InitError> {
        match self.data_provider.set(data_provider) {
            Ok(_) => Ok(()),
            Err(_) => Err(InitError::TaskletAlreadySubscribed),
        }
    }
}

#[cfg(any(doc, test))]
mod tests;
