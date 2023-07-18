#[doc = "Register `DEVEPTCFG[%s]` reader"]
pub struct R(crate::R<DEVEPTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVEPTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVEPTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVEPTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVEPTCFG[%s]` writer"]
pub struct W(crate::W<DEVEPTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVEPTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DEVEPTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVEPTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALLOC` reader - Endpoint Memory Allocate"]
pub type ALLOC_R = crate::BitReader;
#[doc = "Field `ALLOC` writer - Endpoint Memory Allocate"]
pub type ALLOC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTCFG_SPEC, O>;
#[doc = "Field `EPBK` reader - Endpoint Banks"]
pub type EPBK_R = crate::FieldReader<EPBKSELECT_A>;
#[doc = "Endpoint Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPBKSELECT_A {
    #[doc = "0: Single-bank endpoint"]
    _1_BANK = 0,
    #[doc = "1: Double-bank endpoint"]
    _2_BANK = 1,
    #[doc = "2: Triple-bank endpoint"]
    _3_BANK = 2,
}
impl From<EPBKSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EPBKSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPBKSELECT_A {
    type Ux = u8;
}
impl EPBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPBKSELECT_A> {
        match self.bits {
            0 => Some(EPBKSELECT_A::_1_BANK),
            1 => Some(EPBKSELECT_A::_2_BANK),
            2 => Some(EPBKSELECT_A::_3_BANK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1_BANK`"]
    #[inline(always)]
    pub fn is_1_bank(&self) -> bool {
        *self == EPBKSELECT_A::_1_BANK
    }
    #[doc = "Checks if the value of the field is `_2_BANK`"]
    #[inline(always)]
    pub fn is_2_bank(&self) -> bool {
        *self == EPBKSELECT_A::_2_BANK
    }
    #[doc = "Checks if the value of the field is `_3_BANK`"]
    #[inline(always)]
    pub fn is_3_bank(&self) -> bool {
        *self == EPBKSELECT_A::_3_BANK
    }
}
#[doc = "Field `EPBK` writer - Endpoint Banks"]
pub type EPBK_W<'a, const O: u8> = crate::FieldWriter<'a, DEVEPTCFG_SPEC, 2, O, EPBKSELECT_A>;
impl<'a, const O: u8> EPBK_W<'a, O> {
    #[doc = "Single-bank endpoint"]
    #[inline(always)]
    pub fn _1_bank(self) -> &'a mut W {
        self.variant(EPBKSELECT_A::_1_BANK)
    }
    #[doc = "Double-bank endpoint"]
    #[inline(always)]
    pub fn _2_bank(self) -> &'a mut W {
        self.variant(EPBKSELECT_A::_2_BANK)
    }
    #[doc = "Triple-bank endpoint"]
    #[inline(always)]
    pub fn _3_bank(self) -> &'a mut W {
        self.variant(EPBKSELECT_A::_3_BANK)
    }
}
#[doc = "Field `EPSIZE` reader - Endpoint Size"]
pub type EPSIZE_R = crate::FieldReader<EPSIZESELECT_A>;
#[doc = "Endpoint Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPSIZESELECT_A {
    #[doc = "0: 8 bytes"]
    _8_BYTE = 0,
    #[doc = "1: 16 bytes"]
    _16_BYTE = 1,
    #[doc = "2: 32 bytes"]
    _32_BYTE = 2,
    #[doc = "3: 64 bytes"]
    _64_BYTE = 3,
    #[doc = "4: 128 bytes"]
    _128_BYTE = 4,
    #[doc = "5: 256 bytes"]
    _256_BYTE = 5,
    #[doc = "6: 512 bytes"]
    _512_BYTE = 6,
    #[doc = "7: 1024 bytes"]
    _1024_BYTE = 7,
}
impl From<EPSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSIZESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPSIZESELECT_A {
    type Ux = u8;
}
impl EPSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPSIZESELECT_A {
        match self.bits {
            0 => EPSIZESELECT_A::_8_BYTE,
            1 => EPSIZESELECT_A::_16_BYTE,
            2 => EPSIZESELECT_A::_32_BYTE,
            3 => EPSIZESELECT_A::_64_BYTE,
            4 => EPSIZESELECT_A::_128_BYTE,
            5 => EPSIZESELECT_A::_256_BYTE,
            6 => EPSIZESELECT_A::_512_BYTE,
            7 => EPSIZESELECT_A::_1024_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == EPSIZESELECT_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == EPSIZESELECT_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == EPSIZESELECT_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == EPSIZESELECT_A::_64_BYTE
    }
    #[doc = "Checks if the value of the field is `_128_BYTE`"]
    #[inline(always)]
    pub fn is_128_byte(&self) -> bool {
        *self == EPSIZESELECT_A::_128_BYTE
    }
    #[doc = "Checks if the value of the field is `_256_BYTE`"]
    #[inline(always)]
    pub fn is_256_byte(&self) -> bool {
        *self == EPSIZESELECT_A::_256_BYTE
    }
    #[doc = "Checks if the value of the field is `_512_BYTE`"]
    #[inline(always)]
    pub fn is_512_byte(&self) -> bool {
        *self == EPSIZESELECT_A::_512_BYTE
    }
    #[doc = "Checks if the value of the field is `_1024_BYTE`"]
    #[inline(always)]
    pub fn is_1024_byte(&self) -> bool {
        *self == EPSIZESELECT_A::_1024_BYTE
    }
}
#[doc = "Field `EPSIZE` writer - Endpoint Size"]
pub type EPSIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, DEVEPTCFG_SPEC, 3, O, EPSIZESELECT_A>;
impl<'a, const O: u8> EPSIZE_W<'a, O> {
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_8_BYTE)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_16_BYTE)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_32_BYTE)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_64_BYTE)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128_byte(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_128_BYTE)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256_byte(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_256_BYTE)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512_byte(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_512_BYTE)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024_byte(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_1024_BYTE)
    }
}
#[doc = "Field `EPDIR` reader - Endpoint Direction"]
pub type EPDIR_R = crate::BitReader<EPDIRSELECT_A>;
#[doc = "Endpoint Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPDIRSELECT_A {
    #[doc = "0: The endpoint direction is OUT."]
    OUT = 0,
    #[doc = "1: The endpoint direction is IN (nor for control endpoints)."]
    IN = 1,
}
impl From<EPDIRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: EPDIRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl EPDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPDIRSELECT_A {
        match self.bits {
            false => EPDIRSELECT_A::OUT,
            true => EPDIRSELECT_A::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == EPDIRSELECT_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == EPDIRSELECT_A::IN
    }
}
#[doc = "Field `EPDIR` writer - Endpoint Direction"]
pub type EPDIR_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTCFG_SPEC, O, EPDIRSELECT_A>;
impl<'a, const O: u8> EPDIR_W<'a, O> {
    #[doc = "The endpoint direction is OUT."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(EPDIRSELECT_A::OUT)
    }
    #[doc = "The endpoint direction is IN (nor for control endpoints)."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(EPDIRSELECT_A::IN)
    }
}
#[doc = "Field `AUTOSW` reader - Automatic Switch"]
pub type AUTOSW_R = crate::BitReader;
#[doc = "Field `AUTOSW` writer - Automatic Switch"]
pub type AUTOSW_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTCFG_SPEC, O>;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader<EPTYPESELECT_A>;
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPTYPESELECT_A {
    #[doc = "0: Control"]
    CTRL = 0,
    #[doc = "1: Isochronous"]
    ISO = 1,
    #[doc = "2: Bulk"]
    BLK = 2,
    #[doc = "3: Interrupt"]
    INTRPT = 3,
}
impl From<EPTYPESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPTYPESELECT_A {
    type Ux = u8;
}
impl EPTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPTYPESELECT_A {
        match self.bits {
            0 => EPTYPESELECT_A::CTRL,
            1 => EPTYPESELECT_A::ISO,
            2 => EPTYPESELECT_A::BLK,
            3 => EPTYPESELECT_A::INTRPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == EPTYPESELECT_A::CTRL
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == EPTYPESELECT_A::ISO
    }
    #[doc = "Checks if the value of the field is `BLK`"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        *self == EPTYPESELECT_A::BLK
    }
    #[doc = "Checks if the value of the field is `INTRPT`"]
    #[inline(always)]
    pub fn is_intrpt(&self) -> bool {
        *self == EPTYPESELECT_A::INTRPT
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EPTYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, DEVEPTCFG_SPEC, 2, O, EPTYPESELECT_A>;
impl<'a, const O: u8> EPTYPE_W<'a, O> {
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(EPTYPESELECT_A::CTRL)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut W {
        self.variant(EPTYPESELECT_A::ISO)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut W {
        self.variant(EPTYPESELECT_A::BLK)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn intrpt(self) -> &'a mut W {
        self.variant(EPTYPESELECT_A::INTRPT)
    }
}
#[doc = "Field `NBTRANS` reader - Number of transactions per microframe for isochronous endpoint"]
pub type NBTRANS_R = crate::FieldReader<NBTRANSSELECT_A>;
#[doc = "Number of transactions per microframe for isochronous endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NBTRANSSELECT_A {
    #[doc = "0: Reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    _0_TRANS = 0,
    #[doc = "1: Default value: one transaction per microframe."]
    _1_TRANS = 1,
    #[doc = "2: Two transactions per microframe. This endpoint should be configured as double-bank."]
    _2_TRANS = 2,
    #[doc = "3: Three transactions per microframe. This endpoint should be configured as triple-bank."]
    _3_TRANS = 3,
}
impl From<NBTRANSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NBTRANSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NBTRANSSELECT_A {
    type Ux = u8;
}
impl NBTRANS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NBTRANSSELECT_A {
        match self.bits {
            0 => NBTRANSSELECT_A::_0_TRANS,
            1 => NBTRANSSELECT_A::_1_TRANS,
            2 => NBTRANSSELECT_A::_2_TRANS,
            3 => NBTRANSSELECT_A::_3_TRANS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_TRANS`"]
    #[inline(always)]
    pub fn is_0_trans(&self) -> bool {
        *self == NBTRANSSELECT_A::_0_TRANS
    }
    #[doc = "Checks if the value of the field is `_1_TRANS`"]
    #[inline(always)]
    pub fn is_1_trans(&self) -> bool {
        *self == NBTRANSSELECT_A::_1_TRANS
    }
    #[doc = "Checks if the value of the field is `_2_TRANS`"]
    #[inline(always)]
    pub fn is_2_trans(&self) -> bool {
        *self == NBTRANSSELECT_A::_2_TRANS
    }
    #[doc = "Checks if the value of the field is `_3_TRANS`"]
    #[inline(always)]
    pub fn is_3_trans(&self) -> bool {
        *self == NBTRANSSELECT_A::_3_TRANS
    }
}
#[doc = "Field `NBTRANS` writer - Number of transactions per microframe for isochronous endpoint"]
pub type NBTRANS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, DEVEPTCFG_SPEC, 2, O, NBTRANSSELECT_A>;
impl<'a, const O: u8> NBTRANS_W<'a, O> {
    #[doc = "Reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    #[inline(always)]
    pub fn _0_trans(self) -> &'a mut W {
        self.variant(NBTRANSSELECT_A::_0_TRANS)
    }
    #[doc = "Default value: one transaction per microframe."]
    #[inline(always)]
    pub fn _1_trans(self) -> &'a mut W {
        self.variant(NBTRANSSELECT_A::_1_TRANS)
    }
    #[doc = "Two transactions per microframe. This endpoint should be configured as double-bank."]
    #[inline(always)]
    pub fn _2_trans(self) -> &'a mut W {
        self.variant(NBTRANSSELECT_A::_2_TRANS)
    }
    #[doc = "Three transactions per microframe. This endpoint should be configured as triple-bank."]
    #[inline(always)]
    pub fn _3_trans(self) -> &'a mut W {
        self.variant(NBTRANSSELECT_A::_3_TRANS)
    }
}
impl R {
    #[doc = "Bit 1 - Endpoint Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&self) -> ALLOC_R {
        ALLOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Endpoint Banks"]
    #[inline(always)]
    pub fn epbk(&self) -> EPBK_R {
        EPBK_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    pub fn epsize(&self) -> EPSIZE_R {
        EPSIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&self) -> AUTOSW_R {
        AUTOSW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Number of transactions per microframe for isochronous endpoint"]
    #[inline(always)]
    pub fn nbtrans(&self) -> NBTRANS_R {
        NBTRANS_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Endpoint Memory Allocate"]
    #[inline(always)]
    #[must_use]
    pub fn alloc(&mut self) -> ALLOC_W<1> {
        ALLOC_W::new(self)
    }
    #[doc = "Bits 2:3 - Endpoint Banks"]
    #[inline(always)]
    #[must_use]
    pub fn epbk(&mut self) -> EPBK_W<2> {
        EPBK_W::new(self)
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    #[must_use]
    pub fn epsize(&mut self) -> EPSIZE_W<4> {
        EPSIZE_W::new(self)
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<8> {
        EPDIR_W::new(self)
    }
    #[doc = "Bit 9 - Automatic Switch"]
    #[inline(always)]
    #[must_use]
    pub fn autosw(&mut self) -> AUTOSW_W<9> {
        AUTOSW_W::new(self)
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<11> {
        EPTYPE_W::new(self)
    }
    #[doc = "Bits 13:14 - Number of transactions per microframe for isochronous endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn nbtrans(&mut self) -> NBTRANS_W<13> {
        NBTRANS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptcfg](index.html) module"]
pub struct DEVEPTCFG_SPEC;
impl crate::RegisterSpec for DEVEPTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deveptcfg::R](R) reader structure"]
impl crate::Readable for DEVEPTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [deveptcfg::W](W) writer structure"]
impl crate::Writable for DEVEPTCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVEPTCFG[%s]
to value 0"]
impl crate::Resettable for DEVEPTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
