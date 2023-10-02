#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `RXRDY` writer - Receiver Ready Interrupt Disable"]
pub type RXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOR` writer - Receiver Overrun Interrupt Disable"]
pub type RXOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Disable"]
pub type TXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUR` writer - Transmit Underflow Interrupt Disable"]
pub type TXUR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 1 - Receiver Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<IDR_SPEC, 1> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Receiver Overrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxor(&mut self) -> RXOR_W<IDR_SPEC, 2> {
        RXOR_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<IDR_SPEC, 5> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Underflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txur(&mut self) -> TXUR_W<IDR_SPEC, 6> {
        TXUR_W::new(self)
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
