#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_MODE` reader - Read Mode"]
pub type READ_MODE_R = crate::BitReader;
#[doc = "Field `READ_MODE` writer - Read Mode"]
pub type READ_MODE_W<'a, const O: u8> = crate::BitWriter<'a, MODE_SPEC, O>;
#[doc = "Field `WRITE_MODE` reader - Write Mode"]
pub type WRITE_MODE_R = crate::BitReader;
#[doc = "Field `WRITE_MODE` writer - Write Mode"]
pub type WRITE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, MODE_SPEC, O>;
#[doc = "Field `EXNW_MODE` reader - NWAIT Mode"]
pub type EXNW_MODE_R = crate::FieldReader<EXNW_MODESELECT_A>;
#[doc = "NWAIT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXNW_MODESELECT_A {
    #[doc = "0: Disabled-The NWAIT input signal is ignored on the corresponding chip select."]
    DISABLED = 0,
    #[doc = "2: Frozen Mode-If asserted, the NWAIT signal freezes the current read or write cycle. After deassertion, the read/write cycle is resumed from the point where it was stopped."]
    FROZEN = 2,
    #[doc = "3: Ready Mode-The NWAIT signal indicates the availability of the external device at the end of the pulse of the controlling read or write signal, to complete the access. If high, the access normally completes. If low, the access is extended until NWAIT returns high."]
    READY = 3,
}
impl From<EXNW_MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EXNW_MODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXNW_MODESELECT_A {
    type Ux = u8;
}
impl EXNW_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXNW_MODESELECT_A> {
        match self.bits {
            0 => Some(EXNW_MODESELECT_A::DISABLED),
            2 => Some(EXNW_MODESELECT_A::FROZEN),
            3 => Some(EXNW_MODESELECT_A::READY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXNW_MODESELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == EXNW_MODESELECT_A::FROZEN
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == EXNW_MODESELECT_A::READY
    }
}
#[doc = "Field `EXNW_MODE` writer - NWAIT Mode"]
pub type EXNW_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, MODE_SPEC, 2, O, EXNW_MODESELECT_A>;
impl<'a, const O: u8> EXNW_MODE_W<'a, O> {
    #[doc = "Disabled-The NWAIT input signal is ignored on the corresponding chip select."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXNW_MODESELECT_A::DISABLED)
    }
    #[doc = "Frozen Mode-If asserted, the NWAIT signal freezes the current read or write cycle. After deassertion, the read/write cycle is resumed from the point where it was stopped."]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(EXNW_MODESELECT_A::FROZEN)
    }
    #[doc = "Ready Mode-The NWAIT signal indicates the availability of the external device at the end of the pulse of the controlling read or write signal, to complete the access. If high, the access normally completes. If low, the access is extended until NWAIT returns high."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(EXNW_MODESELECT_A::READY)
    }
}
#[doc = "Field `BAT` reader - Byte Access Type"]
pub type BAT_R = crate::BitReader<BATSELECT_A>;
#[doc = "Byte Access Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BATSELECT_A {
    #[doc = "0: Byte select access type:- Write operation is controlled using NCS, NWE, NBS0, NBS1.- Read operation is controlled using NCS, NRD, NBS0, NBS1."]
    BYTE_SELECT = 0,
    #[doc = "1: Byte write access type:- Write operation is controlled using NCS, NWR0, NWR1.- Read operation is controlled using NCS and NRD."]
    BYTE_WRITE = 1,
}
impl From<BATSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BATSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BATSELECT_A {
        match self.bits {
            false => BATSELECT_A::BYTE_SELECT,
            true => BATSELECT_A::BYTE_WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE_SELECT`"]
    #[inline(always)]
    pub fn is_byte_select(&self) -> bool {
        *self == BATSELECT_A::BYTE_SELECT
    }
    #[doc = "Checks if the value of the field is `BYTE_WRITE`"]
    #[inline(always)]
    pub fn is_byte_write(&self) -> bool {
        *self == BATSELECT_A::BYTE_WRITE
    }
}
#[doc = "Field `BAT` writer - Byte Access Type"]
pub type BAT_W<'a, const O: u8> = crate::BitWriter<'a, MODE_SPEC, O, BATSELECT_A>;
impl<'a, const O: u8> BAT_W<'a, O> {
    #[doc = "Byte select access type:- Write operation is controlled using NCS, NWE, NBS0, NBS1.- Read operation is controlled using NCS, NRD, NBS0, NBS1."]
    #[inline(always)]
    pub fn byte_select(self) -> &'a mut W {
        self.variant(BATSELECT_A::BYTE_SELECT)
    }
    #[doc = "Byte write access type:- Write operation is controlled using NCS, NWR0, NWR1.- Read operation is controlled using NCS and NRD."]
    #[inline(always)]
    pub fn byte_write(self) -> &'a mut W {
        self.variant(BATSELECT_A::BYTE_WRITE)
    }
}
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DBW_R = crate::BitReader<DBWSELECT_A>;
#[doc = "Data Bus Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBWSELECT_A {
    #[doc = "0: 8-bit Data Bus"]
    _8_BIT = 0,
    #[doc = "1: 16-bit Data Bus"]
    _16_BIT = 1,
}
impl From<DBWSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DBWSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DBW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBWSELECT_A {
        match self.bits {
            false => DBWSELECT_A::_8_BIT,
            true => DBWSELECT_A::_16_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == DBWSELECT_A::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == DBWSELECT_A::_16_BIT
    }
}
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DBW_W<'a, const O: u8> = crate::BitWriter<'a, MODE_SPEC, O, DBWSELECT_A>;
impl<'a, const O: u8> DBW_W<'a, O> {
    #[doc = "8-bit Data Bus"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(DBWSELECT_A::_8_BIT)
    }
    #[doc = "16-bit Data Bus"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(DBWSELECT_A::_16_BIT)
    }
}
#[doc = "Field `TDF_CYCLES` reader - Data Float Time"]
pub type TDF_CYCLES_R = crate::FieldReader;
#[doc = "Field `TDF_CYCLES` writer - Data Float Time"]
pub type TDF_CYCLES_W<'a, const O: u8> = crate::FieldWriter<'a, MODE_SPEC, 4, O>;
#[doc = "Field `TDF_MODE` reader - TDF Optimization"]
pub type TDF_MODE_R = crate::BitReader;
#[doc = "Field `TDF_MODE` writer - TDF Optimization"]
pub type TDF_MODE_W<'a, const O: u8> = crate::BitWriter<'a, MODE_SPEC, O>;
#[doc = "Field `PMEN` reader - Page Mode Enabled"]
pub type PMEN_R = crate::BitReader;
#[doc = "Field `PMEN` writer - Page Mode Enabled"]
pub type PMEN_W<'a, const O: u8> = crate::BitWriter<'a, MODE_SPEC, O>;
#[doc = "Field `PS` reader - Page Size"]
pub type PS_R = crate::FieldReader<PSSELECT_A>;
#[doc = "Page Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSSELECT_A {
    #[doc = "0: 4-byte page"]
    _4_BYTE = 0,
    #[doc = "1: 8-byte page"]
    _8_BYTE = 1,
    #[doc = "2: 16-byte page"]
    _16_BYTE = 2,
    #[doc = "3: 32-byte page"]
    _32_BYTE = 3,
}
impl From<PSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSSELECT_A {
    type Ux = u8;
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSSELECT_A {
        match self.bits {
            0 => PSSELECT_A::_4_BYTE,
            1 => PSSELECT_A::_8_BYTE,
            2 => PSSELECT_A::_16_BYTE,
            3 => PSSELECT_A::_32_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4_BYTE`"]
    #[inline(always)]
    pub fn is_4_byte(&self) -> bool {
        *self == PSSELECT_A::_4_BYTE
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == PSSELECT_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == PSSELECT_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == PSSELECT_A::_32_BYTE
    }
}
#[doc = "Field `PS` writer - Page Size"]
pub type PS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, MODE_SPEC, 2, O, PSSELECT_A>;
impl<'a, const O: u8> PS_W<'a, O> {
    #[doc = "4-byte page"]
    #[inline(always)]
    pub fn _4_byte(self) -> &'a mut W {
        self.variant(PSSELECT_A::_4_BYTE)
    }
    #[doc = "8-byte page"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(PSSELECT_A::_8_BYTE)
    }
    #[doc = "16-byte page"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(PSSELECT_A::_16_BYTE)
    }
    #[doc = "32-byte page"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(PSSELECT_A::_32_BYTE)
    }
}
impl R {
    #[doc = "Bit 0 - Read Mode"]
    #[inline(always)]
    pub fn read_mode(&self) -> READ_MODE_R {
        READ_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Mode"]
    #[inline(always)]
    pub fn write_mode(&self) -> WRITE_MODE_R {
        WRITE_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> EXNW_MODE_R {
        EXNW_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&self) -> BAT_R {
        BAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&self) -> TDF_CYCLES_R {
        TDF_CYCLES_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&self) -> TDF_MODE_R {
        TDF_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    pub fn pmen(&self) -> PMEN_R {
        PMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn read_mode(&mut self) -> READ_MODE_W<0> {
        READ_MODE_W::new(self)
    }
    #[doc = "Bit 1 - Write Mode"]
    #[inline(always)]
    #[must_use]
    pub fn write_mode(&mut self) -> WRITE_MODE_W<1> {
        WRITE_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn exnw_mode(&mut self) -> EXNW_MODE_W<4> {
        EXNW_MODE_W::new(self)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    #[must_use]
    pub fn bat(&mut self) -> BAT_W<8> {
        BAT_W::new(self)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn dbw(&mut self) -> DBW_W<12> {
        DBW_W::new(self)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    #[must_use]
    pub fn tdf_cycles(&mut self) -> TDF_CYCLES_W<16> {
        TDF_CYCLES_W::new(self)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    #[must_use]
    pub fn tdf_mode(&mut self) -> TDF_MODE_W<20> {
        TDF_MODE_W::new(self)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn pmen(&mut self) -> PMEN_W<24> {
        PMEN_W::new(self)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<28> {
        PS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
