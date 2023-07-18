#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRDY` reader - Command Ready Interrupt Mask"]
pub type CMDRDY_R = crate::BitReader;
#[doc = "Field `RXRDY` reader - Receiver Ready Interrupt Mask"]
pub type RXRDY_R = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmit Ready Interrupt Mask"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `BLKE` reader - Data Block Ended Interrupt Mask"]
pub type BLKE_R = crate::BitReader;
#[doc = "Field `DTIP` reader - Data Transfer in Progress Interrupt Mask"]
pub type DTIP_R = crate::BitReader;
#[doc = "Field `NOTBUSY` reader - Data Not Busy Interrupt Mask"]
pub type NOTBUSY_R = crate::BitReader;
#[doc = "Field `SDIOIRQA` reader - SDIO Interrupt for Slot A Interrupt Mask"]
pub type SDIOIRQA_R = crate::BitReader;
#[doc = "Field `SDIOWAIT` reader - SDIO Read Wait Operation Status Interrupt Mask"]
pub type SDIOWAIT_R = crate::BitReader;
#[doc = "Field `CSRCV` reader - Completion Signal Received Interrupt Mask"]
pub type CSRCV_R = crate::BitReader;
#[doc = "Field `RINDE` reader - Response Index Error Interrupt Mask"]
pub type RINDE_R = crate::BitReader;
#[doc = "Field `RDIRE` reader - Response Direction Error Interrupt Mask"]
pub type RDIRE_R = crate::BitReader;
#[doc = "Field `RCRCE` reader - Response CRC Error Interrupt Mask"]
pub type RCRCE_R = crate::BitReader;
#[doc = "Field `RENDE` reader - Response End Bit Error Interrupt Mask"]
pub type RENDE_R = crate::BitReader;
#[doc = "Field `RTOE` reader - Response Time-out Error Interrupt Mask"]
pub type RTOE_R = crate::BitReader;
#[doc = "Field `DCRCE` reader - Data CRC Error Interrupt Mask"]
pub type DCRCE_R = crate::BitReader;
#[doc = "Field `DTOE` reader - Data Time-out Error Interrupt Mask"]
pub type DTOE_R = crate::BitReader;
#[doc = "Field `CSTOE` reader - Completion Signal Time-out Error Interrupt Mask"]
pub type CSTOE_R = crate::BitReader;
#[doc = "Field `BLKOVRE` reader - DMA Block Overrun Error Interrupt Mask"]
pub type BLKOVRE_R = crate::BitReader;
#[doc = "Field `FIFOEMPTY` reader - FIFO Empty Interrupt Mask"]
pub type FIFOEMPTY_R = crate::BitReader;
#[doc = "Field `XFRDONE` reader - Transfer Done Interrupt Mask"]
pub type XFRDONE_R = crate::BitReader;
#[doc = "Field `ACKRCV` reader - Boot Operation Acknowledge Received Interrupt Mask"]
pub type ACKRCV_R = crate::BitReader;
#[doc = "Field `ACKRCVE` reader - Boot Operation Acknowledge Error Interrupt Mask"]
pub type ACKRCVE_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Interrupt Mask"]
pub type OVRE_R = crate::BitReader;
#[doc = "Field `UNRE` reader - Underrun Interrupt Mask"]
pub type UNRE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command Ready Interrupt Mask"]
    #[inline(always)]
    pub fn cmdrdy(&self) -> CMDRDY_R {
        CMDRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Ready Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Block Ended Interrupt Mask"]
    #[inline(always)]
    pub fn blke(&self) -> BLKE_R {
        BLKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transfer in Progress Interrupt Mask"]
    #[inline(always)]
    pub fn dtip(&self) -> DTIP_R {
        DTIP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Not Busy Interrupt Mask"]
    #[inline(always)]
    pub fn notbusy(&self) -> NOTBUSY_R {
        NOTBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO Interrupt for Slot A Interrupt Mask"]
    #[inline(always)]
    pub fn sdioirqa(&self) -> SDIOIRQA_R {
        SDIOIRQA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SDIO Read Wait Operation Status Interrupt Mask"]
    #[inline(always)]
    pub fn sdiowait(&self) -> SDIOWAIT_R {
        SDIOWAIT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Completion Signal Received Interrupt Mask"]
    #[inline(always)]
    pub fn csrcv(&self) -> CSRCV_R {
        CSRCV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Response Index Error Interrupt Mask"]
    #[inline(always)]
    pub fn rinde(&self) -> RINDE_R {
        RINDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Response Direction Error Interrupt Mask"]
    #[inline(always)]
    pub fn rdire(&self) -> RDIRE_R {
        RDIRE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Response CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn rcrce(&self) -> RCRCE_R {
        RCRCE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Response End Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn rende(&self) -> RENDE_R {
        RENDE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Response Time-out Error Interrupt Mask"]
    #[inline(always)]
    pub fn rtoe(&self) -> RTOE_R {
        RTOE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn dcrce(&self) -> DCRCE_R {
        DCRCE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Time-out Error Interrupt Mask"]
    #[inline(always)]
    pub fn dtoe(&self) -> DTOE_R {
        DTOE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Completion Signal Time-out Error Interrupt Mask"]
    #[inline(always)]
    pub fn cstoe(&self) -> CSTOE_R {
        CSTOE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA Block Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn blkovre(&self) -> BLKOVRE_R {
        BLKOVRE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn fifoempty(&self) -> FIFOEMPTY_R {
        FIFOEMPTY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transfer Done Interrupt Mask"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XFRDONE_R {
        XFRDONE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Boot Operation Acknowledge Received Interrupt Mask"]
    #[inline(always)]
    pub fn ackrcv(&self) -> ACKRCV_R {
        ACKRCV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Boot Operation Acknowledge Error Interrupt Mask"]
    #[inline(always)]
    pub fn ackrcve(&self) -> ACKRCVE_R {
        ACKRCVE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Underrun Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
