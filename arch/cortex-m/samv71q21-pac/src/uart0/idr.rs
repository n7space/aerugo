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
#[doc = "Field `RXRDY` writer - Disable RXRDY Interrupt"]
pub type RXRDY_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TXRDY` writer - Disable TXRDY Interrupt"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `OVRE` writer - Disable Overrun Error Interrupt"]
pub type OVRE_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `FRAME` writer - Disable Framing Error Interrupt"]
pub type FRAME_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `PARE` writer - Disable Parity Error Interrupt"]
pub type PARE_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TXEMPTY` writer - Disable TXEMPTY Interrupt"]
pub type TXEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `CMP` writer - Disable Comparison Interrupt"]
pub type CMP_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Disable RXRDY Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<0> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - Disable TXRDY Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<1> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 5 - Disable Overrun Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<5> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 6 - Disable Framing Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FRAME_W<6> {
        FRAME_W::new(self)
    }
    #[doc = "Bit 7 - Disable Parity Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PARE_W<7> {
        PARE_W::new(self)
    }
    #[doc = "Bit 9 - Disable TXEMPTY Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<9> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 15 - Disable Comparison Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<15> {
        CMP_W::new(self)
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
