//! Handle to a queue.
//!
//! This module contains queue handle implementation, which is used to reference a queue in the
//! system.

use crate::error::RuntimeError;
use crate::message_queue::MessageQueue;

/// Message queue handle.
///
/// Queue handle is available to the user of the system to reference and interact with the queue
/// via exposed interface. All system API functions shall use handles when a reference to queue is
/// required.
///
/// # Generic Parameters
/// * `T` - Type that is stored by the queue.
#[derive(Copy, Clone)]
pub struct MessageQueueHandle<T: 'static, const N: usize> {
    /// Reference to the queue.
    queue: &'static MessageQueue<T, N>,
}

impl<T, const N: usize> MessageQueueHandle<T, N> {
    /// Creates new queue handle.
    ///
    /// # Parameters
    /// * `queue` - Reference to the queue.
    pub(crate) fn new(queue: &'static MessageQueue<T, N>) -> Self {
        MessageQueueHandle { queue }
    }

    /// Send data to the stored queue.
    ///
    /// # Parameters
    /// * `data` - Data to send.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    #[inline(always)]
    pub fn send_data(&self, data: T) -> Result<(), RuntimeError> {
        self.queue.send_data(data)
    }

    /// Returns reference to the queue.
    pub(crate) fn queue(&self) -> &'static MessageQueue<T, N> {
        self.queue
    }
}
