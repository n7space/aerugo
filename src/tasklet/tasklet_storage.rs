//! Static storage for [tasklet](crate::tasklet::Tasklet)
//!
//! As this system cannot use dynamic memory allocation, all structures have to be allocated
//! statically. Per good practices user is separated from the actual implementation and instead
//! only has to provide a static memory (via this structure) where the Tasklet will be allocated.

use super::Tasklet;

use core::marker::PhantomData;
use heapless::Vec;

use crate::internal_cell::InternalCell;
use crate::task::TaskHandle;

/// Type of the tasklet buffer storage.
pub(crate) type TaskletBuffer = Vec<u8, { core::mem::size_of::<Tasklet<(), ()>>() }>;

/// Structure containing memory for Tasklet creation.
pub struct TaskletStorage<T: 'static, C> {
    /// Marks whether this storage is initialized.
    _initialized: InternalCell<bool>,
    /// Buffer for the tasklet strucure.
    _tasklet_buffer: InternalCell<TaskletBuffer>,
    /// Marker for the tasklet data type.
    _data_type_marker: PhantomData<T>,
    /// Marker for the tasklet internal context type.
    _context_type_marker: PhantomData<C>,
}

impl<T, C> TaskletStorage<T, C> {
    /// Creates new storage.
    pub const fn new() -> Self {
        TaskletStorage {
            _initialized: InternalCell::new(false),
            _tasklet_buffer: InternalCell::new(TaskletBuffer::new()),
            _data_type_marker: PhantomData,
            _context_type_marker: PhantomData,
        }
    }

    /// Creates new handle to a tasklet allocated in this storage.
    ///
    /// Returns `Some(handle)` if this storage has been initialized. `None` otherwise.
    pub fn create_task_handle(&'static self) -> Option<TaskHandle<T>> {
        todo!()
    }
}
