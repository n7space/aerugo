#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Enable"]
pub type TXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXEMPTY` writer - Transmit Empty Interrupt Enable"]
pub type TXEMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXRDY` writer - Receive Ready Interrupt Enable"]
pub type RXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVRUN` writer - Receive Overrun Interrupt Enable"]
pub type OVRUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP0` writer - Compare 0 Interrupt Enable"]
pub type CP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CP1` writer - Compare 1 Interrupt Enable"]
pub type CP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXSYN` writer - Tx Sync Interrupt Enable"]
pub type TXSYN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSYN` writer - Rx Sync Interrupt Enable"]
pub type RXSYN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Transmit Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<IER_SPEC, 0> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<IER_SPEC, 1> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 4 - Receive Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<IER_SPEC, 4> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 5 - Receive Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrun(&mut self) -> OVRUN_W<IER_SPEC, 5> {
        OVRUN_W::new(self)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cp0(&mut self) -> CP0_W<IER_SPEC, 8> {
        CP0_W::new(self)
    }
    #[doc = "Bit 9 - Compare 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cp1(&mut self) -> CP1_W<IER_SPEC, 9> {
        CP1_W::new(self)
    }
    #[doc = "Bit 10 - Tx Sync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txsyn(&mut self) -> TXSYN_W<IER_SPEC, 10> {
        TXSYN_W::new(self)
    }
    #[doc = "Bit 11 - Rx Sync Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxsyn(&mut self) -> RXSYN_W<IER_SPEC, 11> {
        RXSYN_W::new(self)
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
