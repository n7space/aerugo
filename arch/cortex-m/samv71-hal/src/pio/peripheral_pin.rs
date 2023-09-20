//! Module containing Parallel I/O (PIO) pin items for peripheral-controlled I/O pin.

use super::{pin::Peripheral, port_metadata::IoPortMetadata, Pin};

/// Peripheral-controlled pin's implementation.
impl<Port: IoPortMetadata, const ID: u8> Pin<Port, ID, Peripheral> {}
