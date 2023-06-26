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
use crate::tasklet::{tasklet_vtable, TaskletVTable};
use crate::time::TimerInstantU64;

/// Raw tasklet pointer.
pub(crate) struct TaskletPtr {
    /// Pointer to the `Tasklet<T, C>` structure.
    ptr: *const (),
    /// Tasklet virtual table.
    vtable: &'static TaskletVTable,
}

unsafe impl Sync for TaskletPtr {}
unsafe impl Send for TaskletPtr {}

impl TaskletPtr {
    /// Creates new pointer
    ///
    /// * `ptr` - Pointer to memory where tasklet is allocated.
    pub(crate) fn new<T: 'static, C>(ptr: *const ()) -> Self {
        TaskletPtr {
            ptr,
            vtable: tasklet_vtable::<T, C>(),
        }
    }

    /// Returns tasklet name.
    #[inline(always)]
    pub(crate) fn get_name(&self) -> &'static str {
        (self.vtable.get_name)(self.ptr)
    }

    /// Returns tasklet status.
    #[inline(always)]
    pub(crate) fn get_status(&self) -> TaskStatus {
        (self.vtable.get_status)(self.ptr)
    }

    /// Sets tasklet status.
    ///
    /// * `status` - New tasklet status.
    #[inline(always)]
    pub(crate) fn set_status(&self, status: TaskStatus) {
        (self.vtable.set_status)(self.ptr, status)
    }

    /// Returns last execution time.
    #[inline(always)]
    pub(crate) fn get_last_execution_time(&self) -> TimerInstantU64<1_000_000> {
        (self.vtable.get_last_execution_time)(self.ptr)
    }

    /// Sets last execution time.
    ///
    /// * `time` - Last execution time.
    #[inline(always)]
    pub(crate) fn set_last_execution_time(&self, time: TimerInstantU64<1_000_000>) {
        (self.vtable.set_last_execution_time)(self.ptr, time)
    }

    /// Checks if there are any more data for the tasklet to process.
    #[inline(always)]
    pub(crate) fn has_work(&self) -> bool {
        (self.vtable.has_work)(self.ptr)
    }

    /// Executes task.
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
        match self
            .get_last_execution_time()
            .partial_cmp(&other.get_last_execution_time())
        {
            Some(ordering) => return Some(ordering.reverse()),
            None => return None,
        }
    }
}

impl Eq for TaskletPtr {}

impl PartialEq for TaskletPtr {
    fn eq(&self, other: &Self) -> bool {
        self.get_last_execution_time()
            .eq(&other.get_last_execution_time())
    }
}
