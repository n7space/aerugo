#[doc = "Register `US_IER_SPI_MODE` writer"]
pub struct W(crate::W<US_IER_SPI_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_IER_SPI_MODE_SPEC>;
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
impl From<crate::W<US_IER_SPI_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_IER_SPI_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Enable"]
pub type RXRDY_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_SPI_MODE_SPEC, O>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Enable"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_SPI_MODE_SPEC, O>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_SPI_MODE_SPEC, O>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Enable"]
pub type TXEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_SPI_MODE_SPEC, O>;
#[doc = "Field `UNRE` writer - Underrun Error Interrupt Enable"]
pub type UNRE_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_SPI_MODE_SPEC, O>;
#[doc = "Field `NSSE` writer - NSS Line (Driving CTS Pin) Rising or Falling Edge Event"]
pub type NSSE_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_SPI_MODE_SPEC, O>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<0> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<1> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<5> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<9> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UNRE_W<10> {
        UNRE_W::new(self)
    }
    #[doc = "Bit 19 - NSS Line (Driving CTS Pin) Rising or Falling Edge Event"]
    #[inline(always)]
    #[must_use]
    pub fn nsse(&mut self) -> NSSE_W<19> {
        NSSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_ier_spi_mode](index.html) module"]
pub struct US_IER_SPI_MODE_SPEC;
impl crate::RegisterSpec for US_IER_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [us_ier_spi_mode::W](W) writer structure"]
impl crate::Writable for US_IER_SPI_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_IER_SPI_MODE to value 0"]
impl crate::Resettable for US_IER_SPI_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
