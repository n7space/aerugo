#[doc = "Register `FMR` reader"]
pub struct R(crate::R<FMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMR` writer"]
pub struct W(crate::W<FMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMR_SPEC>;
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
impl From<crate::W<FMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPOL` reader - Fault Polarity"]
pub type FPOL_R = crate::FieldReader;
#[doc = "Field `FPOL` writer - Fault Polarity"]
pub type FPOL_W<'a, const O: u8> = crate::FieldWriter<'a, FMR_SPEC, 8, O>;
#[doc = "Field `FMOD` reader - Fault Activation Mode"]
pub type FMOD_R = crate::FieldReader;
#[doc = "Field `FMOD` writer - Fault Activation Mode"]
pub type FMOD_W<'a, const O: u8> = crate::FieldWriter<'a, FMR_SPEC, 8, O>;
#[doc = "Field `FFIL` reader - Fault Filtering"]
pub type FFIL_R = crate::FieldReader;
#[doc = "Field `FFIL` writer - Fault Filtering"]
pub type FFIL_W<'a, const O: u8> = crate::FieldWriter<'a, FMR_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Fault Polarity"]
    #[inline(always)]
    pub fn fpol(&self) -> FPOL_R {
        FPOL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Activation Mode"]
    #[inline(always)]
    pub fn fmod(&self) -> FMOD_R {
        FMOD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Filtering"]
    #[inline(always)]
    pub fn ffil(&self) -> FFIL_R {
        FFIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn fpol(&mut self) -> FPOL_W<0> {
        FPOL_W::new(self)
    }
    #[doc = "Bits 8:15 - Fault Activation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmod(&mut self) -> FMOD_W<8> {
        FMOD_W::new(self)
    }
    #[doc = "Bits 16:23 - Fault Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn ffil(&mut self) -> FFIL_W<16> {
        FFIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmr](index.html) module"]
pub struct FMR_SPEC;
impl crate::RegisterSpec for FMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmr::R](R) reader structure"]
impl crate::Readable for FMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmr::W](W) writer structure"]
impl crate::Writable for FMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMR to value 0"]
impl crate::Resettable for FMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
