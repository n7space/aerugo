#[doc = "Register `CIDR` reader"]
pub struct R(crate::R<CIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - Version of the Device"]
pub type VERSION_R = crate::FieldReader;
#[doc = "Field `EPROC` reader - Embedded Processor"]
pub type EPROC_R = crate::FieldReader<EPROCSELECT_A>;
#[doc = "Embedded Processor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPROCSELECT_A {
    #[doc = "0: Cortex-M7"]
    SAMX7 = 0,
    #[doc = "1: ARM946ES"]
    ARM946ES = 1,
    #[doc = "2: ARM7TDMI"]
    ARM7TDMI = 2,
    #[doc = "3: Cortex-M3"]
    CM3 = 3,
    #[doc = "4: ARM920T"]
    ARM920T = 4,
    #[doc = "5: ARM926EJS"]
    ARM926EJS = 5,
    #[doc = "6: Cortex-A5"]
    CA5 = 6,
    #[doc = "7: Cortex-M4"]
    CM4 = 7,
}
impl From<EPROCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EPROCSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPROCSELECT_A {
    type Ux = u8;
}
impl EPROC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPROCSELECT_A {
        match self.bits {
            0 => EPROCSELECT_A::SAMX7,
            1 => EPROCSELECT_A::ARM946ES,
            2 => EPROCSELECT_A::ARM7TDMI,
            3 => EPROCSELECT_A::CM3,
            4 => EPROCSELECT_A::ARM920T,
            5 => EPROCSELECT_A::ARM926EJS,
            6 => EPROCSELECT_A::CA5,
            7 => EPROCSELECT_A::CM4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAMX7`"]
    #[inline(always)]
    pub fn is_samx7(&self) -> bool {
        *self == EPROCSELECT_A::SAMX7
    }
    #[doc = "Checks if the value of the field is `ARM946ES`"]
    #[inline(always)]
    pub fn is_arm946es(&self) -> bool {
        *self == EPROCSELECT_A::ARM946ES
    }
    #[doc = "Checks if the value of the field is `ARM7TDMI`"]
    #[inline(always)]
    pub fn is_arm7tdmi(&self) -> bool {
        *self == EPROCSELECT_A::ARM7TDMI
    }
    #[doc = "Checks if the value of the field is `CM3`"]
    #[inline(always)]
    pub fn is_cm3(&self) -> bool {
        *self == EPROCSELECT_A::CM3
    }
    #[doc = "Checks if the value of the field is `ARM920T`"]
    #[inline(always)]
    pub fn is_arm920t(&self) -> bool {
        *self == EPROCSELECT_A::ARM920T
    }
    #[doc = "Checks if the value of the field is `ARM926EJS`"]
    #[inline(always)]
    pub fn is_arm926ejs(&self) -> bool {
        *self == EPROCSELECT_A::ARM926EJS
    }
    #[doc = "Checks if the value of the field is `CA5`"]
    #[inline(always)]
    pub fn is_ca5(&self) -> bool {
        *self == EPROCSELECT_A::CA5
    }
    #[doc = "Checks if the value of the field is `CM4`"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == EPROCSELECT_A::CM4
    }
}
#[doc = "Field `NVPSIZ` reader - Nonvolatile Program Memory Size"]
pub type NVPSIZ_R = crate::FieldReader<NVPSIZSELECT_A>;
#[doc = "Nonvolatile Program Memory Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVPSIZSELECT_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: 8 Kbytes"]
    _8K = 1,
    #[doc = "2: 16 Kbytes"]
    _16K = 2,
    #[doc = "3: 32 Kbytes"]
    _32K = 3,
    #[doc = "5: 64 Kbytes"]
    _64K = 5,
    #[doc = "7: 128 Kbytes"]
    _128K = 7,
    #[doc = "8: 160 Kbytes"]
    _160K = 8,
    #[doc = "9: 256 Kbytes"]
    _256K = 9,
    #[doc = "10: 512 Kbytes"]
    _512K = 10,
    #[doc = "12: 1024 Kbytes"]
    _1024K = 12,
    #[doc = "14: 2048 Kbytes"]
    _2048K = 14,
}
impl From<NVPSIZSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPSIZSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NVPSIZSELECT_A {
    type Ux = u8;
}
impl NVPSIZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVPSIZSELECT_A> {
        match self.bits {
            0 => Some(NVPSIZSELECT_A::NONE),
            1 => Some(NVPSIZSELECT_A::_8K),
            2 => Some(NVPSIZSELECT_A::_16K),
            3 => Some(NVPSIZSELECT_A::_32K),
            5 => Some(NVPSIZSELECT_A::_64K),
            7 => Some(NVPSIZSELECT_A::_128K),
            8 => Some(NVPSIZSELECT_A::_160K),
            9 => Some(NVPSIZSELECT_A::_256K),
            10 => Some(NVPSIZSELECT_A::_512K),
            12 => Some(NVPSIZSELECT_A::_1024K),
            14 => Some(NVPSIZSELECT_A::_2048K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZSELECT_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZSELECT_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZSELECT_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZSELECT_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZSELECT_A::_128K
    }
    #[doc = "Checks if the value of the field is `_160K`"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        *self == NVPSIZSELECT_A::_160K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZSELECT_A::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZSELECT_A::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZSELECT_A::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZSELECT_A::_2048K
    }
}
#[doc = "Field `NVPSIZ2` reader - Second Nonvolatile Program Memory Size"]
pub type NVPSIZ2_R = crate::FieldReader<NVPSIZ2SELECT_A>;
#[doc = "Second Nonvolatile Program Memory Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVPSIZ2SELECT_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: 8 Kbytes"]
    _8K = 1,
    #[doc = "2: 16 Kbytes"]
    _16K = 2,
    #[doc = "3: 32 Kbytes"]
    _32K = 3,
    #[doc = "5: 64 Kbytes"]
    _64K = 5,
    #[doc = "7: 128 Kbytes"]
    _128K = 7,
    #[doc = "9: 256 Kbytes"]
    _256K = 9,
    #[doc = "10: 512 Kbytes"]
    _512K = 10,
    #[doc = "12: 1024 Kbytes"]
    _1024K = 12,
    #[doc = "14: 2048 Kbytes"]
    _2048K = 14,
}
impl From<NVPSIZ2SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPSIZ2SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NVPSIZ2SELECT_A {
    type Ux = u8;
}
impl NVPSIZ2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVPSIZ2SELECT_A> {
        match self.bits {
            0 => Some(NVPSIZ2SELECT_A::NONE),
            1 => Some(NVPSIZ2SELECT_A::_8K),
            2 => Some(NVPSIZ2SELECT_A::_16K),
            3 => Some(NVPSIZ2SELECT_A::_32K),
            5 => Some(NVPSIZ2SELECT_A::_64K),
            7 => Some(NVPSIZ2SELECT_A::_128K),
            9 => Some(NVPSIZ2SELECT_A::_256K),
            10 => Some(NVPSIZ2SELECT_A::_512K),
            12 => Some(NVPSIZ2SELECT_A::_1024K),
            14 => Some(NVPSIZ2SELECT_A::_2048K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZ2SELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZ2SELECT_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZ2SELECT_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZ2SELECT_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZ2SELECT_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZ2SELECT_A::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZ2SELECT_A::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZ2SELECT_A::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZ2SELECT_A::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZ2SELECT_A::_2048K
    }
}
#[doc = "Field `SRAMSIZ` reader - Internal SRAM Size"]
pub type SRAMSIZ_R = crate::FieldReader<SRAMSIZSELECT_A>;
#[doc = "Internal SRAM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRAMSIZSELECT_A {
    #[doc = "0: 48 Kbytes"]
    _48K = 0,
    #[doc = "1: 192 Kbytes"]
    _192K = 1,
    #[doc = "2: 384 Kbytes"]
    _384K = 2,
    #[doc = "3: 6 Kbytes"]
    _6K = 3,
    #[doc = "4: 24 Kbytes"]
    _24K = 4,
    #[doc = "5: 4 Kbytes"]
    _4K = 5,
    #[doc = "6: 80 Kbytes"]
    _80K = 6,
    #[doc = "7: 160 Kbytes"]
    _160K = 7,
    #[doc = "8: 8 Kbytes"]
    _8K = 8,
    #[doc = "9: 16 Kbytes"]
    _16K = 9,
    #[doc = "10: 32 Kbytes"]
    _32K = 10,
    #[doc = "11: 64 Kbytes"]
    _64K = 11,
    #[doc = "12: 128 Kbytes"]
    _128K = 12,
    #[doc = "13: 256 Kbytes"]
    _256K = 13,
    #[doc = "14: 96 Kbytes"]
    _96K = 14,
    #[doc = "15: 512 Kbytes"]
    _512K = 15,
}
impl From<SRAMSIZSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMSIZSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRAMSIZSELECT_A {
    type Ux = u8;
}
impl SRAMSIZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMSIZSELECT_A {
        match self.bits {
            0 => SRAMSIZSELECT_A::_48K,
            1 => SRAMSIZSELECT_A::_192K,
            2 => SRAMSIZSELECT_A::_384K,
            3 => SRAMSIZSELECT_A::_6K,
            4 => SRAMSIZSELECT_A::_24K,
            5 => SRAMSIZSELECT_A::_4K,
            6 => SRAMSIZSELECT_A::_80K,
            7 => SRAMSIZSELECT_A::_160K,
            8 => SRAMSIZSELECT_A::_8K,
            9 => SRAMSIZSELECT_A::_16K,
            10 => SRAMSIZSELECT_A::_32K,
            11 => SRAMSIZSELECT_A::_64K,
            12 => SRAMSIZSELECT_A::_128K,
            13 => SRAMSIZSELECT_A::_256K,
            14 => SRAMSIZSELECT_A::_96K,
            15 => SRAMSIZSELECT_A::_512K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_48K`"]
    #[inline(always)]
    pub fn is_48k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_48K
    }
    #[doc = "Checks if the value of the field is `_192K`"]
    #[inline(always)]
    pub fn is_192k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_192K
    }
    #[doc = "Checks if the value of the field is `_384K`"]
    #[inline(always)]
    pub fn is_384k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_384K
    }
    #[doc = "Checks if the value of the field is `_6K`"]
    #[inline(always)]
    pub fn is_6k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_6K
    }
    #[doc = "Checks if the value of the field is `_24K`"]
    #[inline(always)]
    pub fn is_24k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_24K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_4K
    }
    #[doc = "Checks if the value of the field is `_80K`"]
    #[inline(always)]
    pub fn is_80k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_80K
    }
    #[doc = "Checks if the value of the field is `_160K`"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_160K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_256K
    }
    #[doc = "Checks if the value of the field is `_96K`"]
    #[inline(always)]
    pub fn is_96k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_96K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == SRAMSIZSELECT_A::_512K
    }
}
#[doc = "Field `ARCH` reader - Architecture Identifier"]
pub type ARCH_R = crate::FieldReader<ARCHSELECT_A>;
#[doc = "Architecture Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARCHSELECT_A {
    #[doc = "16: SAM E70"]
    SAME70 = 16,
    #[doc = "17: SAM S70"]
    SAMS70 = 17,
    #[doc = "18: SAM V71"]
    SAMV71 = 18,
    #[doc = "19: SAM V70"]
    SAMV70 = 19,
}
impl From<ARCHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ARCHSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARCHSELECT_A {
    type Ux = u8;
}
impl ARCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARCHSELECT_A> {
        match self.bits {
            16 => Some(ARCHSELECT_A::SAME70),
            17 => Some(ARCHSELECT_A::SAMS70),
            18 => Some(ARCHSELECT_A::SAMV71),
            19 => Some(ARCHSELECT_A::SAMV70),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SAME70`"]
    #[inline(always)]
    pub fn is_same70(&self) -> bool {
        *self == ARCHSELECT_A::SAME70
    }
    #[doc = "Checks if the value of the field is `SAMS70`"]
    #[inline(always)]
    pub fn is_sams70(&self) -> bool {
        *self == ARCHSELECT_A::SAMS70
    }
    #[doc = "Checks if the value of the field is `SAMV71`"]
    #[inline(always)]
    pub fn is_samv71(&self) -> bool {
        *self == ARCHSELECT_A::SAMV71
    }
    #[doc = "Checks if the value of the field is `SAMV70`"]
    #[inline(always)]
    pub fn is_samv70(&self) -> bool {
        *self == ARCHSELECT_A::SAMV70
    }
}
#[doc = "Field `NVPTYP` reader - Nonvolatile Program Memory Type"]
pub type NVPTYP_R = crate::FieldReader<NVPTYPSELECT_A>;
#[doc = "Nonvolatile Program Memory Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVPTYPSELECT_A {
    #[doc = "0: ROM"]
    ROM = 0,
    #[doc = "1: ROMless or on-chip Flash"]
    ROMLESS = 1,
    #[doc = "2: Embedded Flash Memory"]
    FLASH = 2,
    #[doc = "3: ROM and Embedded Flash Memory- NVPSIZ is ROM size- NVPSIZ2 is Flash size"]
    ROM_FLASH = 3,
    #[doc = "4: SRAM emulating ROM"]
    SRAM = 4,
}
impl From<NVPTYPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPTYPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NVPTYPSELECT_A {
    type Ux = u8;
}
impl NVPTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVPTYPSELECT_A> {
        match self.bits {
            0 => Some(NVPTYPSELECT_A::ROM),
            1 => Some(NVPTYPSELECT_A::ROMLESS),
            2 => Some(NVPTYPSELECT_A::FLASH),
            3 => Some(NVPTYPSELECT_A::ROM_FLASH),
            4 => Some(NVPTYPSELECT_A::SRAM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ROM`"]
    #[inline(always)]
    pub fn is_rom(&self) -> bool {
        *self == NVPTYPSELECT_A::ROM
    }
    #[doc = "Checks if the value of the field is `ROMLESS`"]
    #[inline(always)]
    pub fn is_romless(&self) -> bool {
        *self == NVPTYPSELECT_A::ROMLESS
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == NVPTYPSELECT_A::FLASH
    }
    #[doc = "Checks if the value of the field is `ROM_FLASH`"]
    #[inline(always)]
    pub fn is_rom_flash(&self) -> bool {
        *self == NVPTYPSELECT_A::ROM_FLASH
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == NVPTYPSELECT_A::SRAM
    }
}
#[doc = "Field `EXT` reader - Extension Flag"]
pub type EXT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline(always)]
    pub fn eproc(&self) -> EPROC_R {
        EPROC_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz(&self) -> NVPSIZ_R {
        NVPSIZ_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Second Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz2(&self) -> NVPSIZ2_R {
        NVPSIZ2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Internal SRAM Size"]
    #[inline(always)]
    pub fn sramsiz(&self) -> SRAMSIZ_R {
        SRAMSIZ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Architecture Identifier"]
    #[inline(always)]
    pub fn arch(&self) -> ARCH_R {
        ARCH_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline(always)]
    pub fn nvptyp(&self) -> NVPTYP_R {
        NVPTYP_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Chip ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr](index.html) module"]
pub struct CIDR_SPEC;
impl crate::RegisterSpec for CIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr::R](R) reader structure"]
impl crate::Readable for CIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIDR to value 0"]
impl crate::Resettable for CIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
