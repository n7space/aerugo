//! System control and interface for the user to interact with it.

pub mod error;

pub use self::error::InitError;

use aerugo_hal::system_hal::SystemHal;
use bare_metal::CriticalSection;
use env_parser::read_env;

use crate::api::{InitApi, RuntimeApi};
use crate::boolean_condition::{BooleanConditionSet, BooleanConditionStorage};
use crate::data_receiver::DataReceiver;
use crate::event::{EventHandle, EventStorage};
use crate::execution_monitoring::ExecutionStats;
use crate::executor::Executor;
use crate::hal::{Hal, Peripherals};
use crate::message_queue::{MessageQueueHandle, MessageQueueStorage};
use crate::queue::Queue;
use crate::task::TaskId;
use crate::tasklet::{StepFn, TaskletHandle, TaskletStorage};

/// Core system.
pub static AERUGO: Aerugo = Aerugo::new();

/// System scheduler.
static EXECUTOR: Executor = Executor::new(&AERUGO);

/// System structure.
pub struct Aerugo {
    /// Hardware Access Layer.
    hal: Hal,
}

impl Aerugo {
    /// Maximum number of tasklets registered in the system.
    #[read_env("AERUGO_TASKLET_COUNT")]
    pub(crate) const TASKLET_COUNT: usize = 0;

    /// Starts the system.
    pub fn start(&'static self) -> ! {
        EXECUTOR.run_scheduler()
    }

    /// Creates new system instance.
    pub(crate) const fn new() -> Self {
        let peripherals = Peripherals {};

        Aerugo {
            hal: Hal::new(peripherals),
        }
    }
}

impl InitApi for Aerugo {
    type Duration = crate::time::MillisDurationU32;

    fn create_tasklet<T, C: Default>(
        &'static self,
        config: Self::TaskConfig,
        step_fn: StepFn<T, C>,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error> {
        storage.init(config, step_fn, C::default())?;

        Ok(())
    }

    fn create_tasklet_with_context<T, C>(
        &'static self,
        config: Self::TaskConfig,
        step_fn: StepFn<T, C>,
        context: C,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error> {
        storage.init(config, step_fn, context)?;

        Ok(())
    }

    fn create_message_queue<T, const N: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, N>,
    ) -> Result<(), Self::Error> {
        storage.init()?;

        Ok(())
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

    fn subscribe_tasklet_to_queue<T: Default, C, const N: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<T, C>,
        queue_handle: &MessageQueueHandle<T, N>,
    ) -> Result<(), Self::Error> {
        let tasklet = tasklet_handle.tasklet();
        let queue = queue_handle.queue();

        tasklet.subscribe(queue)?;
        queue.register_tasklet(tasklet.ptr())?;

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

    fn subscribe_tasklet_to_cyclic<T, C>(
        &'static self,
        _tasklet: &TaskletHandle<T, C>,
        _period: Self::Duration,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn init_hardware(&'static self, _init_fn: fn(&mut Peripherals)) {
        todo!()
    }
}

impl RuntimeApi for Aerugo {
    type Instant = crate::time::TimerInstantU64<1_000_000>;
    type Duration = crate::time::TimerDurationU64<1_000_000>;

    fn get_system_time(&'static self) -> Self::Instant {
        self.hal.get_system_time()
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
