//! This module contains all the drivers for supported peripherals.

#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

pub use cortex_m;
pub use embedded_hal;
pub use fugit as time;
pub use samv71q21_pac as pac;

#[cfg(feature = "rt")]
/// Macro for interrupt handlers.
pub use pac::interrupt;

pub mod nvic;
pub mod pio;
pub mod pmc;
pub mod timer;
pub mod uart;
pub mod utils;
pub mod watchdog;
