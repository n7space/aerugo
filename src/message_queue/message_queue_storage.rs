//! Static storage for [message queue](crate::message_queue::MessageQueue).
//!
//! As this system cannot use dynamic memory allocation, all structures have to be allocated
//! statically. Per good practices user is separated from the actual implementation and instead
//! only has to provide a static memory (via this structure) where the MessageQueue will be allocated.

use super::MessageQueue;

use heapless::Vec;

use crate::arch::Mutex;
use crate::internal_cell::InternalCell;
use crate::message_queue::MessageQueueHandle;

/// Type of the queue buffer storage.
pub(crate) type QueueBuffer = Vec<u8, { core::mem::size_of::<MessageQueue<(), 0>>() }>;
/// Type of the queue data storage.
pub(crate) type QueueData<T, const N: usize> = heapless::spsc::Queue<T, N>;

/// Structure containing memory for MessageQueue creation.
///
/// * `T` - Type of the stored data.
/// * `N` - Size of the queue.
pub struct MessageQueueStorage<T, const N: usize> {
    /// Marks whether this storage has been initialized.
    _initialized: InternalCell<bool>,
    /// Buffer for the queue structure.
    _queue_buffer: InternalCell<QueueBuffer>,
    /// Buffer for the queue data.
    _queue_data: Mutex<QueueData<T, N>>,
}

impl<T, const N: usize> MessageQueueStorage<T, N> {
    /// Creates new storage.
    pub const fn new() -> Self {
        MessageQueueStorage {
            _initialized: InternalCell::new(false),
            _queue_buffer: InternalCell::new(QueueBuffer::new()),
            _queue_data: Mutex::new(QueueData::new()),
        }
    }

    /// Creates new handle to a queue allocated in this storage.
    ///
    /// Returns `Some(handle)` if this storage has been initialized, `None` otherwise.
    pub fn create_queue_handle(&'static self) -> Option<MessageQueueHandle<T>> {
        todo!()
    }
}
