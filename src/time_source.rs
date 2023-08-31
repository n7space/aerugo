//! Module containing Aerugo's time source module, providing configurable timestamps for the system
//! Should be used internally by the system.

use crate::hal::Hal;
use crate::internal_cell::InternalCell;
use crate::{Duration, Instant};
use aerugo_hal::AerugoHal;

/// Time source, responsible for creating timestamps.
///
/// Allows time tracking/timestamp generation since three points in time:
/// * Hardware initialization (call to [`Aerugo::initialize`](crate::Aerugo::initialize))
/// * Start of Aerugo scheduler (call to [`Aerugo::start`](crate::Aerugo::start))
/// * User-defined offset
///
/// For safety, instance of TimeSource should never pass interrupt boundary.
/// Failing to adhere to this requirement will invalidate `Sync` trait implementation of this type,
/// unless it's explicitly guaranteed by design that mutations will not occur during interrupt's execution.
pub struct TimeSource {
    /// Time since system's scheduler start.
    system_start_offset: InternalCell<Option<Duration>>,
    /// User-defined offset.
    user_offset: InternalCell<Option<Duration>>,
}

impl TimeSource {
    /// Creates new instance of TimeSource
    pub const fn new() -> Self {
        TimeSource {
            system_start_offset: InternalCell::new(None),
            user_offset: InternalCell::new(None),
        }
    }

    /// Returns time since system initialization (call to [`Aerugo::initialize`](crate::Aerugo::initialize), start of the hardware timer)
    #[inline(always)]
    pub fn time_since_init() -> Instant {
        Hal::get_system_time()
    }

    /// Returns time since system's scheduler start (call to [`Aerugo::start`](crate::Aerugo::start)), or `None` if system hasn't started yet.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::mark_system_start`] in parallel with this function (interrupt is treated as different thread)
    /// is an undefined behavior.
    pub fn time_since_start(&self) -> Option<Instant> {
        match unsafe { *self.system_start_offset.as_ref() } {
            Some(start_offset) => TimeSource::time_since_init().checked_sub_duration(start_offset),
            None => None,
        }
    }

    /// Returns time since user-defined offset, or `None` if offset is not defined, or cannot be subtracted from system time.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::set_user_offset`] in parallel with this function (interrupt is treated as different thread)
    /// is an undefined behavior.
    pub fn time_since_user_offset(&self) -> Option<Instant> {
        match unsafe { *self.user_offset.as_ref() } {
            Some(user_offset) => TimeSource::time_since_init().checked_add_duration(user_offset),
            None => None,
        }
    }

    /// Sets user-defined offset.
    ///
    /// Specified duration will be subtracted from time since system initialization when a timestamp
    /// is generated using [`TimeSource::time_since_user_offset`].
    ///
    /// See [`TimeSource::time_since_init`] for details about time since system initialization.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::time_since_user_offset`] in parallel with this function (interrupt is treated as different
    /// thread) is an undefined behavior.
    ///
    /// # Parameters
    /// * `duration` - Duration to offset the time source with.
    pub(crate) unsafe fn set_user_offset(&self, duration: Duration) {
        let offset_ref = unsafe { self.user_offset.as_mut_ref() };
        offset_ref.replace(duration);
    }

    /// Returns the duration between system initialization and start of the scheduler, or `None` if system hasn't started yet.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::mark_system_start`] in parallel with this function (interrupt is treated as different
    /// thread) is an undefined behavior.
    pub fn startup_duration(&self) -> Option<Duration> {
        unsafe { *self.system_start_offset.as_ref() }
    }

    /// Saves current timestamp as the moment of system start. Should be called by `Aerugo` right before starting the scheduler.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::startup_duration`] in parallel with this function (interrupt is treated as different
    /// thread) is an undefined behavior.
    pub(crate) unsafe fn mark_system_start(&self) {
        let current_time = TimeSource::time_since_init();
        let offset_ref = unsafe { self.system_start_offset.as_mut_ref() };
        offset_ref.replace(current_time.duration_since_epoch());
    }
}
