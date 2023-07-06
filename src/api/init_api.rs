/// System initialization API.
///
/// This API is used for the system initialization, before the scheduler is started.
use crate::boolean_condition::{BooleanConditionSet, BooleanConditionStorage};
use crate::event::{EventHandle, EventStorage};
use crate::hal::Peripherals;
use crate::message_queue::{MessageQueueHandle, MessageQueueStorage};
use crate::tasklet::{StepFn, TaskletHandle, TaskletStorage};

/// System initialization API
pub trait InitApi: ErrorType + TaskConfigType {
    /// Type for a duration of time.
    type Duration;

    /// Creates new tasklet in the system.
    ///
    /// * `T` - Type of the data processed by the tasklet.
    /// * `C` - Type of the structure with tasklet context data.
    ///
    /// * `config` - Tasklet creation configuration.
    /// * `step_fn` - Tasklet step function.
    /// * `storage` - Static memory storage where the tasklet should be allocated.
    ///
    /// Returns `Error` in case of an error, `Ok(())` otherwise.
    fn create_tasklet<T: Default, C: Default>(
        &'static self,
        config: Self::TaskConfig,
        step_fn: StepFn<T, C>,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error>;

    /// Creates new tasklet in the system with initialized context data.
    ///
    /// * `T` - Type of the data processed by the tasklet.
    /// * `C` - Type of the structure with tasklet context data.
    ///
    /// * `config` - Tasklet creation configuration.
    /// * `step_fn` - Tasklet step function.
    /// * `context` - Tasklet context data.
    /// * `storage` - Static memory storage where the tasklet should be allocated.
    ///
    /// Returns `Error` in case of an error, `Ok(())` otherwise.
    fn create_tasklet_with_context<T: Default, C>(
        &'static self,
        config: Self::TaskConfig,
        step_fn: StepFn<T, C>,
        context: C,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error>;

    /// Creates new message queue in the system.
    ///
    /// * `T` - Type of the data stored in the queue.
    /// * `N` - Size of the queue.
    ///
    /// * `storage` - Static memory storage where the queue should be allocated.
    ///
    /// Returns `Error` in case of an error, `Ok(())` otherwise.
    fn create_message_queue<T, const N: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, N>,
    ) -> Result<(), Self::Error>;

    /// Creates new event in the system.
    ///
    /// * `storage` - Static memory storage where the event should be allocated.
    ///
    /// Returns `Error` in case of an error, `Ok(())` otherwise.
    fn create_event(&'static self, storage: &'static EventStorage) -> Result<(), Self::Error>;

    /// Creates new boolean condition in the system.
    ///
    /// * `storage` - Static memory storage where the condition should be allocated.
    ///
    /// Returns `Error` in case of an error, `Ok(())` otherwise.
    fn create_boolean_condition(
        &'static self,
        storage: &'static BooleanConditionStorage,
    ) -> Result<(), Self::Error>;

    /// Subscribes tasklet to the queue.
    ///
    /// * `T` - Type of the data.
    ///
    /// * `tasklet` - Handle to the target tasklet.
    /// * `queue` - Handle to the target queue.
    ///
    /// Returns `Error` in case of an error, `Ok(())` otherwise.
    fn subscribe_tasklet_to_queue<T: Default, C, const N: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<T, C>,
        queue_handle: &MessageQueueHandle<T, N>,
    ) -> Result<(), Self::Error>;

    /// Subscribes tasklet to the event.
    ///
    /// * `tasklet` - Handle to the target tasklet.
    /// * `event` - Target event ID.
    ///
    /// Returns `Error` in case of an error, `Ok(())` otherwise.
    fn subscribe_tasklet_to_event<T, C>(
        &'static self,
        tasklet: &TaskletHandle<T, C>,
        event: &EventHandle,
    ) -> Result<(), Self::Error>;

    /// Subscribes tasklet to the set of conditions.
    ///
    /// * `tasklet` - Handle to the target tasklet.
    /// * `condition` - Set of conditions.
    ///
    /// Returns `Error` in case of an error, `Ok(())` otherwise.
    fn subscribe_tasklet_to_conditions<T, C>(
        &'static self,
        tasklet: &TaskletHandle<T, C>,
        conditions: BooleanConditionSet,
    ) -> Result<(), Self::Error>;

    /// Subscribes tasklet to the cyclic execution.
    ///
    /// * `tasklet` - Handle to the target tasklet.
    /// * `period` - Time period of the execution.
    ///
    /// Returns `Error` in case of an error, `Ok(())` otherwise.
    fn subscribe_tasklet_to_cyclic<T, C>(
        &'static self,
        tasklet: &TaskletHandle<T, C>,
        period: Self::Duration,
    ) -> Result<(), Self::Error>;

    /// Set function for hardware initialization
    ///
    /// * `init_fn` - Hardware initialization function.
    fn init_hardware(&'static self, init_fn: fn(&mut Peripherals));
}

/// Initialization error
pub trait Error: core::fmt::Debug {}

/// Initialization error type trait
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}

/// Configuration used for creating tasklets
pub trait TaskConfig: Default {}

/// Task configuration type trait
pub trait TaskConfigType {
    /// Task configuration type
    type TaskConfig: TaskConfig;
}
