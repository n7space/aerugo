//! Tasklet virtual table.
//!
//! This structure implements a hand-made virtual table for the `Tasklet`. It is based on
//! the fact that `Tasklet` is the only structure that implements `Task` trait, and we can safely
//! store and then cast it from `*const ()`.
//!
//! For more information look at `TaskletPtr` structure.

use crate::task::Task;
use crate::tasklet::Tasklet;

/// Hand-made tasklet virtual table.
pub(crate) struct TaskletVTable {
    /// `get_name` function.
    pub(crate) get_name: fn(*const ()) -> &'static str,
}

/// Constructs `Tasklet` virtual table for given `T` and `C` types.
///
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
pub(crate) fn tasklet_vtable<T: 'static, C>() -> &'static TaskletVTable {
    &TaskletVTable {
        get_name: get_name::<T, C>,
    }
}

/// "Virtual" call to the `get_name` `Tasklet` function.
#[inline(always)]
fn get_name<T: 'static, C>(ptr: *const ()) -> &'static str {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.get_name()
}
