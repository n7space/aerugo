//! Trait with data provider functionality.
//!
//! Data provider is a structure that provided some kind of data to the
//! [data receiver](crate::data_receiver::DataReceiver).

/// Trait with data provider functionality.
///
/// * `T` - Type of the provided data.
pub(crate) trait DataProvider<T> {
    /// Checks whether there is data available for reading.
    ///
    /// Returns `true` if there is data available, `false` otherwise.
    fn data_ready(&self) -> bool;

    /// Provides data.
    ///
    /// Returns `Some(T)` if there was data available, `None` otherwise.
    fn get_data(&self) -> Option<T>;
}
