//! Generic queue.

mod queue_handle;

pub use self::queue_handle::QueueHandle;

use heapless::Vec;

use crate::aerugo::error::{InitError, RuntimeError};
use crate::data_provider::DataProvider;
use crate::task::Task;

/// Trait for generic queue that stores data of type `T`.
///
/// * `T` - Type of the stored data.
pub(crate) trait Queue<T>: DataProvider<T> {
    /// Registers task to this queue.
    ///
    /// * `task` - Task to register.
    ///
    /// Returns `InitError` in case of an error, `Ok(())` otherwise.
    fn register_task(&'static self, task: &'static dyn Task) -> Result<(), InitError>;

    /// Gets tasks registered to this queue.
    ///
    /// Returns a list of references to the registered tasks.
    fn get_registered_tasks(&'static self) -> &Vec<&'static dyn Task, 8>;

    /// Sends given data to this queue.
    ///
    /// * `data` - Data to send.
    ///
    /// Returns `RuntimeError` in case of an error, `Ok(())` otherwise.
    fn send_data(&'static self, data: T) -> Result<(), RuntimeError>;
}
