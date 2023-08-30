/*!
# Aerugo HAL

HAL (Hardware Abstract Layer) for Aerugo system.
*/
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod system_hal;

pub use fugit as time;

/// Constant representing system timer frequency.
///
/// Aerugo requires a timer with frequency of 1MHz to measure time with microsecond precision.
pub const SYSTEM_TIMER_FREQUENCY: u32 = 1_000_000;
/// Type representing Aerugo timestamp.
pub type Instant = time::TimerInstantU64<SYSTEM_TIMER_FREQUENCY>;
/// Type representing Aerugo duration.
pub type Duration = time::TimerDurationU64<SYSTEM_TIMER_FREQUENCY>;
