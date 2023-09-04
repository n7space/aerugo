#[doc = "Register `US_CSR_USART_MODE` reader"]
pub type R = crate::R<US_CSR_USART_MODE_SPEC>;
#[doc = "Field `RXRDY` reader - Receiver Ready (cleared by reading US_RHR)"]
pub type RXRDY_R = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmitter Ready (cleared by writing US_THR)"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `RXBRK` reader - Break Received/End of Break (cleared by writing a one to bit US_CR.RSTSTA)"]
pub type RXBRK_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
pub type OVRE_R = crate::BitReader;
#[doc = "Field `FRAME` reader - Framing Error (cleared by writing a one to bit US_CR.RSTSTA)"]
pub type FRAME_R = crate::BitReader;
#[doc = "Field `PARE` reader - Parity Error (cleared by writing a one to bit US_CR.RSTSTA)"]
pub type PARE_R = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - Receiver Timeout (cleared by writing a one to bit US_CR.STTTO)"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty (cleared by writing US_THR)"]
pub type TXEMPTY_R = crate::BitReader;
#[doc = "Field `ITER` reader - Max Number of Repetitions Reached (cleared by writing a one to bit US_CR.RSTIT)"]
pub type ITER_R = crate::BitReader;
#[doc = "Field `NACK` reader - Non Acknowledge Interrupt (cleared by writing a one to bit US_CR.RSTNACK)"]
pub type NACK_R = crate::BitReader;
#[doc = "Field `RIIC` reader - Ring Indicator Input Change Flag (cleared on read)"]
pub type RIIC_R = crate::BitReader;
#[doc = "Field `DSRIC` reader - Data Set Ready Input Change Flag (cleared on read)"]
pub type DSRIC_R = crate::BitReader;
#[doc = "Field `DCDIC` reader - Data Carrier Detect Input Change Flag (cleared on read)"]
pub type DCDIC_R = crate::BitReader;
#[doc = "Field `CTSIC` reader - Clear to Send Input Change Flag (cleared on read)"]
pub type CTSIC_R = crate::BitReader;
#[doc = "Field `RI` reader - Image of RI Input"]
pub type RI_R = crate::BitReader;
#[doc = "Field `DSR` reader - Image of DSR Input"]
pub type DSR_R = crate::BitReader;
#[doc = "Field `DCD` reader - Image of DCD Input"]
pub type DCD_R = crate::BitReader;
#[doc = "Field `CTS` reader - Image of CTS Input"]
pub type CTS_R = crate::BitReader;
#[doc = "Field `MANERR` reader - Manchester Error (cleared by writing a one to the bit US_CR.RSTSTA)"]
pub type MANERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Ready (cleared by reading US_RHR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Break Received/End of Break (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RXBRK_R {
        RXBRK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receiver Timeout (cleared by writing a one to bit US_CR.STTTO)"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Max Number of Repetitions Reached (cleared by writing a one to bit US_CR.RSTIT)"]
    #[inline(always)]
    pub fn iter(&self) -> ITER_R {
        ITER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt (cleared by writing a one to bit US_CR.RSTNACK)"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Flag (cleared on read)"]
    #[inline(always)]
    pub fn riic(&self) -> RIIC_R {
        RIIC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Flag (cleared on read)"]
    #[inline(always)]
    pub fn dsric(&self) -> DSRIC_R {
        DSRIC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Flag (cleared on read)"]
    #[inline(always)]
    pub fn dcdic(&self) -> DCDIC_R {
        DCDIC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Flag (cleared on read)"]
    #[inline(always)]
    pub fn ctsic(&self) -> CTSIC_R {
        CTSIC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Image of RI Input"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Image of DSR Input"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Image of DCD Input"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Image of CTS Input"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Manchester Error (cleared by writing a one to the bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn manerr(&self) -> MANERR_R {
        MANERR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_csr_usart_mode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_CSR_USART_MODE_SPEC;
impl crate::RegisterSpec for US_CSR_USART_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_csr_usart_mode::R`](R) reader structure"]
impl crate::Readable for US_CSR_USART_MODE_SPEC {}
#[doc = "`reset()` method sets US_CSR_USART_MODE to value 0"]
impl crate::Resettable for US_CSR_USART_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
