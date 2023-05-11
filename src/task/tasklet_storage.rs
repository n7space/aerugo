//! TODO

use core::fmt::Debug;
use core::marker::PhantomData;
use heapless::Vec;

use crate::internal_cell::InternalCell;
use crate::task::{TaskHandle, Tasklet};

/// TODO
pub(crate) type TaskletBuffer = Vec<u8, { core::mem::size_of::<Tasklet<(), ()>>() }>;

/// TODO
pub struct TaskletStorage<T: 'static + Debug, C> {
    /// Marks whether this storage is initialized.
    ///
    /// SAFETY: This is only modified in the [`Self::init()`].
    _initialized: InternalCell<bool>,
    /// Buffer for the tasklet strucure.
    ///
    /// SAFETY: This is only modified in the [`Self::init()`].
    _tasklet_buffer: InternalCell<TaskletBuffer>,
    /// Marker for the tasklet data type.
    _data_type_marker: PhantomData<T>,
    /// Marker for the tasklet internal context type.
    _context_type_marker: PhantomData<C>,
}

impl<T: Debug, C> TaskletStorage<T, C> {
    /// TODO
    pub const fn new() -> Self {
        TaskletStorage {
            _initialized: InternalCell::new(false),
            _tasklet_buffer: InternalCell::new(TaskletBuffer::new()),
            _data_type_marker: PhantomData,
            _context_type_marker: PhantomData,
        }
    }

    /// TODO
    pub fn create_task_handle(&'static self) -> Option<TaskHandle<T>> {
        todo!()
    }
}
