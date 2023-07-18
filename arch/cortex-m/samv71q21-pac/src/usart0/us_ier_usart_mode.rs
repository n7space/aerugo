#[doc = "Register `US_IER_USART_MODE` writer"]
pub struct W(crate::W<US_IER_USART_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_IER_USART_MODE_SPEC>;
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
impl From<crate::W<US_IER_USART_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_IER_USART_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Enable"]
pub type RXRDY_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Enable"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `RXBRK` writer - Receiver Break Interrupt Enable"]
pub type RXBRK_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `FRAME` writer - Framing Error Interrupt Enable"]
pub type FRAME_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `PARE` writer - Parity Error Interrupt Enable"]
pub type PARE_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `TIMEOUT` writer - Timeout Interrupt Enable"]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Enable"]
pub type TXEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `ITER` writer - Max number of Repetitions Reached Interrupt Enable"]
pub type ITER_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `NACK` writer - Non Acknowledge Interrupt Enable"]
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `RIIC` writer - Ring Indicator Input Change Enable"]
pub type RIIC_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `DSRIC` writer - Data Set Ready Input Change Enable"]
pub type DSRIC_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `DCDIC` writer - Data Carrier Detect Input Change Interrupt Enable"]
pub type DCDIC_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `CTSIC` writer - Clear to Send Input Change Interrupt Enable"]
pub type CTSIC_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
#[doc = "Field `MANE` writer - Manchester Error Interrupt Enable"]
pub type MANE_W<'a, const O: u8> = crate::BitWriter<'a, US_IER_USART_MODE_SPEC, O>;
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
    #[doc = "Bit 2 - Receiver Break Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrk(&mut self) -> RXBRK_W<2> {
        RXBRK_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<5> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FRAME_W<6> {
        FRAME_W::new(self)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PARE_W<7> {
        PARE_W::new(self)
    }
    #[doc = "Bit 8 - Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<8> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<9> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 10 - Max number of Repetitions Reached Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iter(&mut self) -> ITER_W<10> {
        ITER_W::new(self)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<13> {
        NACK_W::new(self)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn riic(&mut self) -> RIIC_W<16> {
        RIIC_W::new(self)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsric(&mut self) -> DSRIC_W<17> {
        DSRIC_W::new(self)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdic(&mut self) -> DCDIC_W<18> {
        DCDIC_W::new(self)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsic(&mut self) -> CTSIC_W<19> {
        CTSIC_W::new(self)
    }
    #[doc = "Bit 24 - Manchester Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mane(&mut self) -> MANE_W<24> {
        MANE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_ier_usart_mode](index.html) module"]
pub struct US_IER_USART_MODE_SPEC;
impl crate::RegisterSpec for US_IER_USART_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [us_ier_usart_mode::W](W) writer structure"]
impl crate::Writable for US_IER_USART_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_IER_USART_MODE to value 0"]
impl crate::Resettable for US_IER_USART_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
