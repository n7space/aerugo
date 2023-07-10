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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Task;
    use crate::tasklet::{Tasklet, TaskletConfig};

    fn create_tasklet() -> Tasklet<u8, ()> {
        let tasklet_config = TaskletConfig { name: "TaskName" };

        Tasklet::<u8, ()>::new(tasklet_config)
    }

    fn create_tasklet_ptr(tasklet: &Tasklet<u8, ()>) -> TaskletPtr {
        let ptr = tasklet as *const Tasklet<u8, ()> as *const ();

        TaskletPtr::new::<u8, ()>(ptr)
    }

    #[test]
    fn get_name() {
        let tasklet = create_tasklet();
        let tasklet_ptr = create_tasklet_ptr(&tasklet);
        let tasklet_handle = TaskletHandle::<u8, ()>::new(tasklet_ptr);

        assert_eq!(tasklet_handle.get_name(), tasklet.get_name());
    }
}
