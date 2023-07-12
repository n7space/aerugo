//! Generic queue.

use crate::aerugo::error::{InitError, RuntimeError};
use crate::data_provider::DataProvider;
use crate::tasklet::TaskletPtr;

/// Trait for generic queue that stores data of type `T`.
///
/// * `T` - Type of the stored data.
pub(crate) trait Queue<T>: DataProvider<T> {
    /// Registers task to this queue.
    ///
    /// * `task` - Task to register.
    ///
    /// Returns `InitError` in case of an error, `Ok(())` otherwise.
    fn register_tasklet(&self, tasklet: TaskletPtr) -> Result<(), InitError>;

    /// Sends given data to this queue.
    ///
    /// * `data` - Data to send.
    ///
    /// Returns `RuntimeError` in case of an error, `Ok(())` otherwise.
    fn send_data(&self, data: T) -> Result<(), RuntimeError>;
}
