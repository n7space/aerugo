//! Static storage for [tasklet](crate::tasklet::Tasklet)
//!
//! As this system cannot use dynamic memory allocation, all structures have to be allocated
//! statically. Per good practices user is separated from the actual implementation and instead
//! only has to provide a static memory (via this structure) where the Tasklet will be allocated.

use super::Tasklet;

use core::marker::PhantomData;
use heapless::Vec;

use crate::aerugo::InitError;
use crate::internal_cell::InternalCell;
use crate::tasklet::{StepFn, TaskletConfig, TaskletHandle, TaskletPtr};

/// Type of the tasklet buffer storage.
pub(crate) type TaskletBuffer = Vec<u8, { core::mem::size_of::<Tasklet<(), ()>>() }>;

/// Structure containing memory for Tasklet creation.
pub struct TaskletStorage<T: 'static, C> {
    /// Marks whether this storage is initialized.
    initialized: InternalCell<bool>,
    /// Buffer for the tasklet strucure.
    tasklet_buffer: InternalCell<TaskletBuffer>,
    /// Marker for the tasklet data type.
    _data_type_marker: PhantomData<T>,
    /// Marker for the tasklet internal context type.
    _context_type_marker: PhantomData<C>,
}

impl<T: Default, C> TaskletStorage<T, C> {
    /// Creates new storage.
    pub const fn new() -> Self {
        TaskletStorage {
            initialized: InternalCell::new(false),
            tasklet_buffer: InternalCell::new(TaskletBuffer::new()),
            _data_type_marker: PhantomData,
            _context_type_marker: PhantomData,
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
    /// Returns `Some(handle)` if this storage has been initialized, `None` otherwise.
    pub fn create_handle(&'static self) -> Option<TaskletHandle<T, C>> {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // the `init` function.
        match unsafe { *self.initialized.as_ref() } {
            true => {
                // SAFETY:: This is safe because storage has been initialized.
                let tasklet_ptr = unsafe { self.tasklet_ptr() };
                Some(TaskletHandle::new(tasklet_ptr))
            }
            false => None,
        }
    }

    /// Initializes this storage.
    ///
    /// Returns `InitError` in case of an initialization error, `()` otherwise.
    pub(crate) fn init(
        &'static self,
        config: TaskletConfig,
        step_fn: StepFn<T>,
    ) -> Result<TaskletPtr, InitError> {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // this function.
        if unsafe { *self.initialized.as_ref() } {
            return Err(InitError::StorageAlreadyInitialized);
        }

        let tasklet = Tasklet::<T, C>::new(config, step_fn);

        // SAFETY: This is safe, because it is borrowed mutably only here. It can be modified
        // (initialized) only once, because it is guarded by the `initialized` field. No external
        // borrow can be made before initialization.
        //
        // Because the `Tasklet` structure is of a constant size, the `tasklet_buffer` field is
        // statically allocated with enough memory for a 'placement new'.
        unsafe {
            let tasklet_buffer =
                self.tasklet_buffer.as_mut_ref().as_mut_ptr() as *mut Tasklet<T, C>;
            core::ptr::write(tasklet_buffer, tasklet);
        }

        // SAFETY: This is safe because it is only modified only here, and can't be externally
        // borrowed.
        unsafe {
            *self.initialized.as_mut_ref() = true;
        }

        // SAFETY: This is safe because we just initialized this storage.
        unsafe { Ok(self.tasklet_ptr()) }
    }

    /// Returns a raw pointer to the stored Tasklet structure.
    ///
    /// SAFETY: This is safe to call only when this storage has been initialized.
    #[inline(always)]
    unsafe fn buffer_ptr(&self) -> *const () {
        &*(self.tasklet_buffer.as_ref().as_ptr() as *const ())
    }

    /// Returns a tasklet pointer to the stored Tasklet structure.
    ///
    /// SAFETY: This is safe to call only when this storage has been initialized.
    #[inline(always)]
    unsafe fn tasklet_ptr(&self) -> TaskletPtr {
        TaskletPtr::new::<T, C>(self.buffer_ptr())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        static STORAGE: TaskletStorage<u8, ()> = TaskletStorage::new();

        assert!(!STORAGE.is_initialized());
    }

    #[test]
    fn initialize() {
        static STORAGE: TaskletStorage<u8, ()> = TaskletStorage::new();

        let name = "TaskName";
        let config = TaskletConfig { name };

        let init_result = STORAGE.init(config, |_| {});
        assert!(init_result.is_ok());
        assert!(STORAGE.is_initialized());

        assert_eq!(init_result.unwrap().get_name(), name);
    }

    #[test]
    fn create_handle() {
        static STORAGE: TaskletStorage<u8, ()> = TaskletStorage::new();

        let name = "TaskName";
        let config = TaskletConfig { name };

        let _ = STORAGE.init(config, |_| {});

        let handle = STORAGE.create_handle();
        assert!(handle.is_some());

        assert_eq!(handle.unwrap().get_name(), name);
    }
}
