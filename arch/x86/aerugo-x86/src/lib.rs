/*!
x86 specific implementation for Aerugo.
*/
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

#[cfg(feature = "log")]
mod logger;
mod mutex;

#[cfg(feature = "log")]
pub use self::logger::{init_log, log, logln};
pub use self::mutex::Mutex;
