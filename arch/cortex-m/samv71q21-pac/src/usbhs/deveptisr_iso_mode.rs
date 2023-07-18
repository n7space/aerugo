#[doc = "Register `DEVEPTISR_ISO_MODE[%s]` reader"]
pub struct R(crate::R<DEVEPTISR_ISO_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVEPTISR_ISO_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVEPTISR_ISO_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVEPTISR_ISO_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXINI` reader - Transmitted IN Data Interrupt"]
pub type TXINI_R = crate::BitReader;
#[doc = "Field `RXOUTI` reader - Received OUT Data Interrupt"]
pub type RXOUTI_R = crate::BitReader;
#[doc = "Field `UNDERFI` reader - Underflow Interrupt"]
pub type UNDERFI_R = crate::BitReader;
#[doc = "Field `HBISOINERRI` reader - High Bandwidth Isochronous IN Underflow Error Interrupt"]
pub type HBISOINERRI_R = crate::BitReader;
#[doc = "Field `HBISOFLUSHI` reader - High Bandwidth Isochronous IN Flush Interrupt"]
pub type HBISOFLUSHI_R = crate::BitReader;
#[doc = "Field `OVERFI` reader - Overflow Interrupt"]
pub type OVERFI_R = crate::BitReader;
#[doc = "Field `CRCERRI` reader - CRC Error Interrupt"]
pub type CRCERRI_R = crate::BitReader;
#[doc = "Field `SHORTPACKET` reader - Short Packet Interrupt"]
pub type SHORTPACKET_R = crate::BitReader;
#[doc = "Field `DTSEQ` reader - Data Toggle Sequence"]
pub type DTSEQ_R = crate::FieldReader<DTSEQSELECT_A>;
#[doc = "Data Toggle Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTSEQSELECT_A {
    #[doc = "0: Data0 toggle sequence"]
    DATA0 = 0,
    #[doc = "1: Data1 toggle sequence"]
    DATA1 = 1,
    #[doc = "2: Reserved for high-bandwidth isochronous endpoint"]
    DATA2 = 2,
    #[doc = "3: Reserved for high-bandwidth isochronous endpoint"]
    MDATA = 3,
}
impl From<DTSEQSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DTSEQSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTSEQSELECT_A {
    type Ux = u8;
}
impl DTSEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTSEQSELECT_A {
        match self.bits {
            0 => DTSEQSELECT_A::DATA0,
            1 => DTSEQSELECT_A::DATA1,
            2 => DTSEQSELECT_A::DATA2,
            3 => DTSEQSELECT_A::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DTSEQSELECT_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DTSEQSELECT_A::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == DTSEQSELECT_A::DATA2
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == DTSEQSELECT_A::MDATA
    }
}
#[doc = "Field `ERRORTRANS` reader - High-bandwidth Isochronous OUT Endpoint Transaction Error Interrupt"]
pub type ERRORTRANS_R = crate::BitReader;
#[doc = "Field `NBUSYBK` reader - Number of Busy Banks"]
pub type NBUSYBK_R = crate::FieldReader<NBUSYBKSELECT_A>;
#[doc = "Number of Busy Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NBUSYBKSELECT_A {
    #[doc = "0: 0 busy bank (all banks free)"]
    _0_BUSY = 0,
    #[doc = "1: 1 busy bank"]
    _1_BUSY = 1,
    #[doc = "2: 2 busy banks"]
    _2_BUSY = 2,
    #[doc = "3: 3 busy banks"]
    _3_BUSY = 3,
}
impl From<NBUSYBKSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NBUSYBKSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NBUSYBKSELECT_A {
    type Ux = u8;
}
impl NBUSYBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NBUSYBKSELECT_A {
        match self.bits {
            0 => NBUSYBKSELECT_A::_0_BUSY,
            1 => NBUSYBKSELECT_A::_1_BUSY,
            2 => NBUSYBKSELECT_A::_2_BUSY,
            3 => NBUSYBKSELECT_A::_3_BUSY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_BUSY`"]
    #[inline(always)]
    pub fn is_0_busy(&self) -> bool {
        *self == NBUSYBKSELECT_A::_0_BUSY
    }
    #[doc = "Checks if the value of the field is `_1_BUSY`"]
    #[inline(always)]
    pub fn is_1_busy(&self) -> bool {
        *self == NBUSYBKSELECT_A::_1_BUSY
    }
    #[doc = "Checks if the value of the field is `_2_BUSY`"]
    #[inline(always)]
    pub fn is_2_busy(&self) -> bool {
        *self == NBUSYBKSELECT_A::_2_BUSY
    }
    #[doc = "Checks if the value of the field is `_3_BUSY`"]
    #[inline(always)]
    pub fn is_3_busy(&self) -> bool {
        *self == NBUSYBKSELECT_A::_3_BUSY
    }
}
#[doc = "Field `CURRBK` reader - Current Bank"]
pub type CURRBK_R = crate::FieldReader<CURRBKSELECT_A>;
#[doc = "Current Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CURRBKSELECT_A {
    #[doc = "0: Current bank is bank0"]
    BANK0 = 0,
    #[doc = "1: Current bank is bank1"]
    BANK1 = 1,
    #[doc = "2: Current bank is bank2"]
    BANK2 = 2,
}
impl From<CURRBKSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CURRBKSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CURRBKSELECT_A {
    type Ux = u8;
}
impl CURRBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CURRBKSELECT_A> {
        match self.bits {
            0 => Some(CURRBKSELECT_A::BANK0),
            1 => Some(CURRBKSELECT_A::BANK1),
            2 => Some(CURRBKSELECT_A::BANK2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == CURRBKSELECT_A::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == CURRBKSELECT_A::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == CURRBKSELECT_A::BANK2
    }
}
#[doc = "Field `RWALL` reader - Read/Write Allowed"]
pub type RWALL_R = crate::BitReader;
#[doc = "Field `CFGOK` reader - Configuration OK Status"]
pub type CFGOK_R = crate::BitReader;
#[doc = "Field `BYCT` reader - Byte Count"]
pub type BYCT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txini(&self) -> TXINI_R {
        TXINI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxouti(&self) -> RXOUTI_R {
        RXOUTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt"]
    #[inline(always)]
    pub fn underfi(&self) -> UNDERFI_R {
        UNDERFI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt"]
    #[inline(always)]
    pub fn hbisoinerri(&self) -> HBISOINERRI_R {
        HBISOINERRI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt"]
    #[inline(always)]
    pub fn hbisoflushi(&self) -> HBISOFLUSHI_R {
        HBISOFLUSHI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfi(&self) -> OVERFI_R {
        OVERFI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC Error Interrupt"]
    #[inline(always)]
    pub fn crcerri(&self) -> CRCERRI_R {
        CRCERRI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpacket(&self) -> SHORTPACKET_R {
        SHORTPACKET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - High-bandwidth Isochronous OUT Endpoint Transaction Error Interrupt"]
    #[inline(always)]
    pub fn errortrans(&self) -> ERRORTRANS_R {
        ERRORTRANS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Number of Busy Banks"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Read/Write Allowed"]
    #[inline(always)]
    pub fn rwall(&self) -> RWALL_R {
        RWALL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration OK Status"]
    #[inline(always)]
    pub fn cfgok(&self) -> CFGOK_R {
        CFGOK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:30 - Byte Count"]
    #[inline(always)]
    pub fn byct(&self) -> BYCT_R {
        BYCT_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
}
#[doc = "Device Endpoint Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptisr_iso_mode](index.html) module"]
pub struct DEVEPTISR_ISO_MODE_SPEC;
impl crate::RegisterSpec for DEVEPTISR_ISO_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deveptisr_iso_mode::R](R) reader structure"]
impl crate::Readable for DEVEPTISR_ISO_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVEPTISR_ISO_MODE[%s]
to value 0"]
impl crate::Resettable for DEVEPTISR_ISO_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
