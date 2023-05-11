//! TODO

use crate::aerugo::error::InitError;
use crate::data_provider::DataProvider;

/// TODO
pub(crate) trait DataReceiver<T> {
    /// TODO
    fn set_data_provider(
        &'static self,
        data_provider: &'static dyn DataProvider<T>,
    ) -> Result<(), InitError>;
}
