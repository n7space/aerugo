//! TODO

use super::Queue;

use heapless::Vec;

use crate::aerugo::{
    error::{InitError, RuntimeError},
    Aerugo,
};
use crate::crit_cell::CritCell;
use crate::data_provider::DataProvider;
use crate::notifier::Notifier;
use crate::queue::QueueData;
use crate::task::Task;

/// TODO
pub(crate) struct MessageQueue<'a, T, const N: usize> {
    _data: &'a CritCell<QueueData<T, N>>,
    _system: &'static Aerugo,
}

impl<'a, T, const N: usize> MessageQueue<'a, T, N> {}

impl<'a, T, const N: usize> Queue<T> for MessageQueue<'a, T, N> {
    fn send_data(&'static self, _data: T) -> Result<(), RuntimeError> {
        todo!()
    }
}

impl<'a, T, const N: usize> Notifier for MessageQueue<'a, T, N> {
    fn register_tasklet(&'static self, _task: &'static dyn Task) -> Result<(), InitError> {
        todo!()
    }

    fn get_registered_tasklets(&'static self) -> &Vec<&'static dyn Task, 8> {
        todo!()
    }
}

impl<'a, T, const N: usize> DataProvider<T> for MessageQueue<'a, T, N> {
    fn data_ready(&'static self) -> bool {
        todo!()
    }

    fn get_data(&'static self) -> T {
        todo!()
    }
}
