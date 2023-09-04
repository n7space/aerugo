#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `CMDRDY` writer - Command Ready Interrupt Disable"]
pub type CMDRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXRDY` writer - Receiver Ready Interrupt Disable"]
pub type RXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXRDY` writer - Transmit Ready Interrupt Disable"]
pub type TXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLKE` writer - Data Block Ended Interrupt Disable"]
pub type BLKE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTIP` writer - Data Transfer in Progress Interrupt Disable"]
pub type DTIP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NOTBUSY` writer - Data Not Busy Interrupt Disable"]
pub type NOTBUSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIOIRQA` writer - SDIO Interrupt for Slot A Interrupt Disable"]
pub type SDIOIRQA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIOWAIT` writer - SDIO Read Wait Operation Status Interrupt Disable"]
pub type SDIOWAIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSRCV` writer - Completion Signal received interrupt Disable"]
pub type CSRCV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RINDE` writer - Response Index Error Interrupt Disable"]
pub type RINDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDIRE` writer - Response Direction Error Interrupt Disable"]
pub type RDIRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCRCE` writer - Response CRC Error Interrupt Disable"]
pub type RCRCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RENDE` writer - Response End Bit Error Interrupt Disable"]
pub type RENDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTOE` writer - Response Time-out Error Interrupt Disable"]
pub type RTOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCRCE` writer - Data CRC Error Interrupt Disable"]
pub type DCRCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTOE` writer - Data Time-out Error Interrupt Disable"]
pub type DTOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSTOE` writer - Completion Signal Time out Error Interrupt Disable"]
pub type CSTOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLKOVRE` writer - DMA Block Overrun Error Interrupt Disable"]
pub type BLKOVRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOEMPTY` writer - FIFO empty Interrupt Disable"]
pub type FIFOEMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XFRDONE` writer - Transfer Done Interrupt Disable"]
pub type XFRDONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACKRCV` writer - Boot Acknowledge Interrupt Disable"]
pub type ACKRCV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACKRCVE` writer - Boot Acknowledge Error Interrupt Disable"]
pub type ACKRCVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVRE` writer - Overrun Interrupt Disable"]
pub type OVRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNRE` writer - Underrun Interrupt Disable"]
pub type UNRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Command Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrdy(&mut self) -> CMDRDY_W<IDR_SPEC, 0> {
        CMDRDY_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<IDR_SPEC, 1> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<IDR_SPEC, 2> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 3 - Data Block Ended Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn blke(&mut self) -> BLKE_W<IDR_SPEC, 3> {
        BLKE_W::new(self)
    }
    #[doc = "Bit 4 - Data Transfer in Progress Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dtip(&mut self) -> DTIP_W<IDR_SPEC, 4> {
        DTIP_W::new(self)
    }
    #[doc = "Bit 5 - Data Not Busy Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn notbusy(&mut self) -> NOTBUSY_W<IDR_SPEC, 5> {
        NOTBUSY_W::new(self)
    }
    #[doc = "Bit 8 - SDIO Interrupt for Slot A Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sdioirqa(&mut self) -> SDIOIRQA_W<IDR_SPEC, 8> {
        SDIOIRQA_W::new(self)
    }
    #[doc = "Bit 12 - SDIO Read Wait Operation Status Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sdiowait(&mut self) -> SDIOWAIT_W<IDR_SPEC, 12> {
        SDIOWAIT_W::new(self)
    }
    #[doc = "Bit 13 - Completion Signal received interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn csrcv(&mut self) -> CSRCV_W<IDR_SPEC, 13> {
        CSRCV_W::new(self)
    }
    #[doc = "Bit 16 - Response Index Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rinde(&mut self) -> RINDE_W<IDR_SPEC, 16> {
        RINDE_W::new(self)
    }
    #[doc = "Bit 17 - Response Direction Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rdire(&mut self) -> RDIRE_W<IDR_SPEC, 17> {
        RDIRE_W::new(self)
    }
    #[doc = "Bit 18 - Response CRC Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rcrce(&mut self) -> RCRCE_W<IDR_SPEC, 18> {
        RCRCE_W::new(self)
    }
    #[doc = "Bit 19 - Response End Bit Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rende(&mut self) -> RENDE_W<IDR_SPEC, 19> {
        RENDE_W::new(self)
    }
    #[doc = "Bit 20 - Response Time-out Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rtoe(&mut self) -> RTOE_W<IDR_SPEC, 20> {
        RTOE_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dcrce(&mut self) -> DCRCE_W<IDR_SPEC, 21> {
        DCRCE_W::new(self)
    }
    #[doc = "Bit 22 - Data Time-out Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoe(&mut self) -> DTOE_W<IDR_SPEC, 22> {
        DTOE_W::new(self)
    }
    #[doc = "Bit 23 - Completion Signal Time out Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cstoe(&mut self) -> CSTOE_W<IDR_SPEC, 23> {
        CSTOE_W::new(self)
    }
    #[doc = "Bit 24 - DMA Block Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn blkovre(&mut self) -> BLKOVRE_W<IDR_SPEC, 24> {
        BLKOVRE_W::new(self)
    }
    #[doc = "Bit 26 - FIFO empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoempty(&mut self) -> FIFOEMPTY_W<IDR_SPEC, 26> {
        FIFOEMPTY_W::new(self)
    }
    #[doc = "Bit 27 - Transfer Done Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn xfrdone(&mut self) -> XFRDONE_W<IDR_SPEC, 27> {
        XFRDONE_W::new(self)
    }
    #[doc = "Bit 28 - Boot Acknowledge Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ackrcv(&mut self) -> ACKRCV_W<IDR_SPEC, 28> {
        ACKRCV_W::new(self)
    }
    #[doc = "Bit 29 - Boot Acknowledge Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ackrcve(&mut self) -> ACKRCVE_W<IDR_SPEC, 29> {
        ACKRCVE_W::new(self)
    }
    #[doc = "Bit 30 - Overrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<IDR_SPEC, 30> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 31 - Underrun Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UNRE_W<IDR_SPEC, 31> {
        UNRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
