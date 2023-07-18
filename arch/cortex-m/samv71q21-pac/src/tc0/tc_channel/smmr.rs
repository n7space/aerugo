#[doc = "Register `SMMR` reader"]
pub struct R(crate::R<SMMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMMR` writer"]
pub struct W(crate::W<SMMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMMR_SPEC>;
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
impl From<crate::W<SMMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCEN` reader - Gray Count Enable"]
pub type GCEN_R = crate::BitReader;
#[doc = "Field `GCEN` writer - Gray Count Enable"]
pub type GCEN_W<'a, const O: u8> = crate::BitWriter<'a, SMMR_SPEC, O>;
#[doc = "Field `DOWN` reader - Down Count"]
pub type DOWN_R = crate::BitReader;
#[doc = "Field `DOWN` writer - Down Count"]
pub type DOWN_W<'a, const O: u8> = crate::BitWriter<'a, SMMR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Gray Count Enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Down Count"]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gray Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<0> {
        GCEN_W::new(self)
    }
    #[doc = "Bit 1 - Down Count"]
    #[inline(always)]
    #[must_use]
    pub fn down(&mut self) -> DOWN_W<1> {
        DOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stepper Motor Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smmr](index.html) module"]
pub struct SMMR_SPEC;
impl crate::RegisterSpec for SMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smmr::R](R) reader structure"]
impl crate::Readable for SMMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smmr::W](W) writer structure"]
impl crate::Writable for SMMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMMR to value 0"]
impl crate::Resettable for SMMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
