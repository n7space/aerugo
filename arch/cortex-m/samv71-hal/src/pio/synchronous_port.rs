//! This module contains items related to synchronous mode access to PIO pins.
//!
//! Synchronous mode allows the user to quickly change the state of multiple pins
//! with a single write operation.

use core::ops::Index;
use core::{marker::PhantomData, ops::IndexMut};
use heapless::Vec;

use super::{pin::OutputMode, port_metadata::IoPortMetadata, Pin};

/// Structure representing synchronous pins array.
///
/// This structure allows to combine pins of a single port into an array
/// of synchronous pins. Synchronous pins can be driven in parallel,
/// and setting their state is always a single write operation.
///
/// The pins can be referenced from inside of this structure, as it implements
/// [`Index`] and [`IndexMut`] traits. Order of the pins passed to
/// [`SynchronousPort::new`] is preserved.
pub struct SynchronousPort<Port: IoPortMetadata, const N: usize> {
    /// Array with pins configured in synchronous mode.
    pins: Vec<Pin<OutputMode>, N>,
    /// Phantom port metadata.
    _port: PhantomData<Port>,
}

/// Enumeration representing errors that can happen while working
/// with synchronous mode.
pub enum SynchronousModeError {
    /// Invalid pin was tried to be used while creating new SynchronousPins instance.
    InvalidPin,
}

impl<Port: IoPortMetadata, const N: usize> SynchronousPort<Port, N> {
    /// Creates new instance of SynchronousPins. Enables synchronous mode for passed pins.
    ///
    /// # Parameters
    /// * `pins` - Array of pins used in synchronous mode. **All the pins must belong
    ///            to the same port**
    ///
    /// # Returns
    /// `Ok(Self)` on successful creation, `Err([SynchronousModeError::InvalidPin])` if
    /// all the pins aren't owned by the same port.
    pub fn new(mut pins: [Pin<OutputMode>; N]) -> Result<Self, SynchronousModeError> {
        for pin in &mut pins {
            if pin.port_id() != Port::ID {
                return Err(SynchronousModeError::InvalidPin);
            }

            pin.enable_synchronous_mode();
        }

        Ok(Self {
            pins: pins.into_iter().collect(),
            _port: PhantomData,
        })
    }

    /// Splits SynchronousPins instance into pins array, returning control over them to the user.
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
