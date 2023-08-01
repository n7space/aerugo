//! Generic queue.

use crate::aerugo::error::{InitError, RuntimeError};
use crate::data_provider::DataProvider;
use crate::tasklet::TaskletPtr;

/// Trait for generic queue that stores data of type `T`.
///
/// # Generic Parameters
/// * `T` - Type of the stored data.
pub(crate) trait Queue<T>: DataProvider<T> {
    unsafe fn register_tasklet(&self, tasklet: TaskletPtr) -> Result<(), InitError>;

    fn wake_tasklets(&self);

    fn send_data(&self, data: T) -> Result<(), RuntimeError>;
}
