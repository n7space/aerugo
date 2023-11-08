//! Tasklet virtual table.
//!
//! This structure implements a hand-made virtual table for the `Tasklet`. It is based on
//! the fact that `Tasklet` is the only structure that implements `Task` trait, and we can safely
//! store and then cast it from `*const ()`.
//!
//! For more information look at `TaskletPtr` structure.

use crate::tasklet::{Tasklet, TaskletId, TaskletStatus};
use crate::time::Instant;

/// Hand-made tasklet virtual table.
pub(crate) struct TaskletVTable {
    /// Pointer to [get_id](get_id()) function.
    pub(crate) get_id: fn(*const ()) -> TaskletId,
    /// Pointer to [get_name](get_name()) function.
    pub(crate) get_name: fn(*const ()) -> &'static str,
    /// Pointer to [get_priority](get_priority()) function.
    pub(crate) get_priority: fn(*const ()) -> u8,
    /// Pointer to [get_status](get_status()) function.
    pub(crate) get_status: fn(*const ()) -> TaskletStatus,
    /// Pointer to [set_status](set_status()) function.
    pub(crate) set_status: fn(*const (), TaskletStatus),
    /// Pointer to [get_last_execution_time](get_last_execution_time()) function.
    pub(crate) get_last_execution_time: fn(*const ()) -> Instant,
    /// Pointer to [set_last_execution_time](set_last_execution_time()) function.
    pub(crate) set_last_execution_time: fn(*const (), Instant),
    /// Pointer to [has_work](has_work()) function.
    pub(crate) has_work: fn(*const ()) -> bool,
    /// Pointer to [is_active](is_active()) function.
    pub(crate) is_active: fn(*const ()) -> bool,
    /// Pointer to [is_subscribed](is_subscribed()) function.
    pub(crate) is_subscribed: fn(*const ()) -> bool,
    /// Pointer to [execute](execute()) function.
    pub(crate) execute: fn(*const ()) -> bool,
}

/// Constructs `Tasklet` virtual table for given `T` and `C` types.
///
/// # Generic Parameters
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
pub(crate) fn tasklet_vtable<T: 'static, C: 'static, const COND_COUNT: usize>(
) -> &'static TaskletVTable {
    &TaskletVTable {
        get_id: get_id::<T, C, COND_COUNT>,
        get_name: get_name::<T, C, COND_COUNT>,
        get_priority: get_priority::<T, C, COND_COUNT>,
        get_status: get_status::<T, C, COND_COUNT>,
        set_status: set_status::<T, C, COND_COUNT>,
        get_last_execution_time: get_last_execution_time::<T, C, COND_COUNT>,
        set_last_execution_time: set_last_execution_time::<T, C, COND_COUNT>,
        has_work: has_work::<T, C, COND_COUNT>,
        is_active: is_active::<T, C, COND_COUNT>,
        is_subscribed: is_subscribed::<T, C, COND_COUNT>,
        execute: execute::<T, C, COND_COUNT>,
    }
}

/// "Virtual" call to the `get_id` `Tasklet` function.
///
/// See: [get_id](crate::tasklet::Tasklet::get_id())
#[inline(always)]
fn get_id<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) -> TaskletId {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.get_id()
}

/// "Virtual" call to the `get_name` `Tasklet` function.
///
/// See: [get_name](crate::tasklet::Tasklet::get_name())
#[inline(always)]
fn get_name<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) -> &'static str {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.get_name()
}

/// "Virtual" call to the `get_priority` `Tasklet` function.
///
/// See: [get_priority](crate::tasklet::Tasklet::get_priority())
#[inline(always)]
fn get_priority<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) -> u8 {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.get_priority()
}

/// "Virtual" call to the `get_status` `Tasklet` function.
///
/// See: [get_status](crate::tasklet::Tasklet::get_status())
#[inline(always)]
fn get_status<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) -> TaskletStatus {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.get_status()
}

/// "Virtual" call to the `set_status` `Tasklet` function.
///
/// See: [set_status](crate::tasklet::Tasklet::set_status())
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
/// See: [get_last_execution_time](crate::tasklet::Tasklet::get_last_execution_time())
#[inline(always)]
fn get_last_execution_time<T: 'static, C: 'static, const COND_COUNT: usize>(
    ptr: *const (),
) -> Instant {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.get_last_execution_time()
}

/// "Virtual" call to the `set_last_execution_time` `Tasklet` function.
///
/// See: [set_last_execution_time](crate::tasklet::Tasklet::set_last_execution_time())
#[inline(always)]
fn set_last_execution_time<T: 'static, C: 'static, const COND_COUNT: usize>(
    ptr: *const (),
    time: Instant,
) {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.set_last_execution_time(time)
}

/// "Virtual" call to the `has_work` `Tasklet` function.
///
/// See: [has_work](crate::tasklet::Tasklet::has_work())
#[inline(always)]
fn has_work<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) -> bool {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.has_work()
}

/// "Virtual" call to the `is_active` `Tasklet` function.
///
/// See: [is_active](crate::tasklet::Tasklet::is_active())
#[inline(always)]
fn is_active<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) -> bool {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.is_active()
}

/// "Virtual" call to the `is_subscribed` `Tasklet` function.
///
/// See: [is_active](crate::tasklet::Tasklet::is_subscribed())
#[inline(always)]
fn is_subscribed<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) -> bool {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.is_subscribed()
}

/// "Virtual" call to the `execute` `Tasklet` function.
///
/// See: [execute](crate::tasklet::Tasklet::execute())
#[inline(always)]
fn execute<T: 'static, C: 'static, const COND_COUNT: usize>(ptr: *const ()) -> bool {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C, COND_COUNT>) };
    tasklet.execute()
}
