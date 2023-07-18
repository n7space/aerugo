#[doc = "Register `CMPV` reader"]
pub struct R(crate::R<CMPV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPV` writer"]
pub struct W(crate::W<CMPV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPV_SPEC>;
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
impl From<crate::W<CMPV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CV` reader - Comparison x Value"]
pub type CV_R = crate::FieldReader<u32>;
#[doc = "Field `CV` writer - Comparison x Value"]
pub type CV_W<'a, const O: u8> = crate::FieldWriter<'a, CMPV_SPEC, 24, O, u32>;
#[doc = "Field `CVM` reader - Comparison x Value Mode"]
pub type CVM_R = crate::BitReader<CVMSELECT_A>;
#[doc = "Comparison x Value Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CVMSELECT_A {
    #[doc = "0: Compare when counter is incrementing"]
    COMPARE_AT_INCREMENT = 0,
    #[doc = "1: Compare when counter is decrementing"]
    COMPARE_AT_DECREMENT = 1,
}
impl From<CVMSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CVMSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CVM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CVMSELECT_A {
        match self.bits {
            false => CVMSELECT_A::COMPARE_AT_INCREMENT,
            true => CVMSELECT_A::COMPARE_AT_DECREMENT,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARE_AT_INCREMENT`"]
    #[inline(always)]
    pub fn is_compare_at_increment(&self) -> bool {
        *self == CVMSELECT_A::COMPARE_AT_INCREMENT
    }
    #[doc = "Checks if the value of the field is `COMPARE_AT_DECREMENT`"]
    #[inline(always)]
    pub fn is_compare_at_decrement(&self) -> bool {
        *self == CVMSELECT_A::COMPARE_AT_DECREMENT
    }
}
#[doc = "Field `CVM` writer - Comparison x Value Mode"]
pub type CVM_W<'a, const O: u8> = crate::BitWriter<'a, CMPV_SPEC, O, CVMSELECT_A>;
impl<'a, const O: u8> CVM_W<'a, O> {
    #[doc = "Compare when counter is incrementing"]
    #[inline(always)]
    pub fn compare_at_increment(self) -> &'a mut W {
        self.variant(CVMSELECT_A::COMPARE_AT_INCREMENT)
    }
    #[doc = "Compare when counter is decrementing"]
    #[inline(always)]
    pub fn compare_at_decrement(self) -> &'a mut W {
        self.variant(CVMSELECT_A::COMPARE_AT_DECREMENT)
    }
}
impl R {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&self) -> CVM_R {
        CVM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    #[must_use]
    pub fn cv(&mut self) -> CV_W<0> {
        CV_W::new(self)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cvm(&mut self) -> CVM_W<24> {
        CVM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 0 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv](index.html) module"]
pub struct CMPV_SPEC;
impl crate::RegisterSpec for CMPV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpv::R](R) reader structure"]
impl crate::Readable for CMPV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpv::W](W) writer structure"]
impl crate::Writable for CMPV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPV to value 0"]
impl crate::Resettable for CMPV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
