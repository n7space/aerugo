//! Tasklet virtual table.
//!
//! This structure implements a hand-made virtual table for the `Tasklet`. It is based on
//! the fact that `Tasklet` is the only structure that implements `Task` trait, and we can safely
//! store and then cast it from `*const ()`.
//!
//! For more information look at `TaskletPtr` structure.

use crate::task::{Task, TaskStatus};
use crate::tasklet::Tasklet;
use crate::time::TimerInstantU64;

/// Hand-made tasklet virtual table.
pub(crate) struct TaskletVTable {
    /// Pointer to [get_name](get_name()) function.
    pub(crate) get_name: fn(*const ()) -> &'static str,
    /// Pointer to [get_status](get_status()) function.
    pub(crate) get_status: fn(*const ()) -> TaskStatus,
    /// Pointer to [set_status](set_status()) function.
    pub(crate) set_status: fn(*const (), TaskStatus),
    /// Pointer to [get_last_execution_time](get_last_execution_time()) function.
    pub(crate) get_last_execution_time: fn(*const ()) -> TimerInstantU64<1_000_000>,
    /// Pointer to [set_last_execution_time](set_last_execution_time()) function.
    pub(crate) set_last_execution_time: fn(*const (), TimerInstantU64<1_000_000>),
    /// Pointer to [has_work](has_work()) function.
    pub(crate) has_work: fn(*const ()) -> bool,
    /// Pointer to [execute](execute()) function.
    pub(crate) execute: fn(*const ()),
}

/// Constructs `Tasklet` virtual table for given `T` and `C` types.
///
/// # Generic Parameters
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
pub(crate) fn tasklet_vtable<T: 'static, C: 'static>() -> &'static TaskletVTable {
    &TaskletVTable {
        get_name: get_name::<T, C>,
        get_status: get_status::<T, C>,
        set_status: set_status::<T, C>,
        get_last_execution_time: get_last_execution_time::<T, C>,
        set_last_execution_time: set_last_execution_time::<T, C>,
        has_work: has_work::<T, C>,
        execute: execute::<T, C>,
    }
}

/// "Virtual" call to the `get_name` `Tasklet` function.
///
/// See: [get_name](crate::task::Task::get_name())
#[inline(always)]
fn get_name<T: 'static, C: 'static>(ptr: *const ()) -> &'static str {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.get_name()
}

/// "Virtual" call to the `get_status` `Tasklet` function.
///
/// See: [get_status](crate::task::Task::get_status())
#[inline(always)]
fn get_status<T: 'static, C: 'static>(ptr: *const ()) -> TaskStatus {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.get_status()
}

/// "Virtual" call to the `set_status` `Tasklet` function.
///
/// See: [set_status](crate::task::Task::set_status())
#[inline(always)]
fn set_status<T: 'static, C: 'static>(ptr: *const (), status: TaskStatus) {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.set_status(status)
}

/// "Virtual" call to the `get_last_execution_time` `Tasklet` function.
///
/// See: [get_last_execution_time](crate::task::Task::get_last_execution_time())
#[inline(always)]
fn get_last_execution_time<T: 'static, C: 'static>(ptr: *const ()) -> TimerInstantU64<1_000_000> {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.get_last_execution_time()
}

/// "Virtual" call to the `set_last_execution_time` `Tasklet` function.
///
/// See: [set_last_execution_time](crate::task::Task::set_last_execution_time())
#[inline(always)]
fn set_last_execution_time<T: 'static, C: 'static>(
    ptr: *const (),
    time: TimerInstantU64<1_000_000>,
) {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.set_last_execution_time(time)
}

/// "Virtual" call to the `has_work` `Tasklet` function.
///
/// See: [has_work](crate::task::Task::has_work())
#[inline(always)]
fn has_work<T: 'static, C: 'static>(ptr: *const ()) -> bool {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.has_work()
}

/// "Virtual" call to the `execute` `Tasklet` function.
///
/// See: [execute](crate::task::Task::execute())
#[inline(always)]
fn execute<T: 'static, C: 'static>(ptr: *const ()) {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.execute()
}
