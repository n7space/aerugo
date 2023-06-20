//! Unit of computation in the system.
//!
//! Tasklet is a fine-grained units of computation, that execute a processing step in a finite
//! amount of time.
//!
//! Tasklet should be thought of as a small building block, which processes a given type of data,
//! one element at the time.
mod tasklet_storage;

pub use self::tasklet_storage::TaskletStorage;

use core::marker::PhantomData;

use crate::aerugo::error::InitError;
use crate::data_provider::DataProvider;
use crate::data_receiver::DataReceiver;
use crate::internal_cell::InternalCell;
use crate::task::Task;

/// Tasklet structure.
pub(crate) struct Tasklet<T: 'static, C> {
    /// Source of the data.
    _data_provider: InternalCell<Option<&'static dyn DataProvider<T>>>,
    /// Marker for tasklet context data type.
    _context_type_marker: PhantomData<C>,
}

impl<T, C> Task for Tasklet<T, C> {
    fn is_ready(&self) -> bool {
        todo!()
    }

    fn execute(&self) {
        todo!()
    }
}

impl<T, C> DataReceiver<T> for Tasklet<T, C> {
    fn subscribe(
        &'static self,
        _data_provider: &'static dyn DataProvider<T>,
    ) -> Result<(), InitError> {
        todo!()
    }
}
