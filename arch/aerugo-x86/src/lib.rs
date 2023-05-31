/*!
x86 specific implementation for Aerugo.
*/
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

mod mutex;

pub use self::mutex::Mutex;
