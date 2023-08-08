//! Trait with data receiver functionality.
//!
//! This module contains a trait for a data receiver. Currently only [Tasklet](crate::tasklet::Tasklet)
//! is such a receiver.

use crate::api::InitError;
use crate::data_provider::DataProvider;

/// Trait with data receiver functionality.
///
/// Data receiver is a structure that can read data from the
/// [data provider](crate::data_provider::DataProvider).
///
/// # Generic Parameters
/// * `T` - Type of the received data.
pub(crate) trait DataReceiver<T> {
    /// Subscribes itself to the given data provider.
    ///
    /// # Parameters
    /// * `data_provider` - Data provider.
    ///
    /// # Return
    /// `InitError` in case of an error, `Ok(())` otherwise.
    unsafe fn subscribe(
        &self,
        data_provider: &'static dyn DataProvider<T>,
    ) -> Result<(), InitError>;
}
