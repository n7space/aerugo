//! System control and interface for the user to interact with it.
//!
//! This module contains mainly system API that can be used by the user to interact with the
//! system and internal API used by other parts of the system.
//!
//! This module also contains singleton instances of all system parts.

pub mod error;

pub use self::error::InitError;
use self::error::RuntimeError;

use aerugo_hal::system_hal::SystemHal;
use bare_metal::CriticalSection;
use env_parser::read_env;
use internal_cell::InternalCell;

use crate::api::{InitApi, RuntimeApi, SystemApi};
use crate::boolean_condition::{BooleanConditionSet, BooleanConditionStorage};
use crate::data_receiver::DataReceiver;
use crate::event::{EventHandle, EventStorage};
use crate::execution_monitoring::ExecutionStats;
use crate::executor::Executor;
use crate::hal::{Hal, Peripherals};
use crate::message_queue::{MessageQueueHandle, MessageQueueStorage};
use crate::queue::Queue;
use crate::task::TaskId;
use crate::tasklet::{StepFn, TaskletHandle, TaskletPtr, TaskletStorage};
use crate::time_manager::TimeManager;

/// Core system.
///
/// This is used to access the system API, both by the user and by the internal system parts.
pub static AERUGO: Aerugo = Aerugo::new();

/// System scheduler.
///
/// Singleton instance of the scheduler. Used directly only by the [Aerugo](crate::aerugo::Aerugo)
/// structure, which exposes some functionality via it's API.
static EXECUTOR: Executor = Executor::new();
/// Time manager.
///
/// Singleton instance of the time manager. Used directly only by the [Aerugo](crate::aerugo::Aerugo)
/// structure.
static TIME_MANAGER: TimeManager = TimeManager::new();

/// System structure.
///
/// This shouldn't be created by hand by the user or anywhere else in the code.
/// It should be used as a [singleton](crate::aerugo::AERUGO) that acts as a system API,
/// both for user and for the internal system parts.
pub struct Aerugo {
    /// Hardware Access/Abstraction Layer.
    hal: InternalCell<Option<Hal>>,
}

impl Aerugo {
    /// Maximum number of tasklets registered in the system.
    #[read_env("AERUGO_TASKLET_COUNT")]
    pub(crate) const TASKLET_COUNT: usize = 0;

    /// Creates new system instance.
    ///
    /// # Safety
    /// This shouldn't be called in more that [one place](crate::aerugo::AERUGO).
    const fn new() -> Self {
        Aerugo {
            hal: InternalCell::new(None),
        }
    }

    /// Initialize the system runtime.
    pub fn initialize(&'static self) -> Result<(), RuntimeError> {
        let peripherals = match Peripherals::new() {
            None => return Err(RuntimeError::SystemAlreadyInitialized),
            Some(p) => p,
        };

        // SAFETY: This is safe, because it's a single-core environment,
        // and no other references to Hal should exist during this call.
        unsafe {
            self.hal.as_mut_ref().replace(Hal::new(peripherals));
        }

        Ok(())
    }

    /// Returns PAC peripherals. Can be called successfully only once, as peripherals are moved out of system.
    pub fn peripherals(&'static self) -> Result<Option<Peripherals>, RuntimeError> {
        // SAFETY: This is safe, because it's a single-core environment,
        // and no other references to Hal should exist during this call.
        match unsafe { self.hal.as_mut_ref() } {
            None => Err(RuntimeError::SystemNotInitialized),
            Some(hal) => Ok(hal.peripherals()),
        }
    }

    /// Starts the system.
    ///
    /// This starts an executor that never returns, executing ready tasklets in a loop.
    ///
    /// # Safety
    /// This shouldn't be called more than once.
    pub fn start(&'static self) -> ! {
        EXECUTOR.run_scheduler()
    }
}

impl InitApi for Aerugo {
    type Duration = crate::time::MillisDurationU32;

    /// Creates new tasklet in the system.
    ///
    /// Tasklet is created in the passed `storage` memory. Storage has to be static to keep the stored
    /// tasklet for the whole duration of systems' life.
    ///
    /// # Generic Arguments
    /// * `T` - Type of the data processed by the tasklet.
    /// * `C` - Type of the structure with tasklet context data.
    ///
    /// # Arguments
    /// * `config` - Tasklet creation configuration.
    /// * `step_fn` - Tasklet step function.
    /// * `storage` - Static memory storage where the tasklet should be allocated.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because it initializes the
    /// passed storage which is safe only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{InitApi, TaskletConfig, TaskletStorage, AERUGO};
    /// #[derive(Default)]
    /// struct TaskCtx;
    ///
    /// fn task(_: u8, _: &mut TaskCtx) {}
    ///
    /// static TASK_STORAGE: TaskletStorage<u8, TaskCtx> = TaskletStorage::new();
    ///
    /// fn main() {
    ///     let task_config = TaskletConfig::default();
    ///     AERUGO.create_tasklet(task_config, task, &TASK_STORAGE).expect("Unable to create Tasklet");
    ///     #
    ///     # assert!(TASK_STORAGE.is_initialized());
    ///
    ///     // Do something with the tasklet via handle.
    ///     let task_handle = TASK_STORAGE.create_handle();
    ///     #
    ///     # assert!(task_handle.is_some())
    /// }
    /// ```
    fn create_tasklet<T, C: Default>(
        &'static self,
        config: Self::TaskConfig,
        step_fn: StepFn<T, C>,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error> {
        // SAFETY: This is safe, as long as this function is called only during system initialization.
        unsafe { storage.init(config, step_fn, C::default()) }
    }

    /// Creates new tasklet in the system with initialized context data.
    ///
    /// Tasklet is created in the passed `storage` memory. Storage has to be static to keep the stored
    /// tasklet for the whole duration of system life.
    ///
    /// # Generic Arguments
    /// * `T` - Type of the data processed by the tasklet.
    /// * `C` - Type of the structure with tasklet context data.
    ///
    /// # Arguments
    /// * `config` - Tasklet creation configuration.
    /// * `step_fn` - Tasklet step function.
    /// * `context` - Tasklet context data.
    /// * `storage` - Static memory storage where the tasklet should be allocated.
    ///
    /// # Return
    /// `Error` in case of an error, `Ok(())` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because it initializes the
    /// passed storage which is safe only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{InitApi, TaskletConfig, TaskletStorage, AERUGO};
    /// struct TaskCtx {
    ///     value: u8,
    /// }
    ///
    /// fn task(_: u8, _: &mut TaskCtx) {}
    ///
    /// static TASK_STORAGE: TaskletStorage<u8, TaskCtx> = TaskletStorage::new();
    ///
    /// fn main() {
    ///     let task_config = TaskletConfig::default();
    ///     let task_context = TaskCtx { value: 42 };
    ///     AERUGO.create_tasklet_with_context(task_config, task, task_context, &TASK_STORAGE);
    ///     #
    ///     # assert!(TASK_STORAGE.is_initialized());
    ///
    ///     // Do something with the tasklet via handle.
    ///     let task_handle = TASK_STORAGE.create_handle();
    ///     #
    ///     # assert!(task_handle.is_some())
    /// }
    /// ```
    fn create_tasklet_with_context<T, C>(
        &'static self,
        config: Self::TaskConfig,
        step_fn: StepFn<T, C>,
        context: C,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error> {
        // SAFETY: This is safe as long as this function is called only during system initialization.
        unsafe { storage.init(config, step_fn, context) }
    }

    /// Creates new message queue in the system.
    ///
    /// Queue is created in the passed `storage` memory. Storage has to be static to keep the
    /// stored tasklet for the whole duration of system life.
    ///
    /// # Generic Arguments
    /// * `T` - Type of the data stored in the queue.
    /// * `N` - Size of the queue.
    ///
    /// # Arguments
    /// * `storage` - Static memory storage where the queue should be allocated.
    ///
    /// # Return
    /// `Error` in case of an error, `Ok(())` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because it initializes the
    /// passed storage which is safe only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{InitApi, MessageQueueStorage, AERUGO};
    /// static QUEUE_STORAGE: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();
    ///
    /// fn main() {
    ///     AERUGO.create_message_queue(&QUEUE_STORAGE);
    ///     #
    ///     # assert!(QUEUE_STORAGE.is_initialized());
    ///
    ///     // Do something with the queue via handle.
    ///     let queue_handle = QUEUE_STORAGE.create_handle();
    ///     #
    ///     # assert!(queue_handle.is_some())
    /// }
    /// ```
    fn create_message_queue<T, const N: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, N>,
    ) -> Result<(), Self::Error> {
        // SAFETY: This is safe as long as this function is called only during system initialization.
        unsafe { storage.init() }
    }

    fn create_event(&'static self, _storage: &'static EventStorage) -> Result<(), Self::Error> {
        todo!()
    }

    fn create_boolean_condition(
        &'static self,
        _storage: &'static BooleanConditionStorage,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    /// Subscribes a tasklet to a queue.
    ///
    /// Tasklet subscribes for a new data in this queue. Adding new data to the queue will wake up all
    /// subscribed tasklets and make them ready to be executed. Tasklet is ready for an execution for as
    /// long as there is some data in the queue. On each execution tasklet takes one element from
    /// the queue and performs user-specified processing on it.
    ///
    /// Each tasklet can be subscribed to at maximum one data provider. Queue can have multiple
    /// tasklets registered.
    ///
    /// Strong typing is enforced, tasklet can only be subscribed to a queue that stores the same
    /// type of data, that is processed by the tasklet.
    ///
    /// # Generic Arguments
    /// * `T` - Type of the data.
    /// * `C` - Type of the structure with tasklet context data.
    /// * `N` - Size of the queue.
    ///
    /// # Arguments
    /// * `tasklet` - Handle to the target tasklet.
    /// * `queue` - Handle to the target queue.
    ///
    /// # Return
    /// `Error` in case of an error, `Ok(())` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because subscription is safe
    /// only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{InitApi, MessageQueueStorage, TaskletConfig, TaskletStorage, AERUGO};
    /// #
    /// # fn task(_: u8, _: &mut ()) {}
    /// #
    /// # static TASK_STORAGE: TaskletStorage<u8, ()> = TaskletStorage::new();
    /// # static QUEUE_STORAGE: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();
    /// #
    /// fn main() {
    ///     # let task_config = TaskletConfig::default();
    ///     # AERUGO
    ///     #   .create_tasklet(TaskletConfig::default(), task, &TASK_STORAGE)
    ///     #   .expect("Unable to create Tasklet");
    ///     # AERUGO
    ///     #   .create_message_queue(&QUEUE_STORAGE)
    ///     #   .expect("Unable to create MessageQueue");
    ///     let task_handle = TASK_STORAGE.create_handle().expect("Failed to create Task handle");
    ///     let queue_handle = QUEUE_STORAGE.create_handle().expect("Failed to create Queue handle");
    ///
    ///     AERUGO
    ///         .subscribe_tasklet_to_queue(&task_handle, &queue_handle)
    ///         .expect("Failed to subscribe Task to Queue");
    /// }
    /// ```
    fn subscribe_tasklet_to_queue<T: Default, C, const N: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<T, C>,
        queue_handle: &MessageQueueHandle<T, N>,
    ) -> Result<(), Self::Error> {
        let tasklet = tasklet_handle.tasklet();
        let queue = queue_handle.queue();

        // SAFETY: This is safe as long as this function is called only during system initialization.
        unsafe {
            tasklet.subscribe(queue)?;
            queue.register_tasklet(tasklet.ptr())?;
        }

        Ok(())
    }

    fn subscribe_tasklet_to_event<T, C>(
        &'static self,
        _tasklet: &TaskletHandle<T, C>,
        _event: &EventHandle,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn subscribe_tasklet_to_conditions<T, C>(
        &'static self,
        _tasklet: &TaskletHandle<T, C>,
        _conditions: BooleanConditionSet,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    /// Subscribes tasklet to the cyclic execution.
    ///
    /// Tasklet subscribes for cyclic execution. Tasklet will be executed in specified period,
    /// or will be always ready for execution if such period was not specified. On execution
    /// tasklet won't receive any data, so cycling tasklets are useful mostly as ex. producers or
    /// some periodic housekeeping operations.
    ///
    /// Each tasklet can be subscribed to at maximum on data provider.
    ///
    /// # Generic Arguments
    /// * `C` - Type of the structure with tasklet context data.
    ///
    /// # Arguments
    /// * `tasklet` - Handle to the target tasklet.
    /// * `period` - Time period of the execution.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because subscription is safe
    /// only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{InitApi, TaskletConfig, TaskletStorage, AERUGO};
    /// #
    /// # fn task(_: (), _: &mut ()) {}
    /// #
    /// # static TASK_STORAGE: TaskletStorage<(), ()> = TaskletStorage::new();
    /// #
    /// fn main() {
    ///     # let task_config = TaskletConfig::default();
    ///     # AERUGO
    ///     #   .create_tasklet(TaskletConfig::default(), task, &TASK_STORAGE)
    ///     #   .expect("Unable to create Tasklet");
    ///     let task_handle = TASK_STORAGE.create_handle().expect("Failed to create Task handle");
    ///
    ///     AERUGO
    ///         .subscribe_tasklet_to_cyclic(&task_handle, None)
    ///         .expect("Failed to subscribe Task to cyclic execution");
    /// }
    /// ```
    fn subscribe_tasklet_to_cyclic<C>(
        &'static self,
        tasklet_handle: &TaskletHandle<(), C>,
        period: Option<Self::Duration>,
    ) -> Result<(), Self::Error> {
        let tasklet = tasklet_handle.tasklet();

        // SAFETY: This is safe as long as this function is called only during system initialization.
        unsafe {
            let cyclic_execution = TIME_MANAGER.create_cyclic_execution(tasklet.ptr(), period)?;
            tasklet.subscribe(cyclic_execution)?;
        }

        Ok(())
    }

    fn init_hardware(&'static self, _init_fn: fn(&mut Peripherals)) {
        todo!()
    }
}

impl RuntimeApi for Aerugo {
    type Instant = crate::time::TimerInstantU64<1_000_000>;
    type Duration = crate::time::TimerDurationU64<1_000_000>;

    fn get_system_time(&'static self) -> Self::Instant {
        // SAFETY: This is safe, because it's a single-core environment,
        // and no other references to Hal should exist during this call.
        unsafe {
            self.hal
                .as_ref()
                .as_ref()
                .expect("Tried to use HAL before Aerugo initialization!")
                .get_system_time()
        }
    }

    fn set_system_time_offset(&'static self, _offset: Self::Duration) {
        todo!()
    }

    fn query_tasks(&'static self) -> core::slice::Iter<TaskId> {
        todo!()
    }

    fn get_execution_statistics(&'static self, _task_id: TaskId) -> ExecutionStats {
        todo!()
    }

    fn enter_critical() {
        todo!()
    }

    fn exit_critical() {
        todo!()
    }

    fn execute_critical<F, R>(_f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        todo!()
    }
}

impl SystemApi for Aerugo {
    fn wake_tasklet(&'static self, tasklet: &TaskletPtr) {
        EXECUTOR
            .schedule_tasklet(tasklet)
            .expect("Unable to schedule tasklet");
    }

    fn update(&'static self) {
        TIME_MANAGER.wake_tasklets();
    }
}
