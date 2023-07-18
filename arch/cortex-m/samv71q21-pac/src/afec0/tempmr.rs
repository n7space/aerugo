#[doc = "Register `TEMPMR` reader"]
pub struct R(crate::R<TEMPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMPMR` writer"]
pub struct W(crate::W<TEMPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMPMR_SPEC>;
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
impl From<crate::W<TEMPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCT` reader - Temperature Sensor RTC Trigger Mode"]
pub type RTCT_R = crate::BitReader;
#[doc = "Field `RTCT` writer - Temperature Sensor RTC Trigger Mode"]
pub type RTCT_W<'a, const O: u8> = crate::BitWriter<'a, TEMPMR_SPEC, O>;
#[doc = "Field `TEMPCMPMOD` reader - Temperature Comparison Mode"]
pub type TEMPCMPMOD_R = crate::FieldReader<TEMPCMPMODSELECT_A>;
#[doc = "Temperature Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TEMPCMPMODSELECT_A {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    LOW = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    IN = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    OUT = 3,
}
impl From<TEMPCMPMODSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TEMPCMPMODSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TEMPCMPMODSELECT_A {
    type Ux = u8;
}
impl TEMPCMPMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMPCMPMODSELECT_A {
        match self.bits {
            0 => TEMPCMPMODSELECT_A::LOW,
            1 => TEMPCMPMODSELECT_A::HIGH,
            2 => TEMPCMPMODSELECT_A::IN,
            3 => TEMPCMPMODSELECT_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TEMPCMPMODSELECT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TEMPCMPMODSELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == TEMPCMPMODSELECT_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == TEMPCMPMODSELECT_A::OUT
    }
}
#[doc = "Field `TEMPCMPMOD` writer - Temperature Comparison Mode"]
pub type TEMPCMPMOD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, TEMPMR_SPEC, 2, O, TEMPCMPMODSELECT_A>;
impl<'a, const O: u8> TEMPCMPMOD_W<'a, O> {
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TEMPCMPMODSELECT_A::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TEMPCMPMODSELECT_A::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(TEMPCMPMODSELECT_A::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(TEMPCMPMODSELECT_A::OUT)
    }
}
impl R {
    #[doc = "Bit 0 - Temperature Sensor RTC Trigger Mode"]
    #[inline(always)]
    pub fn rtct(&self) -> RTCT_R {
        RTCT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Temperature Comparison Mode"]
    #[inline(always)]
    pub fn tempcmpmod(&self) -> TEMPCMPMOD_R {
        TEMPCMPMOD_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature Sensor RTC Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rtct(&mut self) -> RTCT_W<0> {
        RTCT_W::new(self)
    }
    #[doc = "Bits 4:5 - Temperature Comparison Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tempcmpmod(&mut self) -> TEMPCMPMOD_W<4> {
        TEMPCMPMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Temperature Sensor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempmr](index.html) module"]
pub struct TEMPMR_SPEC;
impl crate::RegisterSpec for TEMPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tempmr::R](R) reader structure"]
impl crate::Readable for TEMPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tempmr::W](W) writer structure"]
impl crate::Writable for TEMPMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEMPMR to value 0"]
impl crate::Resettable for TEMPMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
