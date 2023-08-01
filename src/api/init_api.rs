//! System initialization API.
//!
//! This API is used for the system initialization, before the scheduler is started.
//!
//! # Safety
//! Functions from this trait shouldn't be called after the system was started.

use crate::boolean_condition::{BooleanConditionSet, BooleanConditionStorage};
use crate::event::{EventHandle, EventStorage};
use crate::message_queue::{MessageQueueHandle, MessageQueueStorage};
use crate::tasklet::{StepFn, TaskletHandle, TaskletStorage};

/// System initialization API
pub trait InitApi: ErrorType + TaskConfigType {
    /// Type for a duration of time.
    type Duration;

    /// Creates new tasklet in the system.
    ///
    /// # Generic Parameters
    /// * `T` - Type of the data processed by the tasklet.
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    ///
    /// # Parameters
    /// * `config` - Tasklet creation configuration.
    /// * `step_fn` - Tasklet step function.
    /// * `storage` - Static memory storage where the tasklet should be allocated.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` otherwise.
    fn create_tasklet<T, C: Default, const COND_COUNT: usize>(
        &'static self,
        config: Self::TaskConfig,
        step_fn: StepFn<T, C>,
        storage: &'static TaskletStorage<T, C, COND_COUNT>,
    ) -> Result<(), Self::Error>;

    /// Creates new tasklet in the system with initialized context data.
    ///
    /// # Generic Parameters
    /// * `T` - Type of the data processed by the tasklet.
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    ///
    /// # Parameters
    /// * `config` - Tasklet creation configuration.
    /// * `step_fn` - Tasklet step function.
    /// * `context` - Tasklet context data.
    /// * `storage` - Static memory storage where the tasklet should be allocated.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` otherwise.
    fn create_tasklet_with_context<T, C, const COND_COUNT: usize>(
        &'static self,
        config: Self::TaskConfig,
        step_fn: StepFn<T, C>,
        context: C,
        storage: &'static TaskletStorage<T, C, COND_COUNT>,
    ) -> Result<(), Self::Error>;

    /// Creates new message queue in the system.
    ///
    /// # Generic Parameters
    /// * `T` - Type of the data stored in the queue.
    /// * `QUEUE_SIZE` - Size of the queue.
    ///
    /// # Parameters
    /// * `storage` - Static memory storage where the queue should be allocated.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` otherwise.
    fn create_message_queue<T, const QUEUE_SIZE: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, QUEUE_SIZE>,
    ) -> Result<(), Self::Error>;

    /// Creates new event in the system.
    ///
    /// # Parameters
    /// * `storage` - Static memory storage where the event should be allocated.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` otherwise.
    fn create_event(&'static self, storage: &'static EventStorage) -> Result<(), Self::Error>;

    /// Creates new boolean condition in the system.
    ///
    /// # Parameters
    /// * `storage` - Static memory storage where the condition should be allocated.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` otherwise.
    fn create_boolean_condition(
        &'static self,
        storage: &'static BooleanConditionStorage,
        value: bool,
    ) -> Result<(), Self::Error>;

    /// Subscribes tasklet to the queue.
    ///
    /// # Generic Parameters
    /// * `T` - Type of the data.
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    /// * `QUEUE_SIZE` - Size of the queue.
    ///
    /// # Parameters
    /// * `tasklet` - Handle to the target tasklet.
    /// * `queue` - Handle to the target queue.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` otherwise.
    fn subscribe_tasklet_to_queue<T: Default, C, const COND_COUNT: usize, const QUEUE_SIZE: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<T, C, COND_COUNT>,
        queue_handle: &MessageQueueHandle<T, QUEUE_SIZE>,
    ) -> Result<(), Self::Error>;

    /// Subscribes tasklet to the event.
    ///
    /// # Generic Parameters
    /// * `T` - Type of the data.
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    ///
    /// # Parameters
    /// * `tasklet` - Handle to the target tasklet.
    /// * `event` - Target event ID.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` otherwise.
    fn subscribe_tasklet_to_event<T, C, const COND_COUNT: usize>(
        &'static self,
        tasklet: &TaskletHandle<T, C, COND_COUNT>,
        event: &EventHandle,
    ) -> Result<(), Self::Error>;

    /// Subscribes tasklet to the cyclic execution.
    ///
    /// # Generic Parameters
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    ///
    /// # Parameters
    /// * `tasklet` - Handle to the target tasklet.
    /// * `period` - Time period of the execution.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` otherwise.
    fn subscribe_tasklet_to_cyclic<C, const COND_COUNT: usize>(
        &'static self,
        tasklet: &TaskletHandle<(), C, COND_COUNT>,
        period: Option<Self::Duration>,
    ) -> Result<(), Self::Error>;

    /// Sets tasklet condition set.
    ///
    /// # Generic Parameters
    /// * `T` - Type of the data.
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of conditions.
    ///
    /// # Parameters
    /// * `tasklet` - Handle to the target tasklet.
    /// * `condition` - Set of conditions.
    ///
    /// # Return
    /// `()` if successful, `Self::Error` otherwise.
    fn set_tasklet_conditions<T, C, const COND_COUNT: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<T, C, COND_COUNT>,
        condition_set: BooleanConditionSet<COND_COUNT>,
    ) -> Result<(), Self::Error>;
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
