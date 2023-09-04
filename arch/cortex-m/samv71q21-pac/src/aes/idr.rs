#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `DATRDY` writer - Data Ready Interrupt Disable"]
pub type DATRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `URAD` writer - Unspecified Register Access Detection Interrupt Disable"]
pub type URAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAGRDY` writer - GCM Tag Ready Interrupt Disable"]
pub type TAGRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Data Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn datrdy(&mut self) -> DATRDY_W<IDR_SPEC, 0> {
        DATRDY_W::new(self)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn urad(&mut self) -> URAD_W<IDR_SPEC, 8> {
        URAD_W::new(self)
    }
    #[doc = "Bit 16 - GCM Tag Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tagrdy(&mut self) -> TAGRDY_W<IDR_SPEC, 16> {
        TAGRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
