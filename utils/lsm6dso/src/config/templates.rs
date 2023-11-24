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

        impl crate::registers::RegisterConversion for $name {
            fn to_reg(self) -> u8 {
                (self as u8) << Self::OFFSET
            }

            fn from_reg(reg: u8) -> Self {
                ((reg & Self::MASK) >> Self::OFFSET).try_into().unwrap()
            }
        }
    };
}

pub(crate) use register_enum;
