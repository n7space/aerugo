//! Implementation of SPI status reader.

use core::marker::PhantomData;

use super::{metadata::SPIMetadata, status::SpiStatus};

/// SPI status reader that can be used to fetch SPI status (for example, from an interrupt).
pub struct StatusReader<Instance: SPIMetadata> {
    /// PAC SPI metadata.
    _meta: PhantomData<Instance>,
}

impl<Instance: SPIMetadata> StatusReader<Instance> {
    /// Returns current SPI status.
    ///
    /// # Safety
    /// Some status bits are automatically cleared after they are read, so all status bits must
    /// be handled immediately after reading them, otherwise some information about SPI state may
    /// be lost.
    pub fn status(&mut self) -> SpiStatus {
        Instance::registers().sr.read().into()
    }

    /// Blocks current thread until provided functor returns `true`.
    ///
    /// # Parameters
    /// * `f` - Function checking the status.
    /// * `timeout` - Maximum amount of status checks before returning `false`.
    ///
    /// # Returns
    /// `true` when `f` returns `true`, `false` on timeout.
    pub fn wait_for_status<F: Fn(SpiStatus) -> bool>(&mut self, f: F, mut timeout: usize) -> bool {
        while timeout > 0 {
            timeout -= 1;

            if f(self.status()) {
                return true;
            }
        }

        false
    }

    /// Private constructor that allows SPI instance to create it's StatusReader.
    pub(super) fn new() -> Self {
        StatusReader { _meta: PhantomData }
    }
}
