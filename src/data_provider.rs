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
