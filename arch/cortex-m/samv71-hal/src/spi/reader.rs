//! Module with implementation of SPI Reader.

use core::marker::PhantomData;

use super::metadata::SPIMetadata;

/// SPI Reader.
pub struct Reader<Instance: SPIMetadata> {
    /// Instance metadata.
    _instance: PhantomData<Instance>,
}

impl<Instance: SPIMetadata> Reader<Instance> {
    /// Creates new Reader instance and returns it.
    pub(super) fn new() -> Self {
        Reader {
            _instance: PhantomData,
        }
    }
}
