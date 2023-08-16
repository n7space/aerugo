/*!
SAMV71 implementation of aerugo HAL.
*/
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

extern crate internal_cell;
extern crate samv71q21_pac as pac;

pub(crate) use fugit as time;

pub mod drivers;
pub mod error;
pub mod hal;
mod system_peripherals;
pub mod user_peripherals;

pub use self::hal::Hal;
pub use embedded_hal;
pub use pac::{NVIC, PMC};

#[cfg(feature = "rt")]
pub use pac::interrupt;
