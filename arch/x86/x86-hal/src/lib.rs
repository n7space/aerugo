/*!
x86 implementation of aerugo HAL.
*/
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

pub(crate) use fugit as time;

pub mod hal;
pub mod peripherals;

pub use self::hal::Hal;
pub use self::peripherals::Peripherals;
