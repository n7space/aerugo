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
/// Macro creating an bitfield enum with methods for converting it from/to register value.
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

        impl $crate::RegisterField for $name {
            const MASK: u8 = $mask;
            const OFFSET: usize = $offset;
        }

        impl $crate::ToRegister for $name {
            fn to_reg(self) -> u8 {
                use $crate::RegisterField;
                (self as u8) << Self::OFFSET
            }
        }

        impl $crate::FromRegister for $name {
            fn from_reg(reg: u8) -> Self {
                use $crate::RegisterField;
                ((reg & Self::MASK) >> Self::OFFSET).try_into().unwrap()
            }
        }

        impl $crate::ApplyToRegister for $name
            where Self: $crate::ToRegister
        {
            fn apply_to_reg(self, reg: u8) -> u8 {
                use $crate::{RegisterField, ToRegister};
                (reg & !Self::MASK) | self.to_reg()
            }
        }
    };
}

/// Trait for single-register fields
pub trait RegisterField
where
    Self: Copy,
{
    /// Field mask, per datasheet (as-in register).
    const MASK: u8;
    /// Offset of the field's LSB to register's LSB.
    const OFFSET: usize;
}

pub trait ToRegister
where
    Self: Copy + RegisterField,
{
    /// This function should return the value of current register field that can be OR'd with
    /// register's content to set it.
    fn to_reg(self) -> u8;
}

pub trait FromRegister
where
    Self: Copy + RegisterField,
{
    /// This function should extract the field's value from the register and return it.
    fn from_reg(reg: u8) -> Self;
}

pub trait ApplyToRegister
where
    Self: Copy + RegisterField,
{
    /// This function modifies existing register's bits and returns it's new value with applied field.
    fn apply_to_reg(self, reg: u8) -> u8;
}

/// Trait for fields that span multiple registers. The order of masks and offsets must be defined
/// respective to the order of registers this field spans, smaller address first.
pub trait MultiRegisterField<const REGISTER_SPAN: usize = 2>
where
    Self: Copy,
{
    /// Field masks, per datasheet (as-in register).
    const MASKS: [u8; REGISTER_SPAN];
    /// Offsets of the field's LSB from register's LSB.
    const OFFSETS: [usize; REGISTER_SPAN];
}

pub trait ToMultiRegister<const REGISTER_SPAN: usize = 2>
where
    Self: Copy + MultiRegisterField<REGISTER_SPAN>,
{
    /// This function should return the value of provided registers as an array. Unused bits should
    /// remain 0.
    fn to_regs(self) -> [u8; REGISTER_SPAN];
}

pub trait FromMultiRegister<const REGISTER_SPAN: usize = 2>
where
    Self: Copy + MultiRegisterField<REGISTER_SPAN>,
{
    /// This function should extract the field's value from the register and return it.
    fn from_regs(regs: &[u8]) -> Self;
}
