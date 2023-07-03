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
    /// `get_name` function.
    pub(crate) get_name: fn(*const ()) -> &'static str,
    /// `get_status` function.
    pub(crate) get_status: fn(*const ()) -> TaskStatus,
    /// `set_status` function.
    pub(crate) set_status: fn(*const (), TaskStatus),
    /// `get_last_execution_time` function.
    pub(crate) get_last_execution_time: fn(*const ()) -> TimerInstantU64<1_000_000>,
    /// `set_last_execution_time` function.
    pub(crate) set_last_execution_time: fn(*const (), TimerInstantU64<1_000_000>),
    /// `has_work` function.
    pub(crate) has_work: fn(*const ()) -> bool,
    /// `execute` function.
    pub(crate) execute: fn(*const ()),
}

/// Constructs `Tasklet` virtual table for given `T` and `C` types.
///
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
pub(crate) fn tasklet_vtable<T: Default + 'static, C>() -> &'static TaskletVTable {
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
#[inline(always)]
fn get_name<T: Default + 'static, C>(ptr: *const ()) -> &'static str {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.get_name()
}

/// "Virtual" call to the `get_status` `Tasklet` function.
#[inline(always)]
fn get_status<T: Default + 'static, C>(ptr: *const ()) -> TaskStatus {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.get_status()
}

/// "Virtual" call to the `set_status` `Tasklet` function.
#[inline(always)]
fn set_status<T: Default + 'static, C>(ptr: *const (), status: TaskStatus) {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.set_status(status)
}

/// "Virtual" call to the `get_last_execution_time` `Tasklet` function.
#[inline(always)]
fn get_last_execution_time<T: Default + 'static, C>(ptr: *const ()) -> TimerInstantU64<1_000_000> {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.get_last_execution_time()
}

/// "Virtual" call to the `set_last_execution_time` `Tasklet` function.
#[inline(always)]
fn set_last_execution_time<T: Default + 'static, C>(
    ptr: *const (),
    time: TimerInstantU64<1_000_000>,
) {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.set_last_execution_time(time)
}

/// "Virtual" call to the `has_work` `Tasklet` function.
#[inline(always)]
fn has_work<T: Default + 'static, C>(ptr: *const ()) -> bool {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.has_work()
}

/// "Virtual" call to the `execute` `Tasklet` function.
#[inline(always)]
fn execute<T: Default + 'static, C>(ptr: *const ()) {
    // SAFETY: This is safe, because `Tasklet` is the only structure that implements `Task` trait,
    // and so is the only type that we store in the `*const ()`.
    let tasklet = unsafe { &*(ptr as *const Tasklet<T, C>) };
    tasklet.execute()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasklet::{Tasklet, TaskletConfig};

    fn create_tasklet() -> Tasklet<u8, ()> {
        let tasklet_config = TaskletConfig { name: "TaskName" };

        Tasklet::<u8, ()>::new(tasklet_config)
    }

    #[test]
    fn get_name() {
        let tasklet = create_tasklet();
        let ptr = &tasklet as *const Tasklet<u8, ()> as *const ();
        let vtable = tasklet_vtable::<u8, ()>();

        assert_eq!((vtable.get_name)(ptr), tasklet.get_name());
    }

    #[test]
    fn get_set_status() {
        let tasklet = create_tasklet();
        let ptr = &tasklet as *const Tasklet<u8, ()> as *const ();
        let vtable = tasklet_vtable::<u8, ()>();

        assert_eq!((vtable.get_status)(ptr), tasklet.get_status());
        (vtable.set_status)(ptr, TaskStatus::Waiting);
        assert_eq!((vtable.get_status)(ptr), tasklet.get_status());
    }

    #[test]
    fn get_set_last_execution_time() {
        let tasklet = create_tasklet();
        let ptr = &tasklet as *const Tasklet<u8, ()> as *const ();
        let vtable = tasklet_vtable::<u8, ()>();

        assert_eq!(
            (vtable.get_last_execution_time)(ptr),
            tasklet.get_last_execution_time()
        );
        (vtable.set_last_execution_time)(ptr, TimerInstantU64::<1_000_000>::from_ticks(42));
        assert_eq!(
            (vtable.get_last_execution_time)(ptr),
            tasklet.get_last_execution_time()
        );
    }

    #[test]
    fn has_work() {
        let tasklet = create_tasklet();
        let ptr = &tasklet as *const Tasklet<u8, ()> as *const ();
        let vtable = tasklet_vtable::<u8, ()>();

        assert_eq!((vtable.has_work)(ptr), tasklet.has_work());
    }

    #[test]
    fn execute() {
        // TODO
    }
}
