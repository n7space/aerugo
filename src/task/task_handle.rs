/// Handle to a task.
///
/// Handle is used in the system to reference a task. It's created from the adequate storage.

use crate::data_receiver::DataReceiver;
use crate::task::Task;

/// Task handle.
///
/// * `T` - Type that is processed by the task.
pub struct TaskHandle<T: 'static> {
    /// Reference to the task.
    _task: &'static dyn Task,
    /// Reference to the task as a DataReceiver.
    _data_receiver: &'static dyn DataReceiver<T>,
}
