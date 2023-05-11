//! TODO

/// TODO
pub(crate) trait DataProvider<T> {
    /// TODO
    fn data_ready(&'static self) -> bool;

    /// TODO
    fn get_data(&'static self) -> T;
}
