//! Unit of computation in the system.
//!
//! Tasklet is a fine-grained units of computation, that execute a processing step in a finite
//! amount of time.
//!
//! Tasklet should be thought of as a small building block, which processes a given type of data,
//! one element at the time.
mod tasklet_config;
mod tasklet_handle;
mod tasklet_ptr;
mod tasklet_storage;
mod tasklet_vtable;

pub(crate) use self::tasklet_ptr::TaskletPtr;
pub(crate) use self::tasklet_vtable::{tasklet_vtable, TaskletVTable};

pub use self::tasklet_config::TaskletConfig;
pub use self::tasklet_handle::TaskletHandle;
pub use self::tasklet_storage::TaskletStorage;

use core::marker::PhantomData;

use crate::aerugo::error::InitError;
use crate::arch::Mutex;
use crate::data_provider::DataProvider;
use crate::data_receiver::DataReceiver;
use crate::internal_cell::InternalCell;
use crate::task::{Task, TaskStatus};
use crate::time::TimerInstantU64;

/// Type of function that is executed by the tasklet in its step.
pub(crate) type StepFn<T> = fn(T);

/// Tasklet structure.
///
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
pub(crate) struct Tasklet<T: 'static, C> {
    /// Tasklet name.
    name: &'static str,
    /// Tasklet status.
    status: Mutex<TaskStatus>,
    /// Last execution time.
    last_execution_time: Mutex<TimerInstantU64<1_000_000>>,
    /// Step function.
    step_fn: StepFn<T>,
    /// Source of the data.
    _data_provider: InternalCell<Option<&'static dyn DataProvider<T>>>,
    /// Marker for tasklet context data type.
    _context_type_marker: PhantomData<C>,
}

impl<T, C> Tasklet<T, C> {
    /// Creates new `Tasklet`.
    pub(crate) fn new(config: TaskletConfig, step_fn: StepFn<T>) -> Self {
        Tasklet {
            name: config.name,
            status: Mutex::new(TaskStatus::Sleeping),
            last_execution_time: Mutex::new(TimerInstantU64::<1_000_000>::from_ticks(0)),
            step_fn,
            _data_provider: InternalCell::new(None),
            _context_type_marker: PhantomData,
        }
    }
}

impl<T: Default, C> Task for Tasklet<T, C> {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_status(&self) -> TaskStatus {
        self.status.lock(|s| *s)
    }

    fn set_status(&self, status: TaskStatus) {
        self.status.lock(|s| *s = status)
    }

    fn get_last_execution_time(&self) -> TimerInstantU64<1_000_000> {
        self.last_execution_time.lock(|t| *t)
    }

    fn set_last_execution_time(&self, time: TimerInstantU64<1_000_000>) {
        self.last_execution_time.lock(|t| *t = time)
    }

    fn has_work(&self) -> bool {
        // TODO: Stub until we have working data providers.
        true
    }

    fn execute(&self) {
        // TODO: Stub until we have working data providers.
        let value: T = T::default();

        (self.step_fn)(value)
    }
}

impl<T, C> DataReceiver<T> for Tasklet<T, C> {
    fn subscribe(&self, _data_provider: &'static dyn DataProvider<T>) -> Result<(), InitError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default() {
        let tasklet = Tasklet::<u8, ()>::new(TaskletConfig::default(), |_| {});

        assert_eq!(tasklet.get_name(), "MISSING_TASKLET_NAME");
        assert_eq!(tasklet.get_status(), TaskStatus::Sleeping);
        assert_eq!(tasklet.get_last_execution_time().ticks(), 0);
    }

    #[test]
    fn create_from_config() {
        let name = "TaskName";

        let config = TaskletConfig { name };
        let tasklet = Tasklet::<u8, ()>::new(config, |_| {});

        assert_eq!(tasklet.get_name(), name);
        assert_eq!(tasklet.get_status(), TaskStatus::Sleeping);
        assert_eq!(tasklet.get_last_execution_time().ticks(), 0);
    }

    #[test]
    fn get_set_status() {
        let tasklet = Tasklet::<u8, ()>::new(TaskletConfig::default(), |_| {});

        assert_eq!(tasklet.get_status(), TaskStatus::Sleeping);
        tasklet.set_status(TaskStatus::Waiting);
        assert_eq!(tasklet.get_status(), TaskStatus::Waiting);
    }

    #[test]
    fn get_set_last_execution_time() {
        let tasklet = Tasklet::<u8, ()>::new(TaskletConfig::default(), |_| {});

        assert_eq!(tasklet.get_last_execution_time().ticks(), 0);
        tasklet.set_last_execution_time(TimerInstantU64::<1_000_000>::from_ticks(42));
        assert_eq!(tasklet.get_last_execution_time().ticks(), 42);
    }
}
