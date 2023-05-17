//! Generic queue.

mod queue_handle;

pub use self::queue_handle::QueueHandle;

use crate::aerugo::error::RuntimeError;
use crate::data_provider::DataProvider;

/// Trait for generic queue that stores data of type `T`.
///
/// * `T` - Type of the stored data.
pub(crate) trait Queue<T>: DataProvider<T> {
    /// Sends given data to this queue.
    ///
    /// * `data` - Data to send.
    ///
    /// Returns `RuntimeError` in case of an error, `Ok(())` otherwise.
    fn send_data(&'static self, data: T) -> Result<(), RuntimeError>;
}
