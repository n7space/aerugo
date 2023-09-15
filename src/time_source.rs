//! Module containing Aerugo's time source module, providing configurable timestamps for the system
//!
//! Should be used internally by the system.

use core::cell::OnceCell;

use aerugo_hal::AerugoHal;

use crate::error::RuntimeError;
use crate::hal::Hal;
use crate::{Duration, Instant};

/// Time source, responsible for creating timestamps.
///
/// Allows time tracking/timestamp generation since three points in time:
/// * Hardware initialization (call to [`Aerugo::initialize`](crate::Aerugo::initialize))
/// * Start of Aerugo scheduler (call to [`Aerugo::start`](crate::InitApi::start))
/// * User-defined offset
///
/// For safety, instance of TimeSource should never pass interrupt boundary.
/// Failing to adhere to this requirement will invalidate `Sync` trait implementation of this type,
/// unless it's explicitly guaranteed by design that mutations will not occur during interrupt's execution.
pub(crate) struct TimeSource {
    /// Time since system's scheduler start.
    system_start_offset: OnceCell<Duration>,
    /// User-defined offset.
    user_offset: OnceCell<Duration>,
}

impl TimeSource {
    /// Creates new instance of TimeSource
    pub(crate) const fn new() -> Self {
        TimeSource {
            system_start_offset: OnceCell::new(),
            user_offset: OnceCell::new(),
        }
    }

    /// Returns time since system initialization (call to [`Aerugo::initialize`](crate::Aerugo::initialize),
    /// start of the hardware timer)
    #[inline(always)]
    pub(crate) fn time_since_init() -> Instant {
        Hal::get_system_time()
    }

    /// Returns time since system's scheduler start (call to [`Aerugo::start`](crate::InitApi::start)),
    /// or `None` if system hasn't started yet.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::set_system_start`] in parallel with this function (interrupt is treated as different
    /// thread) is an undefined behavior.
    pub(crate) fn time_since_start(&self) -> Option<Instant> {
        match self.system_start_offset.get() {
            Some(start_offset) => TimeSource::time_since_init().checked_sub_duration(*start_offset),
            None => None,
        }
    }

    /// Return system time with offset if it was defined.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::set_user_offset`] in parallel with this function (interrupt is treated as different
    /// thread) is an undefined behavior.
    pub(crate) fn system_time(&self) -> Instant {
        match self.user_offset.get() {
            Some(user_offset) => TimeSource::time_since_init()
                .checked_add_duration(*user_offset)
                .expect("Failed to add user offset"),
            None => TimeSource::time_since_init(),
        }
    }

    /// Returns the duration between system initialization and start of the scheduler, or `None` if system
    /// hasn't started yet.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::set_system_start`] in parallel with this function (interrupt is treated as different
    /// thread) is an undefined behavior.
    pub(crate) fn startup_duration(&self) -> Option<Duration> {
        self.system_start_offset.get().copied()
    }

    /// Sets user-defined offset.
    ///
    /// Specified duration will be subtracted from time since system initialization when a timestamp
    /// is generated using [`TimeSource::system_time`].
    ///
    /// See [`TimeSource::time_since_init`] for details about time since system initialization.
    ///
    /// # Return
    /// `()` is offset was set for the first time, `RuntimeError` otherwise.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::system_time`] in parallel with this function (interrupt is treated as different
    /// thread) is an undefined behavior.
    ///
    /// # Parameters
    /// * `duration` - Duration to offset the time source with.
    pub(crate) unsafe fn set_user_offset(&self, duration: Duration) -> Result<(), RuntimeError> {
        match self.user_offset.set(duration) {
            Ok(_) => Ok(()),
            Err(_) => Err(RuntimeError::UserTimeOffsetAlreadySet),
        }
    }

    /// Saves current timestamp as the moment of system start. Should be called by `Aerugo` right before starting
    /// the scheduler.
    ///
    /// # Return
    /// `()` is was set for the first time, `RuntimeError` otherwise.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::startup_duration`] in parallel with this function (interrupt is treated as different
    /// thread) is an undefined behavior.
    pub(crate) unsafe fn set_system_start(&self) -> Result<(), RuntimeError> {
        let current_time = TimeSource::time_since_init();

        match self
            .system_start_offset
            .set(current_time.duration_since_epoch())
        {
            Ok(_) => Ok(()),
            Err(_) => Err(RuntimeError::SystemTimeStartAlreadySet),
        }
    }
}

/// SAFETY: This is safe, because only [Aerugo](crate::aerugo::Aerugo) uses this. Access to it is
/// not available from IRQ context, so it's safe on the single threading environment.
unsafe impl Sync for TimeSource {}
