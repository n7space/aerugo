//! Module with implementation of SPI Reader.

use core::marker::PhantomData;

use super::metadata::SPIMetadata;

/// SPI Reader.
pub struct Reader<Instance: SPIMetadata> {
    /// Instance metadata.
    _instance: PhantomData<Instance>,
}

impl<Instance: SPIMetadata> Reader<Instance> {
    /// Returns the data currently stored in RX register.
    ///
    /// Before reading the data you should make sure that it's available by checking
    /// `rx_data_register_full` field of [`SpiStatus`](super::status::SpiStatus) structure.
    pub fn get_received_data(&self) -> u16 {
        Instance::registers().rdr.read().rd().bits()
    }

    /// Creates new Reader instance and returns it.
    pub(super) fn new() -> Self {
        Reader {
            _instance: PhantomData,
        }
    }
}
