/*!
# Aerugo HAL

HAL (Hardware Abstract Layer) for Aerugo system.
*/
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

mod config;

pub use config::SystemHardwareConfig;
pub use critical_section;
pub use critical_section::CriticalSection;
pub use fugit as time;

/// Constant representing system timer frequency.
///
/// Aerugo requires a timer with frequency of 1MHz to measure time with microsecond precision.
pub const SYSTEM_TIMER_FREQUENCY: u32 = 1_000_000;
/// Type representing Aerugo timestamp.
pub type Instant = time::TimerInstantU64<SYSTEM_TIMER_FREQUENCY>;
/// Type representing Aerugo duration.
pub type Duration = time::TimerDurationU64<SYSTEM_TIMER_FREQUENCY>;

/// System HAL trait.
pub trait AerugoHal {
    /// Type for system HAL error.
    type Error;

    /// Configure system hardware.
    ///
    /// Implementation should initialize and configure all core system peripherals.
    ///
    /// # Parameters
    /// * `config` - System hardware configuration.
    fn configure_hardware(config: SystemHardwareConfig) -> Result<(), Self::Error>;

    /// Gets current system time timestamp.
    fn get_system_time() -> Instant;

    /// Feeds the system watchdog.
    fn feed_watchdog();

    /// Enters critical section
    fn enter_critical();

    /// Exits critical section
    fn exit_critical();

    /// Executes closure `f` in an interrupt-free context.
    ///
    /// # Generic Parameters
    /// * `F` - Closure type.
    /// * `R` - Closure return type.
    ///
    /// # Parameters
    /// * `f` - Closure to execute.
    ///
    /// # Return
    /// Closure result.
    fn execute_critical<F, R>(f: F) -> R
    where
        F: FnOnce(CriticalSection) -> R;
}
