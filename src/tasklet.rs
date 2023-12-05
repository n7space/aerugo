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
mod tasklet_id;
mod tasklet_ptr;
mod tasklet_status;
mod tasklet_storage;
mod tasklet_vtable;

pub(crate) use self::tasklet_ptr::TaskletPtr;
pub(crate) use self::tasklet_status::TaskletStatus;
pub(crate) use self::tasklet_vtable::{tasklet_vtable, TaskletVTable};

pub use self::tasklet_config::TaskletConfig;
pub use self::tasklet_handle::TaskletHandle;
pub use self::tasklet_id::TaskletId;
pub use self::tasklet_storage::TaskletStorage;

use core::cell::{OnceCell, UnsafeCell};

use crate::api::RuntimeApi;
use crate::boolean_condition::BooleanConditionSet;
use crate::data_provider::DataProvider;
use crate::error::SystemError;
use crate::mutex::Mutex;
use crate::time::Instant;

/// Type of function that is executed by the tasklet in its step.
pub(crate) type StepFn<T, C> = fn(T, &mut C, &'static dyn RuntimeApi);

/// Tasklet structure.
///
/// # Generic Parameters
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
/// * `COND_COUNT` - Number of conditions.
#[repr(C)]
pub(crate) struct Tasklet<T: 'static, C: 'static, const COND_COUNT: usize> {
    /// Tasklet ID.
    id: TaskletId,
    /// Tasklet name.
    name: &'static str,
    /// Tasklet priority.
    priority: u8,
    /// Tasklet status.
    status: Mutex<TaskletStatus>,
    /// Last execution time.
    last_execution_time: Mutex<Instant>,
    /// Step function.
    step_fn: StepFn<T, C>,
    /// Context data.
    context: UnsafeCell<&'static mut C>,
    /// Condition set.
    condition_set: &'static OnceCell<BooleanConditionSet<COND_COUNT>>,
    /// Source of the data.
    data_provider: OnceCell<&'static dyn DataProvider<T>>,
    /// Runtime API.
    runtime_api: &'static dyn RuntimeApi,
}

/// It is safe assuming that Tasklet is not available from IRQ context before it's
/// created and that modifications cannot be interrupted.
///
/// Tasklet structure is hidden from the user. Functionalities are exposed to the user via
/// [TaskletHandle].
///
/// Tasklet is only created by `TaskletStorage` with [create_tasklet](crate::api::InitApi::create_tasklet)
/// which is not accessible from the IRQ context.
///
/// Context is never leaked outside of the tasklet. It can be only accessible by the user in the
/// tasklet functions which is executed by the system, one at a given time. There are no way to
/// access tasklet context from the IRQ context.
///
/// Initializations and modifications mustn't be interrupted. Tasklet is only accessible with an
/// unmutable reference. All modifications are implemented with interior mutability using [Mutex]
/// which ensures that those modifications cannot be interrupted.
unsafe impl<T, C, const COND_COUNT: usize> Sync for Tasklet<T, C, COND_COUNT> {}

impl<T, C, const COND_COUNT: usize> Tasklet<T, C, COND_COUNT> {
    /// Creates new `Tasklet`.
    pub(crate) const fn new(
        id: TaskletId,
        config: TaskletConfig,
        step_fn: StepFn<T, C>,
        context: &'static mut C,
        condition_set: &'static OnceCell<BooleanConditionSet<COND_COUNT>>,
        runtime_api: &'static dyn RuntimeApi,
    ) -> Self {
        Tasklet {
            id,
            name: config.name,
            priority: config.priority,
            status: Mutex::new(TaskletStatus::Sleeping),
            last_execution_time: Mutex::new(Instant::from_ticks(0)),
            step_fn,
            context: UnsafeCell::new(context),
            condition_set,
            data_provider: OnceCell::new(),
            runtime_api,
        }
    }

    /// Returns task ID.
    pub(crate) fn get_id(&self) -> TaskletId {
        self.id
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
    pub(crate) fn get_last_execution_time(&self) -> Instant {
        self.last_execution_time.lock(|t| *t)
    }

    /// Sets last execution time.
    ///
    /// # Parameters
    /// * `time` - Last execution time.
    pub(crate) fn set_last_execution_time(&self, time: Instant) {
        self.last_execution_time.lock(|t| *t = time)
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

    /// Checks if this tasklet has data waiting for processing.
    pub(crate) fn has_work(&self) -> bool {
        match self.data_provider.get() {
            Some(data_provider) => data_provider.data_waiting(),
            None => false,
        }
    }

    /// Sets this tasklet conditions.
    ///
    /// # Return
    /// `SystemError` if tasklet already has condition set, `()` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the condition set.
    /// This is safe if it's executed in a critical section during system initialization
    /// (before scheduler is started).
    /// Accessing tasklet from IRQ context during setting is undefined behaviour.
    pub(crate) unsafe fn set_condition_set(
        &self,
        condition_set: BooleanConditionSet<COND_COUNT>,
    ) -> Result<(), SystemError> {
        match self.condition_set.set(condition_set) {
            Ok(_) => Ok(()),
            Err(_) => Err(SystemError::TaskletAlreadyHasConditionSet(self.get_name())),
        }
    }

    /// Checks if tasklet is subscribed to any data provider.
    pub(crate) fn is_subscribed(&self) -> bool {
        self.data_provider.get().is_some()
    }

    /// Subscribes itself to the given data provider.
    ///
    /// # Parameters
    /// * `data_provider` - Data provider.
    ///
    /// # Return
    /// `SystemError` if tasklet already has data provider, `()` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the data provider.
    /// This is safe if it's executed in a critical section during system initialization
    /// (before scheduler is started).
    /// Accessing tasklet from IRQ context during subscribing is undefined behaviour.
    pub(crate) unsafe fn subscribe(
        &self,
        data_provider: &'static dyn DataProvider<T>,
    ) -> Result<(), SystemError> {
        match self.data_provider.set(data_provider) {
            Ok(_) => Ok(()),
            Err(_) => Err(SystemError::TaskletAlreadySubscribed(self.get_name())),
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
                    let context: &mut C = unsafe { *self.context.get() };
                    (self.step_fn)(val, context, self.runtime_api);

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

#[cfg(any(doc, test))]
mod tests {
    use super::*;

    #[test]
    fn const_size() {
        type TaskletStub = Tasklet<(), (), 0>;
        let stub_size = core::mem::size_of::<TaskletStub>();

        struct NoCtx;
        type TaskletNoCtx = Tasklet<u8, NoCtx, 1>;
        let noctx_size = core::mem::size_of::<TaskletNoCtx>();

        struct SmallCtx {
            _a: u16,
        }
        type TaskletSmallCtx = Tasklet<u16, SmallCtx, 2>;
        let smallctx_size = core::mem::size_of::<TaskletSmallCtx>();

        struct BigCtx {
            _a: u64,
            _b: f64,
            _c: u16,
        }
        type TaskletBigCtx = Tasklet<u32, BigCtx, 5>;
        let bigctx_size = core::mem::size_of::<TaskletBigCtx>();

        assert_eq!(noctx_size, stub_size);
        assert_eq!(smallctx_size, stub_size);
        assert_eq!(bigctx_size, stub_size);
    }
}
