//! Module containing Parallel I/O (PIO) pin items for PIO-controlled I/O pin in input mode.

use super::{pin::InputMode, Pin};

use embedded_hal::digital::InputPin;

/// PIO-controlled pin's implementation for pin in input mode.
///
/// There should be input filtering and interrupt configuration functions here.
/// As of now, it's outside of project's requirements, so it's left for future activity.
impl Pin<InputMode> {}

/// InputPin trait implementation from `embedded-hal` crate.
impl InputPin for Pin<InputMode> {
    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}
