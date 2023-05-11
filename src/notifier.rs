//! TODO

use heapless::Vec;

use crate::aerugo::error::InitError;
use crate::task::Task;

/// TODO
pub(crate) trait Notifier {
    /// TODO
    fn register_tasklet(&'static self, task: &'static dyn Task) -> Result<(), InitError>;

    /// TODO
    fn get_registered_tasklets(&'static self) -> &Vec<&'static dyn Task, 8>;
}
