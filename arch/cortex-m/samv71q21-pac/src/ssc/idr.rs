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
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Disable"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TXEMPTY` writer - Transmit Empty Interrupt Disable"]
pub type TXEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `RXRDY` writer - Receive Ready Interrupt Disable"]
pub type RXRDY_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `OVRUN` writer - Receive Overrun Interrupt Disable"]
pub type OVRUN_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `CP0` writer - Compare 0 Interrupt Disable"]
pub type CP0_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `CP1` writer - Compare 1 Interrupt Disable"]
pub type CP1_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TXSYN` writer - Tx Sync Interrupt Enable"]
pub type TXSYN_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `RXSYN` writer - Rx Sync Interrupt Enable"]
pub type RXSYN_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Transmit Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<0> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<1> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 4 - Receive Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<4> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 5 - Receive Overrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrun(&mut self) -> OVRUN_W<5> {
        OVRUN_W::new(self)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cp0(&mut self) -> CP0_W<8> {
        CP0_W::new(self)
    }
    #[doc = "Bit 9 - Compare 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cp1(&mut self) -> CP1_W<9> {
        CP1_W::new(self)
    }
    #[doc = "Bit 10 - Tx Sync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txsyn(&mut self) -> TXSYN_W<10> {
        TXSYN_W::new(self)
    }
    #[doc = "Bit 11 - Rx Sync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxsyn(&mut self) -> RXSYN_W<11> {
        RXSYN_W::new(self)
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
