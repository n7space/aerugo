//! Module representing peripherals internally used by Aerugo.

use samv71_hal::pac::{PMC, TC0};
use samv71_hal::{
    timer::{Ch0, Ch1, Ch2, Channel, Timer, Waveform},
    watchdog::Watchdog,
};

/// System peripherals structure. These peripherals are represented as HAL drivers.
/// Some of these peripherals are available only during HAL initialization
/// (between `AerugoHal::initialize` and `AerugoHal::configure_hardware` calls).
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
