//! Trait with data provider functionality.
//!
//! This module contains a trait for a data provider. In the system, data providers are structures
//! that store the data that can be then passed to a data receiver (currently only
//! [Tasklet](crate::tasklet::Tasklet) is such a receiver).

/// Trait with data provider functionality.
///
/// Data provider is a structure that provides some kind of data to the
/// [data receiver](crate::data_receiver::DataReceiver).
///
/// # Generic Parameters
/// * `T` - Type of the provided data.
pub(crate) trait DataProvider<T> {
    /// Provides data.
    ///
    /// # Return
    /// `Some(T)` if there was data available, `None` otherwise.
    fn get_data(&self) -> Option<T>;

    /// Checks if there is data waiting for being handled.
    fn data_waiting(&self) -> bool;
}
