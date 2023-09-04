#[doc = "Register `US_IDR_USART_MODE` writer"]
pub type W = crate::W<US_IDR_USART_MODE_SPEC>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
pub type RXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
pub type TXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBRK` writer - Receiver Break Interrupt Disable"]
pub type RXBRK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRAME` writer - Framing Error Interrupt Disable"]
pub type FRAME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PARE` writer - Parity Error Interrupt Disable"]
pub type PARE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMEOUT` writer - Timeout Interrupt Disable"]
pub type TIMEOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub type TXEMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ITER` writer - Max Number of Repetitions Reached Interrupt Disable"]
pub type ITER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NACK` writer - Non Acknowledge Interrupt Disable"]
pub type NACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RIIC` writer - Ring Indicator Input Change Disable"]
pub type RIIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSRIC` writer - Data Set Ready Input Change Disable"]
pub type DSRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCDIC` writer - Data Carrier Detect Input Change Interrupt Disable"]
pub type DCDIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTSIC` writer - Clear to Send Input Change Interrupt Disable"]
pub type CTSIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MANE` writer - Manchester Error Interrupt Disable"]
pub type MANE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<US_IDR_USART_MODE_SPEC, 0> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<US_IDR_USART_MODE_SPEC, 1> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrk(&mut self) -> RXBRK_W<US_IDR_USART_MODE_SPEC, 2> {
        RXBRK_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<US_IDR_USART_MODE_SPEC, 5> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FRAME_W<US_IDR_USART_MODE_SPEC, 6> {
        FRAME_W::new(self)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PARE_W<US_IDR_USART_MODE_SPEC, 7> {
        PARE_W::new(self)
    }
    #[doc = "Bit 8 - Timeout Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<US_IDR_USART_MODE_SPEC, 8> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<US_IDR_USART_MODE_SPEC, 9> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 10 - Max Number of Repetitions Reached Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn iter(&mut self) -> ITER_W<US_IDR_USART_MODE_SPEC, 10> {
        ITER_W::new(self)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<US_IDR_USART_MODE_SPEC, 13> {
        NACK_W::new(self)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Disable"]
    #[inline(always)]
    #[must_use]
    pub fn riic(&mut self) -> RIIC_W<US_IDR_USART_MODE_SPEC, 16> {
        RIIC_W::new(self)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dsric(&mut self) -> DSRIC_W<US_IDR_USART_MODE_SPEC, 17> {
        DSRIC_W::new(self)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdic(&mut self) -> DCDIC_W<US_IDR_USART_MODE_SPEC, 18> {
        DCDIC_W::new(self)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsic(&mut self) -> CTSIC_W<US_IDR_USART_MODE_SPEC, 19> {
        CTSIC_W::new(self)
    }
    #[doc = "Bit 24 - Manchester Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mane(&mut self) -> MANE_W<US_IDR_USART_MODE_SPEC, 24> {
        MANE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_idr_usart_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_IDR_USART_MODE_SPEC;
impl crate::RegisterSpec for US_IDR_USART_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_idr_usart_mode::W`](W) writer structure"]
impl crate::Writable for US_IDR_USART_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_IDR_USART_MODE to value 0"]
impl crate::Resettable for US_IDR_USART_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
