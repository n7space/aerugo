//! Module containing Parallel I/O (PIO) pin items for PIO-controlled I/O pin in input mode.

use super::{pin::InputMode, Pin};

use embedded_hal::digital::InputPin;

/// PIO-controlled pin's implementation for pin in input mode.
///
/// There should be input filtering and interrupt configuration functions here.
/// As of now, it's outside of project's requirements, so it's left for future activity.
impl Pin<InputMode> {}

/// Implementation of InputPin trait from `embedded-hal` crate.
/// These are just aliases of the functions from generic Pin implementation,
/// in case of SAMV71 the pin's state can always be read, but for the sake of
/// type safety, pin should be converted to InputMode to be treated as such.
///
/// `Pin<_>::Error` type is [`core::convert::Infallible`]. These functions cannot fail.
/// They will always return valid state measured on the pin's digital line.
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
