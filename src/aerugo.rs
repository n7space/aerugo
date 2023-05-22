//! System control and interface for the user to interact with it.

pub mod error;

mod configuration;

use crate::api::{InitApi, RuntimeApi};
use crate::boolean_condition::{BooleanConditionSet, BooleanConditionStorage};
use crate::event::EventId;
use crate::message_queue::MessageQueueStorage;
use crate::queue::QueueHandle;
use crate::task::TaskHandle;
use crate::tasklet::TaskletStorage;

/// System structure.
pub struct Aerugo {}

impl Aerugo {
    /// Creates new system instance.
    pub const fn new() -> Self {
        Aerugo {}
    }

    /// Starts system scheduler.
    pub fn start_scheduler(&self) -> ! {
        todo!()
    }
}

impl InitApi for Aerugo {
    fn create_tasklet<T, C>(
        &'static self,
        _config: Self::TaskConfig,
        _storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn create_message_queue<T, const N: usize>(
        &'static self,
        _storage: &'static MessageQueueStorage<T, N>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn create_boolean_condition(
        &'static self,
        _storage: &'static BooleanConditionStorage,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn subscribe_tasklet_to_queue<T>(
        &'static self,
        _tasklet: &TaskHandle<T>,
        _queue: &QueueHandle<T>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn subscribe_tasklet_to_event<T>(
        &'static self,
        _tasklet: &TaskHandle<T>,
        _event: EventId,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn subscribe_tasklet_to_conditions<T>(
        &'static self,
        _tasklet: &TaskHandle<T>,
        _conditions: BooleanConditionSet,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

impl RuntimeApi for Aerugo {}