//! Generic queue.

use crate::aerugo::error::{InitError, RuntimeError};
use crate::data_provider::DataProvider;
use crate::tasklet::TaskletPtr;

/// Trait for generic queue that stores data of type `T`.
///
/// # Generic Parameters
/// * `T` - Type of the stored data.
pub(crate) trait Queue<T>: DataProvider<T> {
    /// Registers task to this queue.
    ///
    /// # Parameters
    /// * `task` - Task to register.
    ///
    /// # Returns
    /// `()` if successful, `InitError` otherwise.
    unsafe fn register_tasklet(&self, tasklet: TaskletPtr) -> Result<(), InitError>;

    /// Wakes tasklets registered to this queue.
    fn wake_tasklets(&self);

    /// Sends given data to this queue.
    ///
    /// # Parameters
    /// * `data` - Data to send.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    fn send_data(&self, data: T) -> Result<(), RuntimeError>;
}
