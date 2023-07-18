#[doc = "Register `US_IMR_USART_MODE` reader"]
pub struct R(crate::R<US_IMR_USART_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_IMR_USART_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_IMR_USART_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_IMR_USART_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - RXRDY Interrupt Mask"]
pub type RXRDY_R = crate::BitReader;
#[doc = "Field `TXRDY` reader - TXRDY Interrupt Mask"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `RXBRK` reader - Receiver Break Interrupt Mask"]
pub type RXBRK_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub type OVRE_R = crate::BitReader;
#[doc = "Field `FRAME` reader - Framing Error Interrupt Mask"]
pub type FRAME_R = crate::BitReader;
#[doc = "Field `PARE` reader - Parity Error Interrupt Mask"]
pub type PARE_R = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - Timeout Interrupt Mask"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - TXEMPTY Interrupt Mask"]
pub type TXEMPTY_R = crate::BitReader;
#[doc = "Field `ITER` reader - Max Number of Repetitions Reached Interrupt Mask"]
pub type ITER_R = crate::BitReader;
#[doc = "Field `NACK` reader - Non Acknowledge Interrupt Mask"]
pub type NACK_R = crate::BitReader;
#[doc = "Field `RIIC` reader - Ring Indicator Input Change Mask"]
pub type RIIC_R = crate::BitReader;
#[doc = "Field `DSRIC` reader - Data Set Ready Input Change Mask"]
pub type DSRIC_R = crate::BitReader;
#[doc = "Field `DCDIC` reader - Data Carrier Detect Input Change Interrupt Mask"]
pub type DCDIC_R = crate::BitReader;
#[doc = "Field `CTSIC` reader - Clear to Send Input Change Interrupt Mask"]
pub type CTSIC_R = crate::BitReader;
#[doc = "Field `MANE` reader - Manchester Error Interrupt Mask"]
pub type MANE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Mask"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RXBRK_R {
        RXBRK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timeout Interrupt Mask"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Max Number of Repetitions Reached Interrupt Mask"]
    #[inline(always)]
    pub fn iter(&self) -> ITER_R {
        ITER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt Mask"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Mask"]
    #[inline(always)]
    pub fn riic(&self) -> RIIC_R {
        RIIC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Mask"]
    #[inline(always)]
    pub fn dsric(&self) -> DSRIC_R {
        DSRIC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Mask"]
    #[inline(always)]
    pub fn dcdic(&self) -> DCDIC_R {
        DCDIC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Mask"]
    #[inline(always)]
    pub fn ctsic(&self) -> CTSIC_R {
        CTSIC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Manchester Error Interrupt Mask"]
    #[inline(always)]
    pub fn mane(&self) -> MANE_R {
        MANE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_imr_usart_mode](index.html) module"]
pub struct US_IMR_USART_MODE_SPEC;
impl crate::RegisterSpec for US_IMR_USART_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_imr_usart_mode::R](R) reader structure"]
impl crate::Readable for US_IMR_USART_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_IMR_USART_MODE to value 0"]
impl crate::Resettable for US_IMR_USART_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
