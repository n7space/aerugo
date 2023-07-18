#[doc = "Register `COSR` reader"]
pub struct R(crate::R<COSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COSR` writer"]
pub struct W(crate::W<COSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COSR_SPEC>;
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
impl From<crate::W<COSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSEL` reader - Sample &amp; Hold unit Correction Select"]
pub type CSEL_R = crate::BitReader;
#[doc = "Field `CSEL` writer - Sample &amp; Hold unit Correction Select"]
pub type CSEL_W<'a, const O: u8> = crate::BitWriter<'a, COSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Sample &amp; Hold unit Correction Select"]
    #[inline(always)]
    pub fn csel(&self) -> CSEL_R {
        CSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample &amp; Hold unit Correction Select"]
    #[inline(always)]
    #[must_use]
    pub fn csel(&mut self) -> CSEL_W<0> {
        CSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Correction Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cosr](index.html) module"]
pub struct COSR_SPEC;
impl crate::RegisterSpec for COSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cosr::R](R) reader structure"]
impl crate::Readable for COSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cosr::W](W) writer structure"]
impl crate::Writable for COSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COSR to value 0"]
impl crate::Resettable for COSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
