//! System initialization API.
//!
//! This API is used for the system initialization, before the scheduler is started.
//!
//! # Safety
//! Functions from this trait shouldn't be called after the system was started.

use crate::api::InitError;
use crate::boolean_condition::{
    BooleanConditionHandle, BooleanConditionSet, BooleanConditionStorage,
};
use crate::event::{EventEnabler, EventId};
use crate::message_queue::{MessageQueueHandle, MessageQueueStorage};
use crate::tasklet::{StepFn, TaskletConfig, TaskletHandle, TaskletStorage};

/// System initialization API
pub trait InitApi {
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
    /// `()` if successful, `InitError` otherwise.
    fn create_tasklet<T, C: Default, const COND_COUNT: usize>(
        &'static self,
        config: TaskletConfig,
        step_fn: StepFn<T, C>,
        storage: &'static TaskletStorage<T, C, COND_COUNT>,
    ) -> Result<(), InitError>;

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
    /// `()` if successful, `InitError` otherwise.
    fn create_tasklet_with_context<T, C, const COND_COUNT: usize>(
        &'static self,
        config: TaskletConfig,
        step_fn: StepFn<T, C>,
        context: C,
        storage: &'static TaskletStorage<T, C, COND_COUNT>,
    ) -> Result<(), InitError>;

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
    /// `()` if successful, `InitError` otherwise.
    fn create_message_queue<T, const QUEUE_SIZE: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, QUEUE_SIZE>,
    ) -> Result<(), InitError>;

    /// Creates new event in the system.
    ///
    /// # Parameters
    /// * `event_id` - Identifier of this event.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    fn create_event(&'static self, event_id: EventId) -> Result<(), InitError>;

    /// Creates new boolean condition in the system.
    ///
    /// # Parameters
    /// * `storage` - Static memory storage where the condition should be allocated.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    fn create_boolean_condition(
        &'static self,
        storage: &'static BooleanConditionStorage,
        value: bool,
    ) -> Result<(), InitError>;

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
    /// `()` if successful, `InitError` otherwise.
    fn subscribe_tasklet_to_queue<T: Default, C, const COND_COUNT: usize, const QUEUE_SIZE: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<T, C, COND_COUNT>,
        queue_handle: &MessageQueueHandle<T, QUEUE_SIZE>,
    ) -> Result<(), InitError>;

    /// Subscribes tasklet to the event.
    ///
    /// # Generic Parameters
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    ///
    /// # Parameters
    /// * `tasklet_handle` - Handle to the target tasklet.
    ///
    /// # Return
    /// `EventEnabler` if successful, `InitError` otherwise.
    fn subscribe_tasklet_to_events<C, const COND_COUNT: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<EventId, C, COND_COUNT>,
    ) -> Result<EventEnabler, InitError>;

    /// Subscribes tasklet to the boolean condition.
    ///
    /// # Generic Parameters
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    ///
    /// # Parameters
    /// * `tasklet` - Handle to the target tasklet.
    /// * `condition` - Handle to the target condition.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    fn subscribe_tasklet_to_condition<C, const COND_COUNT: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<bool, C, COND_COUNT>,
        condition_handle: &BooleanConditionHandle,
    ) -> Result<(), InitError>;

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
    /// `()` if successful, `InitError` otherwise.
    fn subscribe_tasklet_to_cyclic<C, const COND_COUNT: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<(), C, COND_COUNT>,
        period: Option<crate::time::MillisDurationU32>,
    ) -> Result<(), InitError>;

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
    /// `()` if successful, `InitError` otherwise.
    fn set_tasklet_conditions<T, C, const COND_COUNT: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<T, C, COND_COUNT>,
        condition_set: BooleanConditionSet<COND_COUNT>,
    ) -> Result<(), InitError>;
}
