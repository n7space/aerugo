//! Handle to a tasklet.
//!
//! Tasklet handle is available to the user of the system to reference and interact with the
//! tasklet via handle interface. All system API functions shall use handles when a reference to
//! tasklet is required, for example in subscribing tasklet to some data source.

use crate::task::Task;
use crate::tasklet::Tasklet;

/// Tasklet handle.
///
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
pub struct TaskletHandle<T: 'static, C: 'static> {
    /// Reference to the tasklet.
    tasklet: &'static Tasklet<T, C>,
}

impl<T: Default, C> TaskletHandle<T, C> {
    /// Creates new tasklet handle.
    ///
    /// * `tasklet` - Pointer to the tasklet.
    pub(crate) fn new(tasklet: &'static Tasklet<T, C>) -> Self {
        TaskletHandle { tasklet }
    }

    /// Returns name of this tasklet.
    #[inline(always)]
    pub fn get_name(&self) -> &'static str {
        self.tasklet.get_name()
    }

    /// Returns reference to the tasklet.
    pub(crate) fn tasklet(&self) -> &'static Tasklet<T, C> {
        self.tasklet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Task;
    use crate::tasklet::{Tasklet, TaskletConfig};

    fn create_tasklet() -> Tasklet<u8, ()> {
        let tasklet_config = TaskletConfig { name: "TaskName" };

        Tasklet::<u8, ()>::new(tasklet_config, |_, _| {}, ())
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
