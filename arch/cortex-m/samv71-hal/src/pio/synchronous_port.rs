//! This module contains items related to synchronous mode access to PIO pins.
//!
//! Synchronous mode allows the user to quickly change the state of multiple pins
//! with a single write operation.

use core::fmt::Debug;
use core::ops::Index;
use core::{marker::PhantomData, ops::IndexMut};
use heapless::Vec;

use super::{
    pin::OutputMode,
    port_metadata::{IoPortMetadata, RegisterBlock},
    Pin,
};

/// Structure representing an array of synchronously-programmable pins.
///
/// This structure allows to combine pins of a single port into an array
/// of synchronous pins. Synchronous pins can be driven in parallel with a
/// single write operation.
///
/// Note that this structure may represent only a part of I/O port, and there
/// might be multiple SPA's for a single port (assuming they won't share any
/// pins), but the interface of this structure is made with one-per-port usage
/// for simplicity. **It's recommended to read and comprehend the documentation
/// of this module before using it, to prevent unexpected code behavior.**
///
/// This structure provides a safe way of changing the state of grouped pins.
/// To change the state of the pins, [`SynchronousPort::set_state`] must
/// be called with bitmask that should specify new state of **all** pins, or
/// [`SynchronousPort::set_masked_state`] must be called with bitmask that
/// should specify new state of **masked** pins.
/// **Read documentation of these functions for more details and usage examples.**
///
/// The pins can be referenced from inside of this structure, as it implements
/// [`Index`] and [`IndexMut`] traits. Order of the pins passed to
/// [`SynchronousPort::new`] is preserved.
pub struct SynchronousPort<Port: IoPortMetadata, const N: usize> {
    /// Array with pins configured in synchronous mode.
    pins: Vec<Pin<OutputMode>, N>,
    /// Bitmask, masking pins owned by the port.
    port_mask: u32,
    /// Phantom port metadata.
    _port: PhantomData<Port>,
}

/// Enumeration representing errors that can happen while working
/// with synchronous mode.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SynchronousModeError {
    /// Invalid pin was tried to be used while creating new SynchronousPort instance.
    InvalidPin,
}

impl<Port: IoPortMetadata, const N: usize> SynchronousPort<Port, N> {
    /// Creates new instance of SynchronousPort. Enables synchronous mode for passed pins.
    ///
    /// # Parameters
    /// * `pins` - Array of pins used in synchronous mode. **All the pins must belong
    ///            to the same port**
    ///
    /// # Returns
    /// `Ok(Self)` on successful creation, `Err([SynchronousModeError::InvalidPin])` if
    /// all the pins aren't owned by the same port.
    pub fn new(mut pins: [Pin<OutputMode>; N]) -> Result<Self, SynchronousModeError> {
        let mut port_mask: u32 = 0;

        for pin in &mut pins {
            if pin.port_id() != Port::ID {
                return Err(SynchronousModeError::InvalidPin);
            }

            pin.enable_synchronous_mode();
            port_mask |= 1u32 << pin.id();
        }

        Ok(Self {
            pins: pins.into_iter().collect(),
            port_mask,
            _port: PhantomData,
        })
    }

    /// Splits SynchronousPort instance into pins array, returning control over them to the user.
    /// Disables synchronous mode for all pins.
    pub fn into_pins(self) -> [Option<Pin<OutputMode>>; N] {
        // Unwrap: This cannot fail as size is bound by the generic argument in both return
        // type, and collected vector.
        self.pins
            .into_iter()
            .map(|mut pin| {
                pin.disable_synchronous_mode();
                Some(pin)
            })
            .collect::<Vec<Option<Pin<OutputMode>>, N>>()
            .into_array()
            .unwrap()
    }

    /// Returns current state forced on synchronous pins.
    /// **This is not the state read from the pin, but the effect of `set_state`/`set_masked_state` operation.**
    ///
    /// # Returns
    /// Bitmask containing the state forced on pins owned by this SynchronousPort.
    /// Order of bits from the register is preserved - state of port X is specified by bit X.
    /// Bits representing pins that are not owned by this SynchronousPort will be set to `0`.
    ///
    /// # Example
    /// ```no_run
    /// let sync_port = SynchronousPort::<PIOC, 4>::new([
    ///     pins[1].take().unwrap().into_output_pin(),
    ///     pins[4].take().unwrap().into_output_pin(),
    ///     pins[12].take().unwrap().into_output_pin(),
    ///     pins[20].take().unwrap().into_output_pin(),
    /// ])
    /// .unwrap();
    ///
    /// // Do stuff...
    /// // ...
    /// // ...
    /// // ...
    ///
    /// // Bits on positions 1, 4, 12 and 20 represent states of port's pins.
    /// // Remaining bits will be set to 0.
    /// let state_bitfield = sync_port.output_state();
    ///
    /// if state_bitfield & ((1 << 4) & (1 << 12)) != 0 {
    ///     logln!("PC4 and PC12 are turned on!")
    /// }
    /// ```
    pub fn output_state(&self) -> u32 {
        self.masked(self.registers_ref().odsr.read().bits())
    }

    /// Sets the state of all pins in the SynchronousPort.
    ///
    /// This allows you to force the state on all pins managed by this port at once.
    /// If you'd rather change the state of only some of the pins, see [`SynchronousPort::set_masked_state`].
    ///
    /// Order of bits from the register is preserved - state of port X is specified by bit X.
    /// Bits representing pins that are not owned by this SynchronousPort will be ignored.
    ///
    /// # Example
    /// ```no_run
    /// let mut sync_port = SynchronousPort::<PIOC, 4>::new([
    ///     pins[1].take().unwrap().into_output_pin(),
    ///     pins[2].take().unwrap().into_output_pin(),
    ///     pins[3].take().unwrap().into_output_pin(),
    ///     pins[4].take().unwrap().into_output_pin(),
    /// ])
    /// .unwrap();
    ///
    /// // Drives all managed pins high.
    /// // State of bit 0 (set to `0` here) doesn't matter.
    /// sync_port.set_state(0b11110);
    ///
    /// // Drives pins 1 and 3 high, 2 and 4 low
    /// sync_port.set_state(0b01010);
    ///
    /// // Drives all managed pins low.
    /// sync_port.set_state(0);
    /// ```
    pub fn set_state(&mut self, state: u32) {
        // Safety: This is safe because we're modifying the value of the register
        // instead of writing new value, and modifications are masked to prevent
        // accessing non-owned bits.
        self.registers_ref().odsr.modify(|r, w| unsafe {
            let preserved_state = r.bits() & !self.port_mask;
            w.bits(preserved_state | self.masked(state))
        });
    }

    /// Sets the state of pins in the array, applying the provided mask to new state beforehand.
    ///
    /// # Example
    /// ```no_run
    /// let mut sync_port = SynchronousPort::<PIOC, 4>::new([
    ///     pins[1].take().unwrap().into_output_pin(),
    ///     pins[2].take().unwrap().into_output_pin(),
    ///     pins[3].take().unwrap().into_output_pin(),
    ///     pins[4].take().unwrap().into_output_pin(),
    /// ])
    /// .unwrap();
    ///
    /// // Drives bits 3 and 4 high, doesn't change the state
    /// // of the rest of the pins.
    /// sync_port.set_masked_state(0b11110, 0b11000);
    ///
    /// // Drives pin 1 high and 2 low, doesn't change the state
    /// // of the rest of the pins
    /// sync_port.set_masked_state(0b01010, 0b110);
    ///
    /// // Does nothing.
    /// sync_port.set_masked_state(0xFFFFFFFF, 0);
    /// ```
    #[inline(always)]
    pub fn set_masked_state(&mut self, state: u32, mask: u32) {
        // Safety: This is safe because we're modifying the value of the register
        // instead of writing new value, and modifications are masked to prevent
        // accessing non-owned and unwanted bits.
        self.registers_ref().odsr.modify(|r, w| unsafe {
            let preserved_state = r.bits() & (!self.port_mask | !mask);
            w.bits(preserved_state | self.masked(state & mask))
        });
    }

    /// Returns a reference to the registers of port owning managed pins.
    ///
    /// # Safety
    /// This function is safe to use, because every operation on port registers
    /// is masked to prevent access or modification of the state of pins not owned
    /// by current instance of this structure.
    ///
    /// Duplicate instances of this structure may exist as long, as there's no duplicate
    /// instances of the pins.
    const fn registers_ref(&self) -> &RegisterBlock {
        unsafe { &*Port::REGISTERS }
    }

    /// Returns value masked with bitmask that clears bits not representing pins owned
    /// by current instance.
    #[inline(always)]
    const fn masked(&self, register_value: u32) -> u32 {
        register_value & self.port_mask
    }
}

impl<Port: IoPortMetadata, const N: usize> Index<usize> for SynchronousPort<Port, N> {
    type Output = Pin<OutputMode>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pins[index]
    }
}

impl<Port: IoPortMetadata, const N: usize> IndexMut<usize> for SynchronousPort<Port, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pins[index]
    }
}

impl<Port: IoPortMetadata, const N: usize> Debug for SynchronousPort<Port, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SynchronousPort")
            .field("port", &Port::ID)
            .field("pins", &self.pins)
            .finish()
    }
}
