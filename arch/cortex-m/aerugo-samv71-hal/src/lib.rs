/*!
SAMV71 implementation of aerugo HAL.
*/
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

mod system_peripherals;

pub mod error;
pub mod hal;
pub mod user_peripherals;

pub use hal::Hal;
pub use samv71_hal as drivers;
pub use samv71_hal::cortex_m;
pub use user_peripherals::UserPeripherals;
