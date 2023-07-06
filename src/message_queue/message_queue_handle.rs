//! Handle to a queue.
//!
//! Handle is used in the system to access a queue.
use crate::aerugo::error::RuntimeError;
use crate::message_queue::MessageQueue;
use crate::queue::Queue;

/// Message queue handle.
///
/// * `T` - Type that is stored by the queue.
#[derive(Copy, Clone)]
pub struct MessageQueueHandle<T: 'static, const N: usize> {
    /// Reference to the queue.
    queue: &'static MessageQueue<T, N>,
}

impl<T, const N: usize> MessageQueueHandle<T, N> {
    /// Creates new queue handle.
    ///
    /// * `queue` - Reference to the queue.
    pub(crate) fn new(queue: &'static MessageQueue<T, N>) -> Self {
        MessageQueueHandle { queue }
    }

    /// Send data to the stored queue.
    ///
    /// * `data` - Data to send.
    ///
    /// Returns `RuntimeError` in case of an error, `Ok(())` otherwise.
    #[inline(always)]
    pub fn send_data(&self, data: T) -> Result<(), RuntimeError> {
        self.queue.send_data(data)
    }
}
