/// Handle to a queue.
///
/// Handle is used in the system to access a queue.
use crate::aerugo::error::RuntimeError;
use crate::data_provider::DataProvider;
use crate::queue::Queue;

/// Queue handle.
///
/// * `T` - Type that is stored by the queue.
#[derive(Copy, Clone)]
pub struct QueueHandle<T: 'static> {
    /// Reference to the queue.
    queue: &'static dyn Queue<T>,
    /// Reference to the queue as a DataProvider.
    _data_provider: &'static dyn DataProvider<T>,
}

impl<T> QueueHandle<T> {
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
