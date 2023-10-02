#[doc = "Register `SSR` writer"]
pub type W = crate::W<SSR_SPEC>;
#[doc = "Field `RXOR` writer - Receive Overrun Status Set"]
pub type RXOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUR` writer - Transmit Underrun Status Set"]
pub type TXUR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXORCH` writer - Receive Overrun Per Channel Status Set"]
pub type RXORCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TXURCH` writer - Transmit Underrun Per Channel Status Set"]
pub type TXURCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl W {
    #[doc = "Bit 2 - Receive Overrun Status Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxor(&mut self) -> RXOR_W<SSR_SPEC, 2> {
        RXOR_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Underrun Status Set"]
    #[inline(always)]
    #[must_use]
    pub fn txur(&mut self) -> TXUR_W<SSR_SPEC, 6> {
        TXUR_W::new(self)
    }
    #[doc = "Bits 8:9 - Receive Overrun Per Channel Status Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxorch(&mut self) -> RXORCH_W<SSR_SPEC, 8> {
        RXORCH_W::new(self)
    }
    #[doc = "Bits 20:21 - Transmit Underrun Per Channel Status Set"]
    #[inline(always)]
    #[must_use]
    pub fn txurch(&mut self) -> TXURCH_W<SSR_SPEC, 20> {
        TXURCH_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSR_SPEC;
impl crate::RegisterSpec for SSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ssr::W`](W) writer structure"]
impl crate::Writable for SSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSR to value 0"]
impl crate::Resettable for SSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
