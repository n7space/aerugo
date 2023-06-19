/*!
x86 specific implementation for Aerugo.
*/
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

mod logger;
mod mutex;

pub use self::logger::logln;
pub use self::mutex::Mutex;
