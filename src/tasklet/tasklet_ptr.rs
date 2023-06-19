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

use crate::tasklet::{tasklet_vtable, TaskletVTable};

/// Raw tasklet pointer.
pub(crate) struct TaskletPtr {
    /// Pointer to the `Tasklet<T, C>` structure.
    ptr: *const (),
    /// Tasklet virtual table.
    vtable: &'static TaskletVTable,
}

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
    pub(crate) fn get_name(&self) -> &'static str {
        (self.vtable.get_name)(self.ptr)
    }
}
