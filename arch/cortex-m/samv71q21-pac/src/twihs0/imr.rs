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
#[doc = "Field `TXCOMP` reader - Transmission Completed Interrupt Mask"]
pub type TXCOMP_R = crate::BitReader;
#[doc = "Field `RXRDY` reader - Receive Holding Register Ready Interrupt Mask"]
pub type RXRDY_R = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmit Holding Register Ready Interrupt Mask"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `SVACC` reader - Slave Access Interrupt Mask"]
pub type SVACC_R = crate::BitReader;
#[doc = "Field `GACC` reader - General Call Access Interrupt Mask"]
pub type GACC_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub type OVRE_R = crate::BitReader;
#[doc = "Field `UNRE` reader - Underrun Error Interrupt Mask"]
pub type UNRE_R = crate::BitReader;
#[doc = "Field `NACK` reader - Not Acknowledge Interrupt Mask"]
pub type NACK_R = crate::BitReader;
#[doc = "Field `ARBLST` reader - Arbitration Lost Interrupt Mask"]
pub type ARBLST_R = crate::BitReader;
#[doc = "Field `SCL_WS` reader - Clock Wait State Interrupt Mask"]
pub type SCL_WS_R = crate::BitReader;
#[doc = "Field `EOSACC` reader - End Of Slave Access Interrupt Mask"]
pub type EOSACC_R = crate::BitReader;
#[doc = "Field `MCACK` reader - Master Code Acknowledge Interrupt Mask"]
pub type MCACK_R = crate::BitReader;
#[doc = "Field `TOUT` reader - Timeout Error Interrupt Mask"]
pub type TOUT_R = crate::BitReader;
#[doc = "Field `PECERR` reader - PEC Error Interrupt Mask"]
pub type PECERR_R = crate::BitReader;
#[doc = "Field `SMBDAM` reader - SMBus Default Address Match Interrupt Mask"]
pub type SMBDAM_R = crate::BitReader;
#[doc = "Field `SMBHHM` reader - SMBus Host Header Address Match Interrupt Mask"]
pub type SMBHHM_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission Completed Interrupt Mask"]
    #[inline(always)]
    pub fn txcomp(&self) -> TXCOMP_R {
        TXCOMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Access Interrupt Mask"]
    #[inline(always)]
    pub fn svacc(&self) -> SVACC_R {
        SVACC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call Access Interrupt Mask"]
    #[inline(always)]
    pub fn gacc(&self) -> GACC_R {
        GACC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Not Acknowledge Interrupt Mask"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock Wait State Interrupt Mask"]
    #[inline(always)]
    pub fn scl_ws(&self) -> SCL_WS_R {
        SCL_WS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End Of Slave Access Interrupt Mask"]
    #[inline(always)]
    pub fn eosacc(&self) -> EOSACC_R {
        EOSACC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Master Code Acknowledge Interrupt Mask"]
    #[inline(always)]
    pub fn mcack(&self) -> MCACK_R {
        MCACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Error Interrupt Mask"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PEC Error Interrupt Mask"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus Default Address Match Interrupt Mask"]
    #[inline(always)]
    pub fn smbdam(&self) -> SMBDAM_R {
        SMBDAM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus Host Header Address Match Interrupt Mask"]
    #[inline(always)]
    pub fn smbhhm(&self) -> SMBHHM_R {
        SMBHHM_R::new(((self.bits >> 21) & 1) != 0)
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
