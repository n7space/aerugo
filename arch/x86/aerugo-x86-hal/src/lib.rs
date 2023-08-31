/*!
x86 implementation of aerugo HAL.
*/
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod error;
pub mod hal;
mod system_peripherals;
pub mod user_peripherals;

pub use self::hal::Hal;
pub use user_peripherals::UserPeripherals;
