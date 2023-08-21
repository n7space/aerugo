//! Module representing peripherals internally used by Aerugo.

use pac::{PMC, TC0};

use crate::drivers::{
    timer::{Ch0, Ch1, Ch2, Channel, Timer, Waveform},
    watchdog::Watchdog,
};

/// System peripherals structure. These peripherals are represented as HAL drivers.
/// They are initialized on system init, and used directly by HAL to provide core functionality.
pub struct SystemPeripherals {
    /// Watchdog instance.
    pub watchdog: Watchdog,
    /// Timer instance.
    pub timer: Timer<TC0>,
    /// Timer's channel 0 instance.
    pub timer_ch0: Option<Channel<TC0, Ch0, Waveform>>,
    /// Timer's channel 1 instance.
    pub timer_ch1: Option<Channel<TC0, Ch1, Waveform>>,
    /// Timer's channel 2 instance.
    pub timer_ch2: Option<Channel<TC0, Ch2, Waveform>>,
    /// PMC instance. This will be stored only temporarily here, between HAL init and system config
    pub pmc: Option<PMC>,
}
