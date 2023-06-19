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
use crate::tasklet::{TaskletConfig, TaskletHandle, TaskletPtr};

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

impl<T, C> TaskletStorage<T, C> {
    /// Creates new storage.
    pub const fn new() -> Self {
        TaskletStorage {
            initialized: InternalCell::new(false),
            tasklet_buffer: InternalCell::new(TaskletBuffer::new()),
            _data_type_marker: PhantomData,
            _context_type_marker: PhantomData,
        }
    }

    /// Creates new handle to a tasklet allocated in this storage.
    ///
    /// Returns `Some(handle)` if this storage has been initialized, `None` otherwise.
    pub fn create_handle(&'static self) -> Option<TaskletHandle<T, C>> {
        match self.buffer_ptr() {
            Some(ptr) => {
                let task_ptr = TaskletPtr::new::<T, C>(ptr);
                Some(TaskletHandle::new(task_ptr))
            }
            None => None,
        }
    }

    /// Initializes this storage.
    ///
    /// Returns `InitError` in case of an initialization error, `()` otherwise.
    pub(crate) fn init(&'static self, config: TaskletConfig) -> Result<(), InitError> {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // this function.
        if unsafe { *self.initialized.as_ref() } {
            return Err(InitError::StorageAlreadyInitialized);
        }

        let tasklet = Tasklet::<T, C>::new(config);

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

        Ok(())
    }

    /// Returns a raw pointer to the stored Tasklet structure.
    ///
    /// Returns `Some(*const ())` if this storage has been initialized, `None` otherwise.
    fn buffer_ptr(&self) -> Option<*const ()> {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // the `init` function.
        match unsafe { *self.initialized.as_ref() } {
            true => Some(unsafe { &*(self.tasklet_buffer.as_ref().as_ptr() as *const ()) }),
            false => None,
        }
    }
}
