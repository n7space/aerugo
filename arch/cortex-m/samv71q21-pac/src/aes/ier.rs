#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `DATRDY` writer - Data Ready Interrupt Enable"]
pub type DATRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `URAD` writer - Unspecified Register Access Detection Interrupt Enable"]
pub type URAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAGRDY` writer - GCM Tag Ready Interrupt Enable"]
pub type TAGRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Data Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn datrdy(&mut self) -> DATRDY_W<IER_SPEC, 0> {
        DATRDY_W::new(self)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn urad(&mut self) -> URAD_W<IER_SPEC, 8> {
        URAD_W::new(self)
    }
    #[doc = "Bit 16 - GCM Tag Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tagrdy(&mut self) -> TAGRDY_W<IER_SPEC, 16> {
        TAGRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
