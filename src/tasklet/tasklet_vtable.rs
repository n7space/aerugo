//! Tasklet virtual table.
//!
//! This structure implements a hand-made virtual table for the `Tasklet`. It is based on
//! the fact that `Tasklet` is the only structure that implements `Task` trait, and we can safely
//! store and then cast it from `*const ()`.
//!
//! For more information look at `TaskletPtr` structure.

use crate::tasklet::{Tasklet, TaskletStatus};
use crate::time::TimerInstantU64;

/// Hand-made tasklet virtual table.
pub(crate) struct TaskletVTable {
    /// Pointer to [get_name](get_name()) function.
    pub(crate) get_name: fn(*const ()) -> &'static str,
    /// Pointer to [get_priority](get_priority()) function.
    pub(crate) get_priority: fn(*const ()) -> u8,
    /// Pointer to [get_status](get_status()) function.
    pub(crate) get_status: fn(*const ()) -> TaskletStatus,
    /// Pointer to [set_status](set_status()) function.
    pub(crate) set_status: fn(*const (), TaskletStatus),
    /// Pointer to [get_last_execution_time](get_last_execution_time()) function.
    pub(crate) get_last_execution_time: fn(*const ()) -> TimerInstantU64<1_000_000>,
    /// Pointer to [set_last_execution_time](set_last_execution_time()) function.
    pub(crate) set_last_execution_time: fn(*const (), TimerInstantU64<1_000_000>),
    /// Pointer to [is_ready](is_ready()) function.
    pub(crate) is_ready: fn(*const ()) -> bool,
    /// Pointer to [execute](execute()) function.
    pub(crate) execute: fn(*const ()),
}

/// Constructs `Tasklet` virtual table for given `T` and `C` types.
///
/// # Generic Parameters
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
pub(crate) fn tasklet_vtable<T: 'static, C: 'static, const COND_COUNT: usize>(
) -> &'static TaskletVTable {
    &TaskletVTable {
        get_name: get_name::<T, C, COND_COUNT>,
        get_priority: get_priority::<T, C, COND_COUNT>,
        get_status: get_status::<T, C, COND_COUNT>,
        set_status: set_status::<T, C, COND_COUNT>,
        get_last_execution_time: get_last_execution_time::<T, C, COND_COUNT>,
        set_last_execution_time: set_last_execution_time::<T, C, COND_COUNT>,
        is_ready: is_ready::<T, C, COND_COUNT>,
        execute: execute::<T, C, COND_COUNT>,
    }
}

/// "Virtual" call to the `get_name` `Tasklet` function.
///
/// See: [get_name](crate::task::Task::get_name())
#[inline(always)]
fn get_name<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) -> &'static str {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.get_name()
}

/// "Virtual" call to the `get_priority` `Tasklet` function.
///
/// See: [get_priority](crate::task::Task::get_priority())
#[inline(always)]
fn get_priority<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) -> u8 {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.get_priority()
}

/// "Virtual" call to the `get_status` `Tasklet` function.
///
/// See: [get_status](crate::task::Task::get_status())
#[inline(always)]
fn get_status<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) -> TaskletStatus {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.get_status()
}

/// "Virtual" call to the `set_status` `Tasklet` function.
///
/// See: [set_status](crate::task::Task::set_status())
#[inline(always)]
fn set_status<T: 'static, C: 'static, const COND_COUNT: usize>(
    ptr: *const (),
    status: TaskletStatus,
) {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.set_status(status)
}

/// "Virtual" call to the `get_last_execution_time` `Tasklet` function.
///
/// See: [get_last_execution_time](crate::task::Task::get_last_execution_time())
#[inline(always)]
fn get_last_execution_time<T: 'static, C: 'static, const COND_COUNT: usize>(
    ptr: *const (),
) -> TimerInstantU64<1_000_000> {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.get_last_execution_time()
}

/// "Virtual" call to the `set_last_execution_time` `Tasklet` function.
///
/// See: [set_last_execution_time](crate::task::Task::set_last_execution_time())
#[inline(always)]
fn set_last_execution_time<T: 'static, C: 'static, const COND_COUNT: usize>(
    ptr: *const (),
    time: TimerInstantU64<1_000_000>,
) {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.set_last_execution_time(time)
}

/// "Virtual" call to the `is_ready` `Tasklet` function.
///
/// See: [is_ready](crate::task::Task::is_ready())
#[inline(always)]
fn is_ready<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) -> bool {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.is_ready()
}

/// "Virtual" call to the `execute` `Tasklet` function.
///
/// See: [execute](crate::task::Task::execute())
#[inline(always)]
fn execute<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.execute()
}
