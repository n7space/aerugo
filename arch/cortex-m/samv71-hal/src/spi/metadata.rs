//! Module containing meta-traits and their implementation for HAL SPI driver
use crate::pac::spi0::RegisterBlock;
pub use crate::pac::{SPI0, SPI1};

/// Trait for PAC SPI instances.
///
/// This trait erases the type of SPI instance, so it can be used as generic argument for
/// [`Spi`](super::Spi) instead of concrete type.
pub trait SPIMetadata {
    /// Pointer to SPI registers.
    const REGISTERS: *const RegisterBlock;

    /// Returns a reference to SPI register block.
    ///
    /// # Safety
    /// This function dereferences a raw pointer.
    /// It's safe to use, as long as there aren't multiple instances of the same SPI peripheral.
    #[inline(always)]
    fn registers() -> &'static RegisterBlock {
        unsafe { &*Self::REGISTERS }
    }
}

impl SPIMetadata for SPI0 {
    const REGISTERS: *const RegisterBlock = SPI0::PTR;
}

impl SPIMetadata for SPI1 {
    const REGISTERS: *const RegisterBlock = SPI1::PTR;
}
