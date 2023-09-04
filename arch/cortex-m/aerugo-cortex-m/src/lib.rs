/*!
Cortex-M specific implementation for Aerugo.
*/
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

#[cfg(feature = "log")]
mod logger;

#[cfg(feature = "log")]
pub use self::logger::{init_log, log, logln};
