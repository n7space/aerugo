//! TODO

use heapless::Vec;

use crate::crit_cell::CritCell;
use crate::internal_cell::InternalCell;
use crate::queue::{MessageQueue, QueueHandle};

pub(crate) type QueueData<T, const N: usize> = heapless::spsc::Queue<T, N>;
pub(crate) type QueueBuffer = Vec<u8, { core::mem::size_of::<MessageQueue<(), 0>>() }>;

/// TODO
pub struct MessageQueueStorage<T, const N: usize> {
    _initialized: InternalCell<bool>,
    _queue_buffer: InternalCell<QueueBuffer>,
    _queue_data: CritCell<QueueData<T, N>>,
}

impl<T, const N: usize> MessageQueueStorage<T, N> {
    /// TODO
    pub const fn new() -> Self {
        MessageQueueStorage {
            _initialized: InternalCell::new(false),
            _queue_buffer: InternalCell::new(QueueBuffer::new()),
            _queue_data: CritCell::new(QueueData::new()),
        }
    }

    /// TODO
    pub fn create_queue_handle(&'static self) -> Option<QueueHandle<T>> {
        todo!()
    }
}
