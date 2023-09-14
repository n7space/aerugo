//! Static storage for [tasklet](crate::tasklet::Tasklet)
//!
//! This module contains a tasklet storage, which is a statically allocated memory that will
//! store tasklet structure for the duration of the system life.

use super::Tasklet;

use core::cell::{OnceCell, UnsafeCell};
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use heapless::Vec;

use crate::api::RuntimeApi;
use crate::boolean_condition::BooleanConditionSet;
use crate::error::SystemError;
use crate::tasklet::{StepFn, TaskletConfig, TaskletHandle};

/// Type of the tasklet buffer storage.
pub(crate) type TaskletBuffer = Vec<u8, { core::mem::size_of::<Tasklet<(), (), 0>>() }>;

/// Structure containing memory for Tasklet creation.
///
/// As this system cannot use dynamic memory allocation, all structures have to be allocated
/// statically. Per good practices user is separated from the actual implementation and instead
/// only has to provide a static memory (via this structure) where the Tasklet will be allocated.
///
/// Storage shall be a static variable and shall be initialized only once, before the system is started.
///
/// # Generic Parameters
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
/// * `COND_COUNT` - Number of tasklet conditions.
pub struct TaskletStorage<T, C, const COND_COUNT: usize> {
    /// Marks whether this storage is initialized.
    initialized: OnceCell<()>,
    /// Buffer for the tasklet structure.
    tasklet_buffer: OnceCell<TaskletBuffer>,
    /// Storage for the context data.
    tasklet_context: UnsafeCell<MaybeUninit<C>>,
    /// Storage for the tasklet conditions.
    tasklet_conditions: OnceCell<BooleanConditionSet<COND_COUNT>>,
    /// Marker for the tasklet data type.
    _data_type_marker: PhantomData<T>,
}

impl<T: 'static, C: 'static, const COND_COUNT: usize> TaskletStorage<T, C, COND_COUNT> {
    /// Creates new storage.
    pub const fn new() -> Self {
        TaskletStorage {
            initialized: OnceCell::new(),
            tasklet_buffer: OnceCell::new(),
            tasklet_context: UnsafeCell::new(MaybeUninit::uninit()),
            tasklet_conditions: OnceCell::new(),
            _data_type_marker: PhantomData,
        }
    }

    /// Returns initialization status of this storage.
    pub fn is_initialized(&'static self) -> bool {
        self.initialized.get().is_some()
    }

    /// Creates new handle to a tasklet allocated in this storage.
    ///
    /// # Return
    /// `handle` if this storage has been initialized.
    pub fn create_handle(&'static self) -> Option<TaskletHandle<T, C, COND_COUNT>> {
        self.tasklet().map(TaskletHandle::new)
    }

    /// Initializes this storage.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the stored tasklet buffer. This is safe to call
    /// before the system initialization.
    pub(crate) unsafe fn init(
        &'static self,
        config: TaskletConfig,
        step_fn: StepFn<T, C>,
        context: C,
        runtime_api: &'static dyn RuntimeApi,
    ) -> Result<(), SystemError> {
        if self.initialized.get().is_some() {
            return Err(SystemError::StorageAlreadyInitialized);
        }

        // SAFETY: The tasklet created below is the only one with access to the context data, and
        // as such stores the mutable reference to the context data.
        let tasklet_context: &mut MaybeUninit<C> = &mut *self.tasklet_context.get();
        tasklet_context.write(context);

        let tasklet = Tasklet::<T, C, COND_COUNT>::new(
            config,
            step_fn,
            // SAFETY: This is safe, because `tasklet_context` was just initialized.
            unsafe { tasklet_context.assume_init_mut() },
            &self.tasklet_conditions,
            runtime_api,
        );

        // This is safe, because `tasklet_buffer` doesn't contain any value yet, and it's size is
        // guaranteed to be large enough to store tasklet structure.
        let tasklet_buffer = TaskletBuffer::new();
        unsafe {
            let tasklet_buffer_ptr = tasklet_buffer.as_ptr() as *mut Tasklet<T, C, COND_COUNT>;
            core::ptr::write(tasklet_buffer_ptr, tasklet);
        }

        match self.tasklet_buffer.set(tasklet_buffer) {
            Ok(_) => Ok(()),
            Err(_) => Err(SystemError::StorageBufferAlreadySet),
        }?;

        match self.initialized.set(()) {
            Ok(_) => Ok(()),
            Err(_) => Err(SystemError::StorageInitializedAlreadySet),
        }?;

        Ok(())
    }

    /// Returns a reference to the stored Tasklet structure.
    ///
    /// # Safety
    /// This is safe to call only when this storage has been initialized.
    #[inline(always)]
    fn tasklet(&'static self) -> Option<&'static Tasklet<T, C, COND_COUNT>> {
        match (self.initialized.get(), self.tasklet_buffer.get()) {
            // This is safe, because buffer is initialized
            (Some(_), Some(buffer)) => unsafe {
                Some(&*(buffer.as_ptr() as *const Tasklet<T, C, COND_COUNT>))
            },
            (_, _) => None,
        }
    }
}

unsafe impl<T: 'static, C: 'static, const COND_COUNT: usize> Sync
    for TaskletStorage<T, C, COND_COUNT>
{
}
