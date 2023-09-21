//! Module containing Parallel I/O (PIO) pin items for PIO-controlled I/O pin in output mode.

use embedded_hal::digital::{OutputPin, PinState, StatefulOutputPin, ToggleableOutputPin};

use super::{pin::OutputMode, Pin};

/// PIO-controlled pin's implementation for pin in output mode.
///
/// This mode allows the pin to:
/// * Set it's output state
/// * Configure it in open-drain or push-pull mode
/// * Configure
impl Pin<OutputMode> {
    /// Sets the pin state. Calls [`Pin<OutputMode>::set_low`] or [`Pin<OutputMode>::set_high`].
    pub fn set_state(&mut self, state: PinState) {
        match state {
            PinState::Low => self.set_low().unwrap(),
            PinState::High => self.set_high().unwrap(),
        }
    }
}

/// Implementation of OutputPin trait from `embedded-hal` crate.
///
/// `Pin<_>::Error` type is [`core::convert::Infallible`]. These functions cannot fail.
/// As long as pin is in output mode, it always can be set high or low.
impl OutputPin for Pin<OutputMode> {
    /// Drives the pin low.
    ///
    /// In both push-pull and open-drain mode, this will pull the pin to GND.
    fn set_low(&mut self) -> Result<(), Self::Error> {
        // Safety: See `Pin::pin_mask` description.
        self.registers_ref()
            .codr
            .write(|w| unsafe { w.bits(self.pin_mask()) });
        Ok(())
    }

    /// Drives the pin high.
    ///
    /// In push-pull mode, this will set the pin to high state by pulling it to VDD.
    ///
    /// In open-drain mode, this will keep the pin floating, and since it will usually
    /// be pulled up, it should also indirectly result in pulling the pin to high state.
    fn set_high(&mut self) -> Result<(), Self::Error> {
        // Safety: See `Pin::pin_mask` description.
        self.registers_ref()
            .sodr
            .write(|w| unsafe { w.bits(self.pin_mask()) });
        Ok(())
    }
}

/// Implementation of OutputPin trait from `embedded-hal` crate.
///
/// `Pin<_>::Error` type is [`core::convert::Infallible`]. These functions cannot fail.
/// As long as pin is in output mode, it's set state can always be looked up.
impl StatefulOutputPin for Pin<OutputMode> {
    /// Returns true if the pin is currently driven "high".
    ///
    /// **It's important to notice that this function does not read pin's state,
    /// it will only inform whether it's set to be driven high, or not.**
    /// If you want to read current logic state of the pin, use [`Pin<_>::is_high`]/[`Pin<_>::is_low`]/[`Pin<_>::state`].
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_pin_bit_set(self.registers_ref().odsr.read().bits()))
    }

    /// Returns true if the pin is currently driven "low".
    ///
    /// **It's important to notice that this function does not read pin's state,
    /// it will only inform whether it's set to be driven high, or not.**
    /// If you want to read current logic state of the pin, use [`Pin<_>::is_high`]/[`Pin<_>::is_low`]/[`Pin<_>::state`].
    #[inline(always)]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_set_high().unwrap())
    }
}

/// Implementation of OutputPin trait from `embedded-hal` crate.
///
/// # Remarks
/// SAMV71 PIO driver does not provide any quick way of toggling the pin's state,
/// so this function uses the naive "check + invert state" approach.
///
/// `Pin<_>::Error` type is [`core::convert::Infallible`]. These functions cannot fail.
/// As long as pin is in output mode, it can always be toggled.
impl ToggleableOutputPin for Pin<OutputMode> {
    /// Toggles pin output (from "high" to "low" or vice versa).
    fn toggle(&mut self) -> Result<(), Self::Error> {
        if self.is_set_high().unwrap() {
            self.set_low().unwrap();
        } else {
            self.set_high().unwrap();
        }
        Ok(())
    }
}
