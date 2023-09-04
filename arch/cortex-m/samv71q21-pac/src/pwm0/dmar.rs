#[doc = "Register `DMAR` writer"]
pub type W = crate::W<DMAR_SPEC>;
#[doc = "Field `DMADUTY` writer - Duty-Cycle Holding Register for DMA Access"]
pub type DMADUTY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl W {
    #[doc = "Bits 0:23 - Duty-Cycle Holding Register for DMA Access"]
    #[inline(always)]
    #[must_use]
    pub fn dmaduty(&mut self) -> DMADUTY_W<DMAR_SPEC, 0> {
        DMADUTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM DMA Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmar::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAR_SPEC;
impl crate::RegisterSpec for DMAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmar::W`](W) writer structure"]
impl crate::Writable for DMAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAR to value 0"]
impl crate::Resettable for DMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
