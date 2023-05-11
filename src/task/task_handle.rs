//! TODO

use core::fmt::Debug;

use crate::data_receiver::DataReceiver;
use crate::task::Task;

/// TODO
pub struct TaskHandle<T: 'static + Debug> {
    _task: &'static dyn Task,
    _data_receiver: &'static dyn DataReceiver<T>,
}

impl<T: Debug> TaskHandle<T> {}
