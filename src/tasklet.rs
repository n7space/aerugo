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
mod tasklet_storage;
mod tasklet_vtable;

pub(crate) use self::tasklet_ptr::TaskletPtr;
pub(crate) use self::tasklet_vtable::{tasklet_vtable, TaskletVTable};

pub use self::tasklet_config::TaskletConfig;
pub use self::tasklet_handle::TaskletHandle;
pub use self::tasklet_storage::TaskletStorage;

use core::cell::OnceCell;

use crate::aerugo::error::InitError;
use crate::arch::Mutex;
use crate::data_provider::DataProvider;
use crate::data_receiver::DataReceiver;
use crate::internal_cell::InternalCell;
use crate::task::{Task, TaskStatus};
use crate::time::TimerInstantU64;

/// Type of function that is executed by the tasklet in its step.
pub(crate) type StepFn<T, C> = fn(T, &mut C);

/// Tasklet structure.
///
/// # Generic Parameters
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
#[repr(C)]
pub(crate) struct Tasklet<T: 'static, C: 'static> {
    /// Tasklet name.
    name: &'static str,
    /// Tasklet status.
    status: Mutex<TaskStatus>,
    /// Last execution time.
    last_execution_time: Mutex<TimerInstantU64<1_000_000>>,
    /// Step function.
    step_fn: StepFn<T, C>,
    /// Context data.
    context: InternalCell<&'static mut C>,
    /// Source of the data.
    data_provider: OnceCell<&'static dyn DataProvider<T>>,
}

impl<T, C> Tasklet<T, C> {
    /// Creates new `Tasklet`.
    pub(crate) fn new(
        config: TaskletConfig,
        step_fn: StepFn<T, C>,
        context: &'static mut C,
    ) -> Self {
        Tasklet {
            name: config.name,
            status: Mutex::new(TaskStatus::Sleeping),
            last_execution_time: Mutex::new(TimerInstantU64::<1_000_000>::from_ticks(0)),
            step_fn,
            context: InternalCell::new(context),
            data_provider: OnceCell::new(),
        }
    }

    /// Creates pointer to this tasklet.
    pub(crate) fn ptr(&'static self) -> TaskletPtr {
        TaskletPtr::new::<T, C>(self)
    }
}

impl<T, C> Task for Tasklet<T, C> {
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
        match self.data_provider.get() {
            Some(dp) => dp.data_ready(),
            None => false,
        }
    }

    fn execute(&self) {
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
}

impl<T, C> DataReceiver<T> for Tasklet<T, C> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_size() {
        type TaskletStub = Tasklet<(), ()>;
        let stub_size = core::mem::size_of::<TaskletStub>();

        struct NoCtx;
        type TaskletNoCtx = Tasklet<u8, NoCtx>;
        let noctx_size = core::mem::size_of::<TaskletNoCtx>();

        struct SmallCtx {
            _a: u16,
        }
        type TaskletSmallCtx = Tasklet<u16, SmallCtx>;
        let smallctx_size = core::mem::size_of::<TaskletSmallCtx>();

        struct BigCtx {
            _a: u64,
            _b: f64,
            _c: u16,
        }
        type TaskletBigCtx = Tasklet<u32, BigCtx>;
        let bigctx_size = core::mem::size_of::<TaskletBigCtx>();

        assert_eq!(noctx_size, stub_size);
        assert_eq!(smallctx_size, stub_size);
        assert_eq!(bigctx_size, stub_size);
    }
}
