//! Module containing Aerugo's time source module, providing configurable timestamps for the system
//! Should be used internally by the system.

use aerugo_hal::system_hal::SystemHal;

use crate::hal::Hal;
use crate::time::{TimerDurationU64, TimerInstantU64};

/// Type representing TimeSource's timestamp.
pub type Timestamp = TimerInstantU64<1_000_000>;
/// Type representing TimeSource's duration.
pub type Duration = TimerDurationU64<1_000_000>;

/// Time source, responsible for creating timestamps.
///
/// Allows time tracking/timestamp generation since three points in time:
/// * Hardware initialization (call to [`Aerugo::initialize`](crate::Aerugo::initialize))
/// * Start of Aerugo scheduler (call to [`Aerugo::start`](crate::Aerugo::start))
/// * User-defined offset
pub struct TimeSource {
    /// Time since system's scheduler start.
    _system_start_offset: Option<Duration>,
    /// User-defined offset.
    _user_offset: Option<Duration>,
}

impl TimeSource {
    /// Creates new instance of TimeSource
    pub const fn new() -> Self {
        TimeSource {
            _system_start_offset: None,
            _user_offset: None,
        }
    }

    /// Returns time since system initialization (call to [`Aerugo::initialize`](crate::Aerugo::initialize), start of the hardware timer)
    pub fn time_since_init() -> Timestamp {
        Hal::get_system_time()
    }

    /// Returns time since system's scheduler start (call to [`Aerugo::start`](crate::Aerugo::start)), or `None` if system hasn't started yet.
    pub fn _time_since_start(&self) -> Option<Timestamp> {
        todo!()
    }

    /// Returns time since user-defined offset, or `None` if offset is not defined.
    pub fn _time_since_user_offset(&self) -> Option<Timestamp> {
        todo!()
    }

    /// Sets user-defined offset.
    ///
    /// Specified duration will be subtracted from time since system initialization when a timestamp
    /// is generated using [`TimeSource::time_since_user_offset`].
    ///
    /// See [`TimeSource::time_since_init`] for details about time since system initialization.
    ///
    /// # Parameters
    /// * `duration` - Duration to offset the time source with.
    pub fn _set_user_offset(&mut self, _duration: Duration) {
        todo!()
    }

    /// Returns the duration between system initialization and start of the scheduler, or `None` if system hasn't started yet.
    pub fn _startup_duration(&self) -> Option<Duration> {
        todo!()
    }

    /// Saves current timestamp as the moment of system start. Should be called by `Aerugo` right before starting the scheduler.
    pub(crate) fn _mark_system_start(&mut self) {
        todo!()
    }
}
