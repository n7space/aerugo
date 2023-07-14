//! Raw tasklet pointer.
//!
//! To hide generic parameters of the `Tasklet` we use `Task` trait. It's necessary for in example
//! storing tasklets that are ready for execution. But then using trait objects creates a dynamic
//! dispatching, which is unwanted in the embedded environment especially in tight loops (like
//! tasklet exectution loop). Additionally we don't need polymorphism, as the trait is not used for
//! defining a shared behavior between different types and only one structure (`Tasklet`) is
//! implementing the trait.
//!
//! This structure is used instead of trait objects (`&'static dyn Task`) and uses a hand-made
//! virtual table. It is based on the fact that `Task` is only implemented for the `Tasklet` so
//! we can safely store `&'static Tasklet<T, C>` in `*const ()`.

use core::cmp::Ordering;

use crate::task::TaskStatus;
use crate::tasklet::{tasklet_vtable, Tasklet, TaskletVTable};
use crate::time::TimerInstantU64;

/// Raw tasklet pointer.
#[derive(Clone)]
pub(crate) struct TaskletPtr {
    /// Pointer to the `Tasklet<T, C>` structure.
    ptr: *const (),
    /// Tasklet virtual table.
    vtable: &'static TaskletVTable,
}

unsafe impl Sync for TaskletPtr {}
unsafe impl Send for TaskletPtr {}

impl TaskletPtr {
    /// Creates new tasklet pointer from referernce
    ///
    /// * `tasklet` - Reference to the tasklet
    pub(crate) fn new<T: 'static, C: 'static>(tasklet: &'static Tasklet<T, C>) -> Self {
        TaskletPtr {
            ptr: tasklet as *const Tasklet<T, C> as *const (),
            vtable: tasklet_vtable::<T, C>(),
        }
    }

    /// See: [get_name](crate::task::Task::get_name())
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn get_name(&self) -> &'static str {
        (self.vtable.get_name)(self.ptr)
    }

    /// See: [get_status](crate::task::Task::get_status())
    #[inline(always)]
    pub(crate) fn get_status(&self) -> TaskStatus {
        (self.vtable.get_status)(self.ptr)
    }

    /// See: [set_status](crate::task::Task::set_status())
    #[inline(always)]
    pub(crate) fn set_status(&self, status: TaskStatus) {
        (self.vtable.set_status)(self.ptr, status)
    }

    /// See: [get_last_execution_time](crate::task::Task::get_last_execution_time())
    #[inline(always)]
    pub(crate) fn get_last_execution_time(&self) -> TimerInstantU64<1_000_000> {
        (self.vtable.get_last_execution_time)(self.ptr)
    }

    /// See: [set_last_execution_time](crate::task::Task::set_last_execution_time())
    #[inline(always)]
    pub(crate) fn set_last_execution_time(&self, time: TimerInstantU64<1_000_000>) {
        (self.vtable.set_last_execution_time)(self.ptr, time)
    }

    /// See: [has_work](crate::task::Task::has_work())
    #[inline(always)]
    pub(crate) fn has_work(&self) -> bool {
        (self.vtable.has_work)(self.ptr)
    }

    /// See: [execute](crate::task::Task::execute())
    #[inline(always)]
    pub(crate) fn execute(&self) {
        (self.vtable.execute)(self.ptr)
    }
}

impl Ord for TaskletPtr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_last_execution_time()
            .cmp(&other.get_last_execution_time())
            .reverse()
    }
}

impl PartialOrd for TaskletPtr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_last_execution_time()
            .partial_cmp(&other.get_last_execution_time())
            .map(|ordering| ordering.reverse())
    }
}

impl Eq for TaskletPtr {}

impl PartialEq for TaskletPtr {
    fn eq(&self, other: &Self) -> bool {
        self.get_last_execution_time()
            .eq(&other.get_last_execution_time())
    }
}
