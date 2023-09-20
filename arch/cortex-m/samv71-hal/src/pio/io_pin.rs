//! Module containing Parallel I/O (PIO) pin items for PIO-controlled I/O pin.

use super::{pin::IO, port_metadata::IoPortMetadata, Pin};

/// PIO-controlled pin's implementation.
impl<Port: IoPortMetadata, const ID: u8> Pin<Port, ID, IO> {}
