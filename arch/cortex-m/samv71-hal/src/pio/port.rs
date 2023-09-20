//! Module containing Parallel I/O (PIO) port items.

use core::marker::PhantomData;

use super::{pin::ResetMode, port_metadata::IoPortMetadata, Pin};

/// Structure representing a generic I/O port.
///
/// Instances of this structure can be consumed and split into pins array.
///
/// # Generic parameters
/// * `PortMetadata` - PAC PIO port type. This type indicates which PIO port
///                    is represented by Port instance.
pub struct Port<PortMetadata: IoPortMetadata> {
    /// Port marker.
    _port_meta: PhantomData<PortMetadata>,
}

/// Type representing pins of a port.
/// Value of this type can be obtained via [`Port::into_pins`].
pub type Pins = [Option<Pin<ResetMode>>; 32];

impl<Instance: IoPortMetadata> Port<Instance> {
    /// Creates new I/O Port instance from PAC PIO Port structure.
    ///
    /// # Parameters
    /// * `_instance` - PAC PIO Port instance, consumed on construction to prevent
    ///                 creation of duplicate Port instances. Not used otherwise.
    pub fn new(_instance: Instance) -> Self {
        Self {
            _port_meta: PhantomData,
        }
    }

    /// Returns ID (uppercase letter) of the port.
    pub const fn id(&self) -> char {
        Instance::ID
    }

    /// Consumes port instance and returns it's pins.
    pub const fn into_pins(self) -> Pins {
        [
            Some(Pin::new(&self, 0)),
            Some(Pin::new(&self, 1)),
            Some(Pin::new(&self, 2)),
            Some(Pin::new(&self, 3)),
            Some(Pin::new(&self, 4)),
            Some(Pin::new(&self, 5)),
            Some(Pin::new(&self, 6)),
            Some(Pin::new(&self, 7)),
            Some(Pin::new(&self, 8)),
            Some(Pin::new(&self, 9)),
            Some(Pin::new(&self, 10)),
            Some(Pin::new(&self, 11)),
            Some(Pin::new(&self, 12)),
            Some(Pin::new(&self, 13)),
            Some(Pin::new(&self, 14)),
            Some(Pin::new(&self, 15)),
            Some(Pin::new(&self, 16)),
            Some(Pin::new(&self, 17)),
            Some(Pin::new(&self, 18)),
            Some(Pin::new(&self, 19)),
            Some(Pin::new(&self, 20)),
            Some(Pin::new(&self, 21)),
            Some(Pin::new(&self, 22)),
            Some(Pin::new(&self, 23)),
            Some(Pin::new(&self, 24)),
            Some(Pin::new(&self, 25)),
            Some(Pin::new(&self, 26)),
            Some(Pin::new(&self, 27)),
            Some(Pin::new(&self, 28)),
            Some(Pin::new(&self, 29)),
            Some(Pin::new(&self, 30)),
            Some(Pin::new(&self, 31)),
        ]
    }
}
