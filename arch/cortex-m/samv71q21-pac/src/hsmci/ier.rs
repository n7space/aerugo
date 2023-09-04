#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `CMDRDY` writer - Command Ready Interrupt Enable"]
pub type CMDRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXRDY` writer - Receiver Ready Interrupt Enable"]
pub type RXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Enable"]
pub type TXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLKE` writer - Data Block Ended Interrupt Enable"]
pub type BLKE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTIP` writer - Data Transfer in Progress Interrupt Enable"]
pub type DTIP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NOTBUSY` writer - Data Not Busy Interrupt Enable"]
pub type NOTBUSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIOIRQA` writer - SDIO Interrupt for Slot A Interrupt Enable"]
pub type SDIOIRQA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIOWAIT` writer - SDIO Read Wait Operation Status Interrupt Enable"]
pub type SDIOWAIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSRCV` writer - Completion Signal Received Interrupt Enable"]
pub type CSRCV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RINDE` writer - Response Index Error Interrupt Enable"]
pub type RINDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDIRE` writer - Response Direction Error Interrupt Enable"]
pub type RDIRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCRCE` writer - Response CRC Error Interrupt Enable"]
pub type RCRCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RENDE` writer - Response End Bit Error Interrupt Enable"]
pub type RENDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTOE` writer - Response Time-out Error Interrupt Enable"]
pub type RTOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCRCE` writer - Data CRC Error Interrupt Enable"]
pub type DCRCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTOE` writer - Data Time-out Error Interrupt Enable"]
pub type DTOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSTOE` writer - Completion Signal Timeout Error Interrupt Enable"]
pub type CSTOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLKOVRE` writer - DMA Block Overrun Error Interrupt Enable"]
pub type BLKOVRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOEMPTY` writer - FIFO empty Interrupt enable"]
pub type FIFOEMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XFRDONE` writer - Transfer Done Interrupt enable"]
pub type XFRDONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACKRCV` writer - Boot Acknowledge Interrupt Enable"]
pub type ACKRCV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACKRCVE` writer - Boot Acknowledge Error Interrupt Enable"]
pub type ACKRCVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVRE` writer - Overrun Interrupt Enable"]
pub type OVRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNRE` writer - Underrun Interrupt Enable"]
pub type UNRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Command Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrdy(&mut self) -> CMDRDY_W<IER_SPEC, 0> {
        CMDRDY_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<IER_SPEC, 1> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<IER_SPEC, 2> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 3 - Data Block Ended Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blke(&mut self) -> BLKE_W<IER_SPEC, 3> {
        BLKE_W::new(self)
    }
    #[doc = "Bit 4 - Data Transfer in Progress Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtip(&mut self) -> DTIP_W<IER_SPEC, 4> {
        DTIP_W::new(self)
    }
    #[doc = "Bit 5 - Data Not Busy Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn notbusy(&mut self) -> NOTBUSY_W<IER_SPEC, 5> {
        NOTBUSY_W::new(self)
    }
    #[doc = "Bit 8 - SDIO Interrupt for Slot A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdioirqa(&mut self) -> SDIOIRQA_W<IER_SPEC, 8> {
        SDIOIRQA_W::new(self)
    }
    #[doc = "Bit 12 - SDIO Read Wait Operation Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdiowait(&mut self) -> SDIOWAIT_W<IER_SPEC, 12> {
        SDIOWAIT_W::new(self)
    }
    #[doc = "Bit 13 - Completion Signal Received Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csrcv(&mut self) -> CSRCV_W<IER_SPEC, 13> {
        CSRCV_W::new(self)
    }
    #[doc = "Bit 16 - Response Index Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rinde(&mut self) -> RINDE_W<IER_SPEC, 16> {
        RINDE_W::new(self)
    }
    #[doc = "Bit 17 - Response Direction Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdire(&mut self) -> RDIRE_W<IER_SPEC, 17> {
        RDIRE_W::new(self)
    }
    #[doc = "Bit 18 - Response CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rcrce(&mut self) -> RCRCE_W<IER_SPEC, 18> {
        RCRCE_W::new(self)
    }
    #[doc = "Bit 19 - Response End Bit Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rende(&mut self) -> RENDE_W<IER_SPEC, 19> {
        RENDE_W::new(self)
    }
    #[doc = "Bit 20 - Response Time-out Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtoe(&mut self) -> RTOE_W<IER_SPEC, 20> {
        RTOE_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcrce(&mut self) -> DCRCE_W<IER_SPEC, 21> {
        DCRCE_W::new(self)
    }
    #[doc = "Bit 22 - Data Time-out Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoe(&mut self) -> DTOE_W<IER_SPEC, 22> {
        DTOE_W::new(self)
    }
    #[doc = "Bit 23 - Completion Signal Timeout Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cstoe(&mut self) -> CSTOE_W<IER_SPEC, 23> {
        CSTOE_W::new(self)
    }
    #[doc = "Bit 24 - DMA Block Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn blkovre(&mut self) -> BLKOVRE_W<IER_SPEC, 24> {
        BLKOVRE_W::new(self)
    }
    #[doc = "Bit 26 - FIFO empty Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoempty(&mut self) -> FIFOEMPTY_W<IER_SPEC, 26> {
        FIFOEMPTY_W::new(self)
    }
    #[doc = "Bit 27 - Transfer Done Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn xfrdone(&mut self) -> XFRDONE_W<IER_SPEC, 27> {
        XFRDONE_W::new(self)
    }
    #[doc = "Bit 28 - Boot Acknowledge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackrcv(&mut self) -> ACKRCV_W<IER_SPEC, 28> {
        ACKRCV_W::new(self)
    }
    #[doc = "Bit 29 - Boot Acknowledge Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackrcve(&mut self) -> ACKRCVE_W<IER_SPEC, 29> {
        ACKRCVE_W::new(self)
    }
    #[doc = "Bit 30 - Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<IER_SPEC, 30> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 31 - Underrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UNRE_W<IER_SPEC, 31> {
        UNRE_W::new(self)
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
