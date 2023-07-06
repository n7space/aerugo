//! Message queue used for exchanging data between tasklets.
mod message_queue_handle;
mod message_queue_storage;

pub use self::message_queue_handle::MessageQueueHandle;
pub use self::message_queue_storage::MessageQueueStorage;

pub(crate) use self::message_queue_storage::QueueData;

use heapless::Vec;

use crate::aerugo::error::{InitError, RuntimeError};
use crate::arch::Mutex;
use crate::data_provider::DataProvider;
use crate::queue::Queue;
use crate::task::Task;

/// Message queue used for exchanging data between tasklets.
///
/// * `T` - Type of the stored data.
/// * `N` - Size of the queue.
pub(crate) struct MessageQueue<T: 'static, const N: usize> {
    /// Reference to the queue data storage.
    data_queue: &'static Mutex<QueueData<T, N>>,
}

impl<T, const N: usize> MessageQueue<T, N> {
    /// Creates new `MessageQueue`.
    pub(crate) fn new(data_queue: &'static Mutex<QueueData<T, N>>) -> Self {
        MessageQueue { data_queue }
    }
}

impl<T, const N: usize> Queue<T> for MessageQueue<T, N> {
    fn register_task(&self, _task: &'static dyn Task) -> Result<(), InitError> {
        todo!()
    }

    fn get_registered_tasks(&self) -> &Vec<&'static dyn Task, 8> {
        todo!()
    }

    fn send_data(&self, data: T) -> Result<(), RuntimeError> {
        match self.data_queue.lock(|q| q.enqueue(data)) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(RuntimeError::DataQueueFull),
        }
    }
}

impl<T, const N: usize> DataProvider<T> for MessageQueue<T, N> {
    fn data_ready(&self) -> bool {
        todo!()
    }

    fn get_data(&self) -> Option<T> {
        todo!()
    }

    fn get_data_unchecked(&self) -> T {
        todo!()
    }
}
