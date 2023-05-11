//! TODO

use core::fmt::Debug;

use crate::queue::{MessageQueueStorage, QueueHandle};
use crate::task::{TaskHandle, TaskletStorage};

/// TODO
pub trait InitApi: ErrorType + TaskConfigType {
    /// TODO
    fn create_tasklet<T: Debug, C>(
        &'static self,
        config: Self::TaskConfig,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error>;

    /// TODO
    fn create_message_queue<T: Debug, const N: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, N>,
    ) -> Result<(), Self::Error>;

    /// TODO
    fn register_tasklet_to_queue<T: Debug>(
        &'static self,
        tasklet: &TaskHandle<T>,
        queue: &QueueHandle<T>,
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
pub trait TaskConfiguration: Default {}

/// Task configuration type trait
pub trait TaskConfigType {
    /// Task configuration type
    type TaskConfig: TaskConfiguration;
}
