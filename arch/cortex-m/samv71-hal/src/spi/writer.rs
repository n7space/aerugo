//! Module with implementation of SPI Writer.

use core::marker::PhantomData;

use super::metadata::SPIMetadata;

/// SPI Writer.
pub struct Writer<Instance: SPIMetadata> {
    /// Instance metadata.
    _instance: PhantomData<Instance>,
}

impl<Instance: SPIMetadata> Writer<Instance> {
    /// Sets the value that will be transmitted via SPI.
    ///
    /// # Remarks
    /// There's no runtime check for value width, so you have to make sure that you don't try to
    /// send data wider than configured data width for currently selected chip.
    pub fn transmit_value(&mut self, value: u16) {
        Instance::registers().tdr.write(|w| w.td().variant(value));
    }

    /// Creates new Writer instance and returns it.
    pub(super) fn new() -> Self {
        Writer {
            _instance: PhantomData,
        }
    }
}
