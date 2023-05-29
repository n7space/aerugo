//! Message queue used for exchanging data between tasklets.
mod message_queue_storage;

pub use self::message_queue_storage::MessageQueueStorage;

pub(crate) use self::message_queue_storage::QueueData;

use aerugo_cortex_m::Mutex;
use heapless::Vec;

use crate::aerugo::{
    error::{InitError, RuntimeError},
    Aerugo,
};
use crate::data_provider::DataProvider;
use crate::queue::Queue;
use crate::task::Task;

/// Message queue used for exchanging data between tasklets.
///
/// * `T` - Type of the stored data.
/// * `N` - Size of the queue.
pub(crate) struct MessageQueue<'a, T, const N: usize> {
    /// Reference to the queue data storage.
    _data: &'a Mutex<QueueData<T, N>>,
    /// System API.
    _system: &'static Aerugo,
}

impl<'a, T, const N: usize> Queue<T> for MessageQueue<'a, T, N> {
    fn register_task(&'static self, _task: &'static dyn Task) -> Result<(), InitError> {
        todo!()
    }

    fn get_registered_tasks(&'static self) -> &Vec<&'static dyn Task, 8> {
        todo!()
    }

    fn send_data(&'static self, _data: T) -> Result<(), RuntimeError> {
        todo!()
    }
}

impl<'a, T, const N: usize> DataProvider<T> for MessageQueue<'a, T, N> {
    fn data_ready(&'static self) -> bool {
        todo!()
    }

    fn get_data(&'static self) -> Option<T> {
        todo!()
    }

    fn get_data_unchecked(&'static self) -> T {
        todo!()
    }
}
