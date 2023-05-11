//! TODO

pub mod error;

mod configuration;

use core::fmt::Debug;

use crate::api::{InitApi, RuntimeApi};
use crate::queue::{MessageQueueStorage, QueueHandle};
use crate::task::{TaskHandle, TaskletStorage};

pub struct Aerugo {}

impl Aerugo {
    pub const fn new() -> Self {
        Aerugo {}
    }

    pub fn start_scheduler(&self) -> ! {
        todo!()
    }
}

impl InitApi for Aerugo {
    fn create_tasklet<T: Debug, C>(
        &'static self,
        _config: Self::TaskConfig,
        _storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn create_message_queue<T: Debug, const N: usize>(
        &'static self,
        _storage: &'static MessageQueueStorage<T, N>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn register_tasklet_to_queue<T: Debug>(
        &'static self,
        _tasklet: &TaskHandle<T>,
        _queue: &QueueHandle<T>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

impl RuntimeApi for Aerugo {}
