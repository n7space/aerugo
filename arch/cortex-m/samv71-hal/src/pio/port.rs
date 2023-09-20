//! Module containing Parallel I/O (PIO) port items.

use core::marker::PhantomData;

use super::port_metadata::IoPortMetadata;

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
}

/// Extension trait which allows the port to be split into pins.
///
/// This trait should be implemented via macro to avoid unnecessary repetition of code,
/// as each port will have pins with different generic types.
pub trait IntoPins {
    /// Type representing a structure with I/O pins of port implementing this trait.
    type Pins;

    /// Consumes current port instance and returns structure containing pins associated with consumed port.
    fn into_pins(self) -> Self::Pins;
}

/// Helper macro used to define Pins structure for specified PIO port and implementation of IntoPins trait
/// for specified PIO port.
///
/// Should only be used internally by this module.
macro_rules! implement_into_pins_for_port {
    ($pins_structure_name:ident, $port_type:ident, $port_letter:ident, [$($pin:literal),*]) => {
        paste::paste! {
            #[doc = "Structure containing all port " $port_letter:upper " pins."]
            pub struct $pins_structure_name<Port: super::IoPortMetadata> {
            $(
                #[doc = "Pin " $pin " of port " $port_letter:upper "."]
                pub [<p$port_letter:lower$pin>]: Pin<$port_type, $pin, PostReset>,
            )*
                #[doc = "Port " $port_letter:upper " metadata."]
                _port_meta: core::marker::PhantomData<Port>,
            }

            impl super::IntoPins for super::Port<$port_type> {
                type Pins = Pins<$port_type>;

                fn into_pins(self) -> Self::Pins {
                    $pins_structure_name {
                    $(
                        [<p$port_letter:lower$pin>]: Pin::new(),
                    )*
                        _port_meta: core::marker::PhantomData
                    }
                }
            }
        }
    };
}

/// Helper macro used to generate implementation of IntoPins trait for specified PIO port.
///
/// Should only be used internally by this module.
///
/// By passing a single letter (case doesn't matter, as it's always explicitly stated), a module with implementation
/// of IntoPins trait for PIOx port is generated, where `x` is the passed letter.
macro_rules! generate_port_implementation {
    ($port_letter:ident) => {
        paste::paste! {
            #[doc = "Module containing IntoPins implementation for port " $port_letter:upper]
            pub mod [<port_$port_letter:lower>] {
                use crate::pac::[<PIO$port_letter:upper>];
                use crate::pio::pin::{Pin, PostReset};

                implement_into_pins_for_port!(Pins, [<PIO$port_letter:upper>], [<$port_letter:lower>], [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31]);
            }
        }
    };
}

generate_port_implementation!(A);
generate_port_implementation!(B);
generate_port_implementation!(C);
generate_port_implementation!(D);
generate_port_implementation!(E);
