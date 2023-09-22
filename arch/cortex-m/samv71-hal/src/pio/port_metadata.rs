//! Module containing meta-traits for PIO port that enable type erasure.
//!
//! I/O ports are represented in PAC as separate structures for each port.
//! This design can be cumbersome to use, as all of these ports share exactly the same
//! functionality and register layout, but since they have different types, user must
//! explicitly specify which port he wants to use every time. There's no way of writing
//! a generic function or structure that would work with any of available PAC PIO ports.
//!
//! Traits from this module provide common interface for all the ports, which makes
//! implementing a generic type that will accept any PAC PIO port possible.

pub(super) use crate::pac::pioa::RegisterBlock;
pub use crate::pac::{PIOA, PIOB, PIOC, PIOD, PIOE};

/// Trait for PAC PIO port instances.
///
/// This trait erases the type of PIO port instance, so it can be used as generic argument for [`Port`](super::Port) instead of concrete port type.
pub trait IoPortMetadata {
    /// Pointer to the register block of I/O port.
    const REGISTERS: *const RegisterBlock;
    /// Letter identifying the port ('A' for PIOA, and so on).
    const ID: char;
}

/// Internal macro used to generate IoPortMetadata implementations for every available PIO peripheral.
macro_rules! implement_io_port_metadata_for {
    ($pio:ty, $name:literal) => {
        impl IoPortMetadata for $pio {
            const REGISTERS: *const RegisterBlock = <$pio>::PTR;
            const ID: char = $name;
        }
    };
}

implement_io_port_metadata_for!(PIOA, 'A');
implement_io_port_metadata_for!(PIOB, 'B');
implement_io_port_metadata_for!(PIOC, 'C');
implement_io_port_metadata_for!(PIOD, 'D');
implement_io_port_metadata_for!(PIOE, 'E');
