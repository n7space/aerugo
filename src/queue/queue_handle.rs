//! TODO

use crate::aerugo::error::RuntimeError;
use crate::data_provider::DataProvider;
use crate::notifier::Notifier;
use crate::queue::Queue;

/// TODO
#[derive(Copy, Clone)]
pub struct QueueHandle<T: 'static> {
    queue: &'static dyn Queue<T>,
    _notifier: &'static dyn Notifier,
    _data_provider: &'static dyn DataProvider<T>,
}

impl<T> QueueHandle<T> {
    /// TODO
    #[inline(always)]
    pub fn send_data(&self, data: T) -> Result<(), RuntimeError> {
        self.queue.send_data(data)
    }
}
