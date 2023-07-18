#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
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
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDY` writer - Receiver Ready Interrupt Disable"]
pub type RXRDY_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `RXOR` writer - Receiver Overrun Interrupt Disable"]
pub type RXOR_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Disable"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TXUR` writer - Transmit Underflow Interrupt Disable"]
pub type TXUR_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
impl W {
    #[doc = "Bit 1 - Receiver Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<1> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Receiver Overrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxor(&mut self) -> RXOR_W<2> {
        RXOR_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<5> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Underflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txur(&mut self) -> TXUR_W<6> {
        TXUR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
