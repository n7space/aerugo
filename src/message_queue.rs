//! Message queue used for exchanging data between tasklets.
mod message_queue_handle;
mod message_queue_storage;

pub use self::message_queue_handle::MessageQueueHandle;
pub use self::message_queue_storage::MessageQueueStorage;

pub(crate) use self::message_queue_storage::QueueData;

use heapless::Vec;

use crate::aerugo::AERUGO;
use crate::aerugo::{
    error::{InitError, RuntimeError},
    Aerugo,
};
use crate::api::SystemApi;
use crate::arch::Mutex;
use crate::data_provider::DataProvider;
use crate::internal_cell::InternalCell;
use crate::queue::Queue;
use crate::tasklet::TaskletPtr;

/// List of tasklets registered to a queue
type TaskletList = Vec<TaskletPtr, { Aerugo::TASKLET_COUNT }>;

/// Message queue used for exchanging data between tasklets.
///
/// * `T` - Type of the stored data.
/// * `N` - Size of the queue.
#[repr(C)]
pub(crate) struct MessageQueue<T: 'static, const N: usize> {
    /// Reference to the queue data storage.
    data_queue: &'static Mutex<QueueData<T, N>>,
    /// Tasklets registered to this queue.
    registered_tasklets: InternalCell<TaskletList>,
}

impl<T, const N: usize> MessageQueue<T, N> {
    /// Creates new `MessageQueue`.
    pub(crate) fn new(data_queue: &'static Mutex<QueueData<T, N>>) -> Self {
        MessageQueue {
            data_queue,
            registered_tasklets: TaskletList::new().into(),
        }
    }
}

impl<T, const N: usize> Queue<T> for MessageQueue<T, N> {
    fn register_tasklet(&self, tasklet: TaskletPtr) -> Result<(), InitError> {
        // SAFETY: This is safe, because this is only mutably referenced here and no external
        // references can be made.
        match unsafe { self.registered_tasklets.as_mut_ref().push(tasklet) } {
            Ok(_) => Ok(()),
            Err(_) => Err(InitError::TaskletListFull),
        }
    }

    fn wake_tasklets(&self) {
        for t in unsafe { self.registered_tasklets.as_ref() } {
            AERUGO.wake_tasklet(t);
        }
    }

    fn send_data(&self, data: T) -> Result<(), RuntimeError> {
        match self.data_queue.lock(|q| q.enqueue(data)) {
            Ok(_) => (),
            Err(_) => return Err(RuntimeError::DataQueueFull),
        };

        self.wake_tasklets();

        Ok(())
    }
}

impl<T, const N: usize> DataProvider<T> for MessageQueue<T, N> {
    fn data_ready(&self) -> bool {
        self.data_queue.lock(|q| !q.is_empty())
    }

    fn get_data(&self) -> Option<T> {
        self.data_queue.lock(|q| q.dequeue())
    }
}
