//! Module containing Aerugo's time source module, providing configurable timestamps for the system
//!
//! Should be used internally by the system.

use core::cell::OnceCell;

use aerugo_hal::AerugoHal;

use crate::error::RuntimeError;
use crate::hal::Hal;
use crate::time::{Duration, Instant};

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
    /// Time when system scheduler started.
    system_start: OnceCell<Instant>,
    /// Time it took to start the system scheduler.
    system_start_offset: OnceCell<Duration>,
    /// User-defined offset.
    user_offset: OnceCell<Duration>,
}

/// SAFETY: It is safe assuming that TimeSource is not accessible from the IRQ context.
/// This is ensured by one instance, owned by [Aerugo](crate::aerugo::Aerugo) structure which
/// makes it inaccessible in IRQ context and initializes it before scheduler starts.
///
/// Internal `OnceCell`s for storing system start timestamps are only mutably accessed by
/// [`TimeSource::set_system_start] which is called by [start function](crate::api::InitApi::start)
/// from `InitApi`, which is not accessible from the IRQ context.
///
/// Internal `OnceCell` for user offset is only mutable accessed by [`TimeSource::set_user_offset`]
/// which is called by [set_system_time_offset](crate::api::RuntimeApi::set_system_time_offset)
/// from `RuntimeApi`, which is not accessible from the IRQ context.
///
/// If user somehow exposes `InitApi` or `RuntimeApi` trait interfaces to the IRQ context, any
/// usage from that context can be considered unsafe.
unsafe impl Sync for TimeSource {}

impl TimeSource {
    /// Creates new instance of TimeSource
    pub(crate) const fn new() -> Self {
        TimeSource {
            system_start: OnceCell::new(),
            system_start_offset: OnceCell::new(),
            user_offset: OnceCell::new(),
        }
    }

    /// Return system time with offset if it was defined.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass
    /// interrupt boundary. Calling [`TimeSource::set_user_offset`] in parallel with this function
    /// (interrupt is treated as different thread) is an undefined behavior.
    pub(crate) fn system_time(&self) -> Instant {
        let start_time = self
            .time_since_start()
            .expect("System start offset not set");

        match self.apply_offset(start_time) {
            Some(start_time_with_offset) => start_time_with_offset,
            None => start_time,
        }
    }

    /// Return time elapsed since scheduler start.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass
    /// interrupt boundary. Calling [`TimeSource::set_system_start`] in parallel with this function
    /// (interrupt is treated as different thread) is an undefined behavior.
    pub(crate) fn elapsed_time(&self) -> Duration {
        Hal::get_system_time() - *self.system_start.get().expect("System not started")
    }

    /// Saves current timestamp as the moment of system start. Should be called by `Aerugo` right
    /// before starting the scheduler.
    ///
    /// # Return
    /// `()` is was set for the first time, `RuntimeError` otherwise.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass
    /// interrupt boundary. Calling [`TimeSource::startup_duration`],
    /// [`TimeSource::calculate_absolute_time`], [`TimeSource::elapsed_time`] in parallel with
    /// this function (interrupt is treated as different thread) is an undefined behavior.
    pub(crate) unsafe fn set_system_start(&self) {
        let current_time = Hal::get_system_time();

        self.system_start
            .set(current_time)
            .expect("Failed to set system start timestamp");
        self.system_start_offset
            .set(current_time.duration_since_epoch())
            .expect("Failed to set system start offset");
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
    /// Calling [`TimeSource::apply_offset`] or [`TimeSource::system_time`] in parallel with this function
    /// (interrupt is treated as different thread) is an undefined behavior.
    ///
    /// # Parameters
    /// * `duration` - Duration to offset the time source with.
    pub(crate) unsafe fn set_user_offset(&self, duration: Duration) -> Result<(), RuntimeError> {
        match self.user_offset.set(duration) {
            Ok(_) => Ok(()),
            Err(_) => Err(RuntimeError::UserTimeOffsetAlreadySet),
        }
    }

    /// Returns the duration between system initialization and start of the scheduler, or `None` if system
    /// hasn't started yet.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::set_system_start`] in parallel with this function (interrupt is treated as different
    /// thread) is an undefined behavior.
    pub(crate) fn startup_duration(&self) -> Duration {
        *self
            .system_start_offset
            .get()
            .expect("System start offset not set")
    }

    /// Calculates absolute time based on system time.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::set_system_start`] in parallel with this function (interrupt is treated as different
    /// thread) is an undefined behavior.
    pub(crate) fn calculate_absolute_time(&self, time: Duration) -> Instant {
        let system_start = self
            .system_start
            .get()
            .expect("System start timestamp not set");

        let absolute_time = *system_start + time;

        match self.apply_offset(absolute_time) {
            Some(absolute_time_with_offset) => absolute_time_with_offset,
            None => absolute_time,
        }
    }

    /// Returns time since system's scheduler start (call to [`Aerugo::start`](crate::InitApi::start)),
    /// or `None` if system hasn't started yet.
    ///
    /// # Safety
    /// This is safe as long as it's used in single-core context, and `TimeSource` does not pass interrupt boundary.
    /// Calling [`TimeSource::set_system_start`] in parallel with this function (interrupt is treated as different
    /// thread) is an undefined behavior.
    fn time_since_start(&self) -> Option<Instant> {
        self.system_start_offset.get().map(|offset| {
            self.time_since_init()
                .checked_sub_duration(*offset)
                .expect("Failed to sub system start offset")
        })
    }

    /// Returns time since system initialization (call to [`Aerugo::initialize`](crate::Aerugo::initialize),
    /// start of the hardware timer)
    fn time_since_init(&self) -> Instant {
        Hal::get_system_time()
    }

    /// Applies user offset to the given time.
    fn apply_offset(&self, time: Instant) -> Option<Instant> {
        self.user_offset.get().map(|offset| {
            time.checked_add_duration(*offset)
                .expect("Failed to add user offset")
        })
    }
}

#[cfg(any(doc, test))]
mod tests {
    use super::*;

    /// @SRS{ROS-FUN-RTOS-6010}
    #[cfg_attr(not(doc), test)]
    #[allow(non_upper_case_globals)]
    fn req_query_elapsed_time() {
        let time_source = TimeSource::new();

        unsafe {
            time_source.set_system_start();
        }

        let elapsed_time = time_source.elapsed_time();
        assert!(elapsed_time.ticks() > 0);
    }
}
