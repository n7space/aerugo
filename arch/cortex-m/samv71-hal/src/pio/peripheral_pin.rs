//! Module containing Parallel I/O (PIO) pin items for peripheral-controlled I/O pin.

use super::{pin::PeripheralMode, Pin};

/// Enumeration specifying peripheral controlling the pin.
///
/// For details about peripheral mapping, see your MCU documentation (Package and
/// Pinout section), or documentation for [`Pin<Port, ID, PeripheralMode>`] implementation.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Peripheral {
    /// Peripheral A.
    A,
    /// Peripheral B.
    B,
    /// Peripheral C.
    C,
    /// Peripheral D.
    D,
}

/// Peripheral-controlled pin's implementation.
///
/// Each pin can be multiplexed with up to four peripherals, and controlled either by
/// one of them, or by PIO controller. Peripheral mapping depends on specific MCU version and package,
/// so for the sake of developer's sanity, it's up to PIO driver's user to specify (and maybe alias?)
/// peripherals mapped to PIO ports correctly.
///
/// Selecting a peripheral that's not mapped to the pin is not a hard error, as it does not enable
/// any additional functionality, nor it prevents using any of the generic pin functions, so it's safe
/// to map PIO pins to unmapped peripherals, although not very useful. Hence, neither the driver,
/// nor the hardware, provides any additional safety measures preventing the user from doing that.
///
/// For more details, consult your MCU's documentation ("Package and Pinout" section, and "Functional
/// Description" section of PIO driver if you want to confirm that this comment is actually true)
impl Pin<PeripheralMode> {
    /// Changes the peripheral controlling the pin. This can be used instead of [`Pin::into_peripheral_pin`]
    /// if the pin is already controlled by a peripheral.
    ///
    /// # Parameters
    /// * `peripheral` - Peripheral that will control the pin.
    pub fn change_peripheral(&mut self, peripheral: Peripheral) {
        self.select_peripheral(peripheral);
    }

    /// Returns the peripheral currently controlling the pin.
    pub fn peripheral(&self) -> Peripheral {
        let select_registers = (
            self.registers_ref().abcdsr[0].read().bits(),
            self.registers_ref().abcdsr[1].read().bits(),
        );

        match (
            self.is_pin_bit_set(select_registers.0),
            self.is_pin_bit_set(select_registers.1),
        ) {
            (false, false) => Peripheral::A,
            (true, false) => Peripheral::B,
            (false, true) => Peripheral::C,
            (true, true) => Peripheral::D,
        }
    }
}
