//! Raw tasklet pointer.
//!
//! To hide generic parameters of the `Tasklet` we use `Task` trait. It's necessary for in example
//! storing tasklets that are ready for execution. But then using trait objects creates a dynamic
//! dispatching, which is unwanted in the embedded environment especially in tight loops (like
//! tasklet execution loop). Additionally we don't need polymorphism, as the trait is not used for
//! defining a shared behavior between different types and only one structure (`Tasklet`) is
//! implementing the trait.
//!
//! This structure is used instead of trait objects (`&'static dyn Task`) and uses a hand-made
//! virtual table. It is based on the fact that `Task` is only implemented for the `Tasklet` so
//! we can safely store `&'static Tasklet<T, C>` in `*const ()`.

use core::cmp::Ordering;

use crate::tasklet::{tasklet_vtable, Tasklet, TaskletStatus, TaskletVTable};
use crate::SystemInstant;

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
    /// Creates new tasklet pointer from reference
    ///
    /// # Parameters
    /// * `tasklet` - Reference to the tasklet
    pub(crate) fn new<T: 'static, C: 'static, const COND_COUNT: usize>(
        tasklet: &'static Tasklet<T, C, COND_COUNT>,
    ) -> Self {
        TaskletPtr {
            ptr: tasklet as *const Tasklet<T, C, COND_COUNT> as *const (),
            vtable: tasklet_vtable::<T, C, COND_COUNT>(),
        }
    }

    /// See: [get_name](crate::tasklet::Tasklet::get_name())
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn get_name(&self) -> &'static str {
        (self.vtable.get_name)(self.ptr)
    }

    /// See: [get_priority](crate::tasklet::Tasklet::get_priority())
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn get_priority(&self) -> u8 {
        (self.vtable.get_priority)(self.ptr)
    }

    /// See: [get_status](crate::tasklet::Tasklet::get_status())
    #[inline(always)]
    pub(crate) fn get_status(&self) -> TaskletStatus {
        (self.vtable.get_status)(self.ptr)
    }

    /// See: [set_status](crate::tasklet::Tasklet::set_status())
    #[inline(always)]
    pub(crate) fn set_status(&self, status: TaskletStatus) {
        (self.vtable.set_status)(self.ptr, status)
    }

    /// See: [get_last_execution_time](crate::tasklet::Tasklet::get_last_execution_time())
    #[inline(always)]
    pub(crate) fn get_last_execution_time(&self) -> SystemInstant {
        (self.vtable.get_last_execution_time)(self.ptr)
    }

    /// See: [set_last_execution_time](crate::tasklet::Tasklet::set_last_execution_time())
    #[inline(always)]
    pub(crate) fn set_last_execution_time(&self, time: SystemInstant) {
        (self.vtable.set_last_execution_time)(self.ptr, time)
    }

    /// See: [has_work](crate::tasklet::Tasklet::has_work())
    #[inline(always)]
    pub(crate) fn has_work(&self) -> bool {
        (self.vtable.has_work)(self.ptr)
    }

    /// See: [is_active](crate::tasklet::Tasklet::is_active())
    #[inline(always)]
    pub(crate) fn is_active(&self) -> bool {
        (self.vtable.is_active)(self.ptr)
    }

    /// See: [execute](crate::tasklet::Tasklet::execute())
    #[inline(always)]
    pub(crate) fn execute(&self) -> bool {
        (self.vtable.execute)(self.ptr)
    }
}

impl Ord for TaskletPtr {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_priority() == other.get_priority() {
            self.get_last_execution_time()
                .cmp(&other.get_last_execution_time())
                .reverse()
        } else {
            self.get_priority().cmp(&other.get_priority())
        }
    }
}

impl PartialOrd for TaskletPtr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TaskletPtr {}

impl PartialEq for TaskletPtr {
    fn eq(&self, other: &Self) -> bool {
        self.ptr.eq(&other.ptr)
    }
}
