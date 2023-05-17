//! Trait with data provider functionality.
//!
//! Data provider is a structure that provided some kind of data to the
//! [data receiver](crate::data_receiver::DataReceiver).

use heapless::Vec;

use crate::aerugo::error::InitError;
use crate::task::Task;

/// Trait with data provider functionality.
///
/// * `T` - Type of the provided data.
pub(crate) trait DataProvider<T> {
    /// Registers task to this provider.
    ///
    /// * `task` - Task to register.
    ///
    /// Returns `InitError` in case of an error, `Ok(())` otherwise.
    fn register_task(&'static self, task: &'static dyn Task) -> Result<(), InitError>;

    /// Gets tasks registered to this provider.
    ///
    /// Returns a list of referenced to the registered tasks.
    fn get_registered_tasks(&'static self) -> &Vec<&'static dyn Task, 8>;

    /// Checks whether there is data available for reading.
    ///
    /// Returns `true` if there is data available, `false` otherwise.
    fn data_ready(&'static self) -> bool;

    /// Provides data.
    ///
    /// Returns `Some(T)` if there was data available, `None` otherwise.
    fn get_data(&'static self) -> Option<T>;

    /// Provides data without checking.
    ///
    /// Returns data.
    ///
    /// # Panic
    /// Panics if no data was available.
    fn get_data_unchecked(&'static self) -> T;
}
