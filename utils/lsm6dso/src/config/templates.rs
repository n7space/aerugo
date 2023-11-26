/// Macro creating an bitfield enum with methods for converting it from/to register value.
macro_rules! register_enum {
    ($name:ident [mask = $mask:literal, offset = $offset:literal] { $( $(#[doc = $doc:expr])? $variant_name:ident = $variant_value:literal,)+ }) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, derive_more::TryFrom)]
        #[try_from(repr)]
        #[repr(u8)]
        pub enum $name {
            $(
                $(#[doc = $doc])?
                $variant_name = $variant_value,
            )+
        }

        impl crate::registers::RegisterField for $name {
            const MASK: u8 = $mask;
            const OFFSET: usize = $offset;
        }

        impl crate::registers::ToRegister for $name {
            fn to_reg(self) -> u8 {
                use crate::registers::RegisterField;
                (self as u8) << Self::OFFSET
            }
        }

        impl crate::registers::FromRegister for $name {
            fn from_reg(reg: u8) -> Self {
                use crate::registers::RegisterField;
                ((reg & Self::MASK) >> Self::OFFSET).try_into().unwrap()
            }
        }

        impl crate::registers::ApplyToRegister for $name
            where Self: crate::registers::ToRegister
        {
            fn apply_to_reg(self, reg: u8) -> u8 {
                use crate::registers::{RegisterField, ToRegister};
                (reg & !Self::MASK) | self.to_reg()
            }
        }
    };
}

pub(crate) use register_enum;
