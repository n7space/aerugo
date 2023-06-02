/// System HAL.
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

    /// Gets current system time timestamp.
    fn get_system_time(&'static self) -> Self::Instant;
}
