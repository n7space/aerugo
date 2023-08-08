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

use crate::api::InitError;
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
    /// Condition set.
    condition_set: &'static OnceCell<BooleanConditionSet<COND_COUNT>>,
    /// Source of the data.
    data_provider: OnceCell<&'static dyn DataProvider<T>>,
}

impl<T, C, const COND_COUNT: usize> Tasklet<T, C, COND_COUNT> {
    /// Creates new `Tasklet`.
    pub(crate) fn new(
        config: TaskletConfig,
        step_fn: StepFn<T, C>,
        context: &'static mut C,
        condition_set: &'static OnceCell<BooleanConditionSet<COND_COUNT>>,
    ) -> Self {
        Tasklet {
            name: config.name,
            priority: config.priority,
            status: Mutex::new(TaskletStatus::Sleeping),
            last_execution_time: Mutex::new(TimerInstantU64::<1_000_000>::from_ticks(0)),
            step_fn,
            context: InternalCell::new(context),
            condition_set,
            data_provider: OnceCell::new(),
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

    /// Sets this tasklet conditions.
    pub(crate) unsafe fn set_condition_set(
        &self,
        condition_set: BooleanConditionSet<COND_COUNT>,
    ) -> Result<(), InitError> {
        match self.condition_set.set(condition_set) {
            Ok(_) => Ok(()),
            Err(_) => Err(InitError::TaskletAlreadyConditioned),
        }
    }

    /// Checks if this tasklet has data waiting for processing.
    pub(crate) fn has_work(&self) -> bool {
        match self.data_provider.get() {
            Some(data_provider) => data_provider.data_ready(),
            None => false,
        }
    }

    /// Check if this tasklet is active.
    ///
    /// Tasklet is not active if it's condition evaluates to `false`.
    pub(crate) fn is_active(&self) -> bool {
        match self.condition_set.get() {
            Some(condition_set) => condition_set.evaluate(),
            None => true,
        }
    }

    /// Executes task.
    ///
    /// # Return
    /// `true` if tasklet was executed, `false` otherwise
    pub(crate) fn execute(&self) -> bool {
        match self.data_provider.get() {
            Some(dp) => {
                let value = dp.get_data();

                if let Some(val) = value {
                    // SAFETY: This is safe, because this field is only accessed here, and given tasklet can
                    // be executed only once at a given time.
                    let context = unsafe { self.context.as_mut_ref() };
                    (self.step_fn)(val, context);

                    true
                } else {
                    false
                }
            }
            None => false,
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
