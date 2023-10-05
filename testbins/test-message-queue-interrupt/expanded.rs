#![feature(prelude_import)]
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
extern crate calldwell;
use aerugo::{
    logln, Aerugo, BooleanConditionHandle, BooleanConditionStorage, InitApi, RuntimeApi,
    SystemHardwareConfig, TaskletConfig, TaskletStorage,
};
use cortex_m_rt::{entry, exception};
use lazy_static::lazy_static;
struct TaskAContext {}
#[automatically_derived]
impl ::core::default::Default for TaskAContext {
    #[inline]
    fn default() -> TaskAContext {
        TaskAContext {}
    }
}
#[allow(clippy::needless_pass_by_ref_mut)]
fn task_a(value: bool, _: &mut TaskAContext, _: &dyn RuntimeApi) {
    {};
}
static TASK_A_STORAGE: TaskletStorage<bool, TaskAContext, 1> = TaskletStorage::new();
static DONE_CONDITION_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();
#[doc(hidden)]
#[export_name = "main"]
pub unsafe extern "C" fn __cortex_m_rt_main_trampoline() {
    __cortex_m_rt_main()
}
fn __cortex_m_rt_main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());
    aerugo.create_boolean_condition(false, &DONE_CONDITION_STORAGE);
    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };
    aerugo.create_tasklet(task_a_config, task_a, &TASK_A_STORAGE);
    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    let done_condition_handle = DONE_CONDITION_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_condition(&task_a_handle, &done_condition_handle);
    aerugo.start();
}
const _: () = {
    let _ = cortex_m_rt::Exception::SysTick;
};
#[doc(hidden)]
#[export_name = "SysTick"]
pub unsafe extern "C" fn __cortex_m_rt_SysTick_trampoline() {
    __cortex_m_rt_SysTick()
}
fn __cortex_m_rt_SysTick() {
    {
        exception::SysTick;
    }
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct DONE_CONDITION_HANDLE {
        __private_field: (),
    }
    #[doc(hidden)]
    static DONE_CONDITION_HANDLE: DONE_CONDITION_HANDLE = DONE_CONDITION_HANDLE {
        __private_field: (),
    };
    impl ::lazy_static::__Deref for DONE_CONDITION_HANDLE {
        type Target = BooleanConditionHandle;
        fn deref(&self) -> &BooleanConditionHandle {
            #[inline(always)]
            fn __static_ref_initialize() -> BooleanConditionHandle {
                DONE_CONDITION_STORAGE.create_handle().unwrap()
            }
            #[inline(always)]
            fn __stability() -> &'static BooleanConditionHandle {
                static LAZY: ::lazy_static::lazy::Lazy<BooleanConditionHandle> = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for DONE_CONDITION_HANDLE {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    DONE_CONDITION_HANDLE.set_value(true);
}
