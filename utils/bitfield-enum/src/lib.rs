//! You must add `derive_more` (version 1.0.0.beta-6 is recommended) to dependencies if you want to
//! use this library:
//!
//! ```
//! derive_more = { version = "1.0.0-beta.6", features = [
//!    "try_from",
//!    "try_into",
//! ], default-features = false }
//! ```

#![no_std]
pub extern crate derive_more;

#[macro_export]
/// Macro creating an bitfield enum with methods for converting it from/to bitfield value.
macro_rules! bitfield_enum {
    ($name:ident [mask = $mask:literal, offset = $offset:literal] { $( $(#[doc = $doc:expr])? $variant_name:ident = $variant_value:literal,)+ }) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, $crate::derive_more::TryFrom)]
        #[try_from(repr)]
        #[repr(u8)]
        pub enum $name {
            $(
                $(#[doc = $doc])?
                $variant_name = $variant_value,
            )+
        }

        impl $crate::BitField for $name {
            const MASK: u8 = $mask;
            const OFFSET: usize = $offset;
        }

        impl $crate::BitFieldToByte for $name {
            fn to_byte(self) -> u8 {
                use $crate::BitField;
                (self as u8) << Self::OFFSET
            }
        }

        impl $crate::BitFieldFromByte for $name {
            fn from_byte(byte: u8) -> Self {
                use $crate::BitField;
                ((byte & Self::MASK) >> Self::OFFSET).try_into().unwrap()
            }
        }

        impl $crate::ApplyBitFieldToByte for $name
            where Self: $crate::BitField
        {
            fn apply_to_byte(self, byte: u8) -> u8 {
                use $crate::{BitField, BitFieldToByte};
                (byte & !Self::MASK) | self.to_byte()
            }
        }
    };
}

/// Trait for single-byte bitfields
pub trait BitField
where
    Self: Copy,
{
    // Bitfield mask.
    const MASK: u8;
    /// Offset of the field's LSB.
    const OFFSET: usize;
}

pub trait BitFieldToByte
where
    Self: Copy + BitField,
{
    /// This function should return the value of current bitfield with unused bits set to 0.
    fn to_byte(self) -> u8;
}

pub trait BitFieldFromByte
where
    Self: Copy + BitField,
{
    /// This function should extract the field's value from the bitfield and return it.
    fn from_byte(byte: u8) -> Self;
}

pub trait ApplyBitFieldToByte
where
    Self: Copy + BitField,
{
    /// This function modifies existing byte's bits and returns it's new value with applied bitfield.
    fn apply_to_byte(self, byte: u8) -> u8;
}

/// Trait for bitfields that span multiple bytes. The order of masks and offsets must be defined
/// respective to the order of bytes this field spans.
pub trait MultiByteBitField<const BITFIELD_SPAN: usize = 2>
where
    Self: Copy,
{
    /// Field masks.
    const MASKS: [u8; BITFIELD_SPAN];
    /// Offsets of the field's LSB.
    const OFFSETS: [usize; BITFIELD_SPAN];
}

pub trait MultiByteBitFieldToBytes<const BITFIELD_SPAN: usize = 2>
where
    Self: Copy + MultiByteBitField<BITFIELD_SPAN>,
{
    /// This function should return the value of provided bitfield as an array. Unused bits should
    /// remain 0.
    fn to_bytes(self) -> [u8; BITFIELD_SPAN];
}

pub trait MultiByteBitFieldFromBytes<const BITFIELD_SPAN: usize = 2>
where
    Self: Copy + MultiByteBitField<BITFIELD_SPAN>,
{
    /// This function should extract the field's value from the bitfield and return it.
    fn from_bytes(bytes: &[u8]) -> Self;
}
