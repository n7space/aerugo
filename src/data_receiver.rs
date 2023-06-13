//! Trait with data receiver functionality.
//!
//! Data receiver is a structure that can read data from the
//! [data provider](crate::data_provider::DataProvider).

use crate::aerugo::error::InitError;
use crate::data_provider::DataProvider;

/// Trait with data receiver functionality.
///
/// * `T` - Type of the received data.
pub(crate) trait DataReceiver<T> {
    /// Subscribes itself to the given data provider.
    ///
    /// * `data_provider` - Data provider.
    ///
    /// Returns `InitError` in case of an error, `Ok(())` otherwise.
    fn subscribe(
        &'static self,
        data_provider: &'static dyn DataProvider<T>,
    ) -> Result<(), InitError>;
}
