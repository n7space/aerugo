//! Module with implementation of SPI Writer.

use core::marker::PhantomData;

use super::metadata::SPIMetadata;

/// SPI Writer.
pub struct Writer<Instance: SPIMetadata> {
    /// Instance metadata.
    _instance: PhantomData<Instance>,
}

impl<Instance: SPIMetadata> Writer<Instance> {
    /// Creates new Writer instance and returns it.
    pub(super) fn new() -> Self {
        Writer {
            _instance: PhantomData,
        }
    }
}
