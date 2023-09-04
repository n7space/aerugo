#[doc = "Register `SSPUP` writer"]
pub type W = crate::W<SSPUP_SPEC>;
#[doc = "Field `SPRDUP` writer - Spread Spectrum Limit Value Update"]
pub type SPRDUP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl W {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value Update"]
    #[inline(always)]
    #[must_use]
    pub fn sprdup(&mut self) -> SPRDUP_W<SSPUP_SPEC, 0> {
        SPRDUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Spread Spectrum Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspup::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPUP_SPEC;
impl crate::RegisterSpec for SSPUP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sspup::W`](W) writer structure"]
impl crate::Writable for SSPUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSPUP to value 0"]
impl crate::Resettable for SSPUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
