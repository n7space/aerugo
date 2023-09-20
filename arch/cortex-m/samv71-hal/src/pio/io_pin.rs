//! Module containing Parallel I/O (PIO) pin items for PIO-controlled I/O pin.

use super::{pin::IOMode, port_metadata::IoPortMetadata, Pin};

/// Enumeration representing I/O pin's direction.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    /// Pin is an input
    Input,
    /// Pin is an output
    Output,
}

/// PIO-controlled pin's implementation.
impl<Port: IoPortMetadata, const ID: u8> Pin<Port, ID, IOMode> {
    /// Returns current direction of the pin (Input/Output)
    pub fn direction(&self) -> Direction {
        match self.is_pin_bit_set(self.registers_ref().osr.read().bits()) {
            true => Direction::Output,
            false => Direction::Input,
        }
    }

    /// Sets the direction of the pin (Input/Output)
    ///
    /// # Parameters
    /// * `direction` - Desired direction.
    pub fn set_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Input => self
                .registers_ref()
                .odr
                .write(|w| unsafe { w.bits(self.pin_mask()) }),
            Direction::Output => self
                .registers_ref()
                .oer
                .write(|w| unsafe { w.bits(self.pin_mask()) }),
        };
    }

    /// Returns true if pin is currently configured as an output.
    pub fn is_output(&self) -> bool {
        self.direction() == Direction::Output
    }

    /// Returns true if pin is currently configured as an input.
    pub fn is_input(&self) -> bool {
        self.direction() == Direction::Input
    }
}
