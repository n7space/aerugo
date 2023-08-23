//! System HAL.

mod config;

pub use self::config::SystemHardwareConfig;

use bare_metal::CriticalSection;
use core::ops::{Add, Sub};

/// System HAL trait.
pub trait SystemHal {
    /// Type for an instant in time.
    type Instant: Ord
        + Copy
        + Add<Self::Duration, Output = Self::Instant>
        + Sub<Self::Duration, Output = Self::Instant>
        + Sub<Self::Instant, Output = Self::Duration>;

    /// Type for a duration of time.
    type Duration;

    /// Type for system HAL error.
    type Error;

    /// Creates global HAL instance. Since there can only be a single instance of HAL, this function
    /// should initialize it's global state and prepare the environment for hardware configuration.
    fn create() -> Result<(), Self::Error>;

    /// Configure system hardware.
    ///
    /// Implementation should initialize and configure all core system peripherals.
    ///
    /// # Parameters
    /// * `config` - System hardware configuration.
    fn configure_hardware(config: SystemHardwareConfig) -> Result<(), Self::Error>;

    /// Gets current system time timestamp.
    fn get_system_time() -> Self::Instant;

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
        F: FnOnce(&CriticalSection) -> R;
}
