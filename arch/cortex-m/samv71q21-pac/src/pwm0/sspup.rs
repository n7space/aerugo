#[doc = "Register `SSPUP` writer"]
pub struct W(crate::W<SSPUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPUP_SPEC>;
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
impl From<crate::W<SSPUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPRDUP` writer - Spread Spectrum Limit Value Update"]
pub type SPRDUP_W<'a, const O: u8> = crate::FieldWriter<'a, SSPUP_SPEC, 24, O, u32>;
impl W {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value Update"]
    #[inline(always)]
    #[must_use]
    pub fn sprdup(&mut self) -> SPRDUP_W<0> {
        SPRDUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Spread Spectrum Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspup](index.html) module"]
pub struct SSPUP_SPEC;
impl crate::RegisterSpec for SSPUP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sspup::W](W) writer structure"]
impl crate::Writable for SSPUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSPUP to value 0"]
impl crate::Resettable for SSPUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
