//! TODO

use super::Task;

use core::fmt::Debug;
use core::marker::PhantomData;

use crate::aerugo::error::InitError;
use crate::data_provider::DataProvider;
use crate::data_receiver::DataReceiver;
use crate::internal_cell::InternalCell;

/// TODO
pub(crate) struct Tasklet<T: 'static + Debug, C> {
    _data_provider: InternalCell<Option<&'static dyn DataProvider<T>>>,
    _context_type_marker: PhantomData<C>,
}

impl<T: Debug, C> Tasklet<T, C> {}

impl<T: Debug, C> Task for Tasklet<T, C> {
    fn is_ready(&self) -> bool {
        todo!()
    }

    fn execute(&self) {
        todo!()
    }
}

impl<T: Debug, C> DataReceiver<T> for Tasklet<T, C> {
    fn set_data_provider(
        &'static self,
        _data_provider: &'static dyn DataProvider<T>,
    ) -> Result<(), InitError> {
        todo!()
    }
}
