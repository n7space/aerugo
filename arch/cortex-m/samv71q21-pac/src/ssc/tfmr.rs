#[doc = "Register `TFMR` reader"]
pub struct R(crate::R<TFMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFMR` writer"]
pub struct W(crate::W<TFMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFMR_SPEC>;
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
impl From<crate::W<TFMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATLEN` reader - Data Length"]
pub type DATLEN_R = crate::FieldReader;
#[doc = "Field `DATLEN` writer - Data Length"]
pub type DATLEN_W<'a, const O: u8> = crate::FieldWriter<'a, TFMR_SPEC, 5, O>;
#[doc = "Field `DATDEF` reader - Data Default Value"]
pub type DATDEF_R = crate::BitReader;
#[doc = "Field `DATDEF` writer - Data Default Value"]
pub type DATDEF_W<'a, const O: u8> = crate::BitWriter<'a, TFMR_SPEC, O>;
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MSBF_R = crate::BitReader;
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MSBF_W<'a, const O: u8> = crate::BitWriter<'a, TFMR_SPEC, O>;
#[doc = "Field `DATNB` reader - Data Number per Frame"]
pub type DATNB_R = crate::FieldReader;
#[doc = "Field `DATNB` writer - Data Number per Frame"]
pub type DATNB_W<'a, const O: u8> = crate::FieldWriter<'a, TFMR_SPEC, 4, O>;
#[doc = "Field `FSLEN` reader - Transmit Frame Sync Length"]
pub type FSLEN_R = crate::FieldReader;
#[doc = "Field `FSLEN` writer - Transmit Frame Sync Length"]
pub type FSLEN_W<'a, const O: u8> = crate::FieldWriter<'a, TFMR_SPEC, 4, O>;
#[doc = "Field `FSOS` reader - Transmit Frame Sync Output Selection"]
pub type FSOS_R = crate::FieldReader<FSOSSELECT_A>;
#[doc = "Transmit Frame Sync Output Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSOSSELECT_A {
    #[doc = "0: None, TF pin is an input"]
    NONE = 0,
    #[doc = "1: Negative Pulse, TF pin is an output"]
    NEGATIVE = 1,
    #[doc = "2: Positive Pulse, TF pin is an output"]
    POSITIVE = 2,
    #[doc = "3: Driven Low during data transfer"]
    LOW = 3,
    #[doc = "4: Driven High during data transfer"]
    HIGH = 4,
    #[doc = "5: Toggling at each start of data transfer"]
    TOGGLING = 5,
}
impl From<FSOSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FSOSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSOSSELECT_A {
    type Ux = u8;
}
impl FSOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FSOSSELECT_A> {
        match self.bits {
            0 => Some(FSOSSELECT_A::NONE),
            1 => Some(FSOSSELECT_A::NEGATIVE),
            2 => Some(FSOSSELECT_A::POSITIVE),
            3 => Some(FSOSSELECT_A::LOW),
            4 => Some(FSOSSELECT_A::HIGH),
            5 => Some(FSOSSELECT_A::TOGGLING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == FSOSSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == FSOSSELECT_A::NEGATIVE
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == FSOSSELECT_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == FSOSSELECT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == FSOSSELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLING`"]
    #[inline(always)]
    pub fn is_toggling(&self) -> bool {
        *self == FSOSSELECT_A::TOGGLING
    }
}
#[doc = "Field `FSOS` writer - Transmit Frame Sync Output Selection"]
pub type FSOS_W<'a, const O: u8> = crate::FieldWriter<'a, TFMR_SPEC, 3, O, FSOSSELECT_A>;
impl<'a, const O: u8> FSOS_W<'a, O> {
    #[doc = "None, TF pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(FSOSSELECT_A::NONE)
    }
    #[doc = "Negative Pulse, TF pin is an output"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(FSOSSELECT_A::NEGATIVE)
    }
    #[doc = "Positive Pulse, TF pin is an output"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(FSOSSELECT_A::POSITIVE)
    }
    #[doc = "Driven Low during data transfer"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(FSOSSELECT_A::LOW)
    }
    #[doc = "Driven High during data transfer"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(FSOSSELECT_A::HIGH)
    }
    #[doc = "Toggling at each start of data transfer"]
    #[inline(always)]
    pub fn toggling(self) -> &'a mut W {
        self.variant(FSOSSELECT_A::TOGGLING)
    }
}
#[doc = "Field `FSDEN` reader - Frame Sync Data Enable"]
pub type FSDEN_R = crate::BitReader;
#[doc = "Field `FSDEN` writer - Frame Sync Data Enable"]
pub type FSDEN_W<'a, const O: u8> = crate::BitWriter<'a, TFMR_SPEC, O>;
#[doc = "Field `FSEDGE` reader - Frame Sync Edge Detection"]
pub type FSEDGE_R = crate::BitReader<FSEDGESELECT_A>;
#[doc = "Frame Sync Edge Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSEDGESELECT_A {
    #[doc = "0: Positive Edge Detection"]
    POSITIVE = 0,
    #[doc = "1: Negative Edge Detection"]
    NEGATIVE = 1,
}
impl From<FSEDGESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FSEDGESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FSEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEDGESELECT_A {
        match self.bits {
            false => FSEDGESELECT_A::POSITIVE,
            true => FSEDGESELECT_A::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == FSEDGESELECT_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == FSEDGESELECT_A::NEGATIVE
    }
}
#[doc = "Field `FSEDGE` writer - Frame Sync Edge Detection"]
pub type FSEDGE_W<'a, const O: u8> = crate::BitWriter<'a, TFMR_SPEC, O, FSEDGESELECT_A>;
impl<'a, const O: u8> FSEDGE_W<'a, O> {
    #[doc = "Positive Edge Detection"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(FSEDGESELECT_A::POSITIVE)
    }
    #[doc = "Negative Edge Detection"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(FSEDGESELECT_A::NEGATIVE)
    }
}
#[doc = "Field `FSLEN_EXT` reader - FSLEN Field Extension"]
pub type FSLEN_EXT_R = crate::FieldReader;
#[doc = "Field `FSLEN_EXT` writer - FSLEN Field Extension"]
pub type FSLEN_EXT_W<'a, const O: u8> = crate::FieldWriter<'a, TFMR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Data Default Value"]
    #[inline(always)]
    pub fn datdef(&self) -> DATDEF_R {
        DATDEF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    pub fn datnb(&self) -> DATNB_R {
        DATNB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Transmit Frame Sync Length"]
    #[inline(always)]
    pub fn fslen(&self) -> FSLEN_R {
        FSLEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Frame Sync Output Selection"]
    #[inline(always)]
    pub fn fsos(&self) -> FSOS_R {
        FSOS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Frame Sync Data Enable"]
    #[inline(always)]
    pub fn fsden(&self) -> FSDEN_R {
        FSDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    pub fn fsedge(&self) -> FSEDGE_R {
        FSEDGE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    pub fn fslen_ext(&self) -> FSLEN_EXT_R {
        FSLEN_EXT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<0> {
        DATLEN_W::new(self)
    }
    #[doc = "Bit 5 - Data Default Value"]
    #[inline(always)]
    #[must_use]
    pub fn datdef(&mut self) -> DATDEF_W<5> {
        DATDEF_W::new(self)
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<7> {
        MSBF_W::new(self)
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline(always)]
    #[must_use]
    pub fn datnb(&mut self) -> DATNB_W<8> {
        DATNB_W::new(self)
    }
    #[doc = "Bits 16:19 - Transmit Frame Sync Length"]
    #[inline(always)]
    #[must_use]
    pub fn fslen(&mut self) -> FSLEN_W<16> {
        FSLEN_W::new(self)
    }
    #[doc = "Bits 20:22 - Transmit Frame Sync Output Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fsos(&mut self) -> FSOS_W<20> {
        FSOS_W::new(self)
    }
    #[doc = "Bit 23 - Frame Sync Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fsden(&mut self) -> FSDEN_W<23> {
        FSDEN_W::new(self)
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline(always)]
    #[must_use]
    pub fn fsedge(&mut self) -> FSEDGE_W<24> {
        FSEDGE_W::new(self)
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline(always)]
    #[must_use]
    pub fn fslen_ext(&mut self) -> FSLEN_EXT_W<28> {
        FSLEN_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Frame Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfmr](index.html) module"]
pub struct TFMR_SPEC;
impl crate::RegisterSpec for TFMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfmr::R](R) reader structure"]
impl crate::Readable for TFMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tfmr::W](W) writer structure"]
impl crate::Writable for TFMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFMR to value 0"]
impl crate::Resettable for TFMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
