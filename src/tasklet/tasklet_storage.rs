//! Static storage for [tasklet](crate::tasklet::Tasklet)
//!
//! This module contains a tasklet storage, which is a statically allocated memory that will
//! store tasklet structure for the duration of the system life.

use super::Tasklet;

use core::cell::OnceCell;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use heapless::Vec;

use crate::aerugo::InitError;
use crate::boolean_condition::BooleanConditionSet;
use crate::internal_cell::InternalCell;
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
    initialized: InternalCell<bool>,
    /// Buffer for the tasklet structure.
    tasklet_buffer: InternalCell<TaskletBuffer>,
    /// Storage for the context data.
    tasklet_context: InternalCell<MaybeUninit<C>>,
    /// Storage for the tasklet conditions.
    tasklet_conditions: InternalCell<OnceCell<BooleanConditionSet<COND_COUNT>>>,
    /// Marker for the tasklet data type.
    _data_type_marker: PhantomData<T>,
}

impl<T: 'static, C: 'static, const COND_COUNT: usize> TaskletStorage<T, C, COND_COUNT> {
    /// Creates new storage.
    pub const fn new() -> Self {
        TaskletStorage {
            initialized: InternalCell::new(false),
            tasklet_buffer: InternalCell::new(TaskletBuffer::new()),
            tasklet_context: InternalCell::new(MaybeUninit::uninit()),
            tasklet_conditions: InternalCell::new(OnceCell::new()),
            _data_type_marker: PhantomData,
        }
    }

    /// Returns initialization status of this storage.
    pub fn is_initialized(&'static self) -> bool {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // the `init` function.
        unsafe { *self.initialized.as_ref() }
    }

    /// Creates new handle to a tasklet allocated in this storage.
    ///
    /// # Return
    /// `handle` if this storage has been initialized.
    pub fn create_handle(&'static self) -> Option<TaskletHandle<T, C, COND_COUNT>> {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // the `init` function.
        match unsafe { *self.initialized.as_ref() } {
            true => {
                // SAFETY: This is safe because storage has been initialized.
                let tasklet = unsafe { self.tasklet() };
                Some(TaskletHandle::new(tasklet))
            }
            false => None,
        }
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
    ) -> Result<(), InitError> {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // this function.
        if unsafe { *self.initialized.as_ref() } {
            return Err(InitError::StorageAlreadyInitialized);
        }

        let tasklet_context = self.tasklet_context.as_mut_ref();
        *tasklet_context = MaybeUninit::new(context);

        // SAFETY: This is safe, because `tasklet_context` was just initialized.
        let tasklet = Tasklet::<T, C, COND_COUNT>::new(
            config,
            step_fn,
            unsafe { tasklet_context.assume_init_mut() },
            self.tasklet_conditions.as_mut_ref(),
        );

        // This is safe, because `tasklet_buffer` doesn't contain any value yet, and it's size is
        // guaranteed to be large enough to store tasklet structure.
        let tasklet_buffer =
            self.tasklet_buffer.as_mut_ref().as_mut_ptr() as *mut Tasklet<T, C, COND_COUNT>;
        unsafe {
            core::ptr::write(tasklet_buffer, tasklet);
        }

        *self.initialized.as_mut_ref() = true;

        Ok(())
    }

    /// Returns a reference to the stored Tasklet structure.
    ///
    /// # Safety
    /// This is safe to call only when this storage has been initialized.
    #[inline(always)]
    unsafe fn tasklet(&'static self) -> &'static Tasklet<T, C, COND_COUNT> {
        &*(self.tasklet_buffer.as_ref().as_ptr() as *const Tasklet<T, C, COND_COUNT>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        static STORAGE: TaskletStorage<u8, (), 0> = TaskletStorage::new();

        assert!(!STORAGE.is_initialized());
    }

    #[test]
    fn initialize() {
        static STORAGE: TaskletStorage<u8, (), 0> = TaskletStorage::new();

        let init_result = unsafe { STORAGE.init(TaskletConfig::default(), |_, _| {}, ()) };
        assert!(init_result.is_ok());
        assert!(STORAGE.is_initialized());
    }

    #[test]
    fn fail_double_initialization() {
        static STORAGE: TaskletStorage<u8, (), 0> = TaskletStorage::new();

        let mut init_result = unsafe { STORAGE.init(TaskletConfig::default(), |_, _| {}, ()) };
        assert!(init_result.is_ok());

        init_result = unsafe { STORAGE.init(TaskletConfig::default(), |_, _| {}, ()) };
        assert!(init_result.is_err());
        assert_eq!(
            init_result.err().unwrap(),
            InitError::StorageAlreadyInitialized
        );
    }

    #[test]
    fn create_handle() {
        static STORAGE: TaskletStorage<u8, (), 0> = TaskletStorage::new();

        let _ = unsafe { STORAGE.init(TaskletConfig::default(), |_, _| {}, ()) };

        let handle = STORAGE.create_handle();
        assert!(handle.is_some());
    }

    #[test]
    fn fail_create_handle_uninitialized() {
        static STORAGE: TaskletStorage<u8, (), 0> = TaskletStorage::new();

        let handle = STORAGE.create_handle();
        assert!(handle.is_none());
    }
}
