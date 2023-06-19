//! Handle to a tasklet.
//!
//! Tasklet handle is available to the user of the system to reference and interact with the
//! tasklet via handle interface. All system API functions shall use handles when a reference to
//! tasklet is required, for example in subscribing tasklet to some data source.

use core::marker::PhantomData;

use crate::tasklet::TaskletPtr;

/// Tasklet handle.
///
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
pub struct TaskletHandle<T: 'static, C> {
    /// Pointer to the tasklet.
    tasklet: TaskletPtr,
    /// Marker for the type that is processed by the tasklet.
    _data_marker: PhantomData<T>,
    /// Marker for the type of tasklet context data.
    _context_marker: PhantomData<C>,
}

impl<T: 'static, C> TaskletHandle<T, C> {
    /// Creates new tasklet handle.
    ///
    /// * `tasklet` - Pointer to the tasklet.
    pub(crate) fn new(tasklet: TaskletPtr) -> Self {
        TaskletHandle {
            tasklet,
            _data_marker: PhantomData,
            _context_marker: PhantomData,
        }
    }

    /// Returns name of this tasklet.
    pub fn get_name(&self) -> &'static str {
        self.tasklet.get_name()
    }
}
