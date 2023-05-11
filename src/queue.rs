//! TODO

mod message_queue;
mod message_queue_storage;
mod queue_handle;

pub(crate) use self::message_queue::MessageQueue;
pub(crate) use self::message_queue_storage::QueueData;

pub use self::message_queue_storage::MessageQueueStorage;
pub use self::queue_handle::QueueHandle;

use crate::aerugo::error::RuntimeError;
use crate::data_provider::DataProvider;
use crate::notifier::Notifier;

/// TODO
pub(crate) trait Queue<T>: Notifier + DataProvider<T> {
    /// TODO
    fn send_data(&'static self, data: T) -> Result<(), RuntimeError>;
}
