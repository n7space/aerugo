#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCOMP` reader - Transmission Completed (cleared by writing TWIHS_THR)"]
pub type TXCOMP_R = crate::BitReader;
#[doc = "Field `RXRDY` reader - Receive Holding Register Ready (cleared by reading TWIHS_RHR)"]
pub type RXRDY_R = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmit Holding Register Ready (cleared by writing TWIHS_THR)"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `SVREAD` reader - Slave Read"]
pub type SVREAD_R = crate::BitReader;
#[doc = "Field `SVACC` reader - Slave Access"]
pub type SVACC_R = crate::BitReader;
#[doc = "Field `GACC` reader - General Call Access (cleared on read)"]
pub type GACC_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error (cleared on read)"]
pub type OVRE_R = crate::BitReader;
#[doc = "Field `UNRE` reader - Underrun Error (cleared on read)"]
pub type UNRE_R = crate::BitReader;
#[doc = "Field `NACK` reader - Not Acknowledged (cleared on read)"]
pub type NACK_R = crate::BitReader;
#[doc = "Field `ARBLST` reader - Arbitration Lost (cleared on read)"]
pub type ARBLST_R = crate::BitReader;
#[doc = "Field `SCLWS` reader - Clock Wait State"]
pub type SCLWS_R = crate::BitReader;
#[doc = "Field `EOSACC` reader - End Of Slave Access (cleared on read)"]
pub type EOSACC_R = crate::BitReader;
#[doc = "Field `MCACK` reader - Master Code Acknowledge (cleared on read)"]
pub type MCACK_R = crate::BitReader;
#[doc = "Field `TOUT` reader - Timeout Error (cleared on read)"]
pub type TOUT_R = crate::BitReader;
#[doc = "Field `PECERR` reader - PEC Error (cleared on read)"]
pub type PECERR_R = crate::BitReader;
#[doc = "Field `SMBDAM` reader - SMBus Default Address Match (cleared on read)"]
pub type SMBDAM_R = crate::BitReader;
#[doc = "Field `SMBHHM` reader - SMBus Host Header Address Match (cleared on read)"]
pub type SMBHHM_R = crate::BitReader;
#[doc = "Field `SCL` reader - SCL Line Value"]
pub type SCL_R = crate::BitReader;
#[doc = "Field `SDA` reader - SDA Line Value"]
pub type SDA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission Completed (cleared by writing TWIHS_THR)"]
    #[inline(always)]
    pub fn txcomp(&self) -> TXCOMP_R {
        TXCOMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready (cleared by reading TWIHS_RHR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready (cleared by writing TWIHS_THR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Read"]
    #[inline(always)]
    pub fn svread(&self) -> SVREAD_R {
        SVREAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Access"]
    #[inline(always)]
    pub fn svacc(&self) -> SVACC_R {
        SVACC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call Access (cleared on read)"]
    #[inline(always)]
    pub fn gacc(&self) -> GACC_R {
        GACC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun Error (cleared on read)"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Underrun Error (cleared on read)"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Not Acknowledged (cleared on read)"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost (cleared on read)"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock Wait State"]
    #[inline(always)]
    pub fn sclws(&self) -> SCLWS_R {
        SCLWS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End Of Slave Access (cleared on read)"]
    #[inline(always)]
    pub fn eosacc(&self) -> EOSACC_R {
        EOSACC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Master Code Acknowledge (cleared on read)"]
    #[inline(always)]
    pub fn mcack(&self) -> MCACK_R {
        MCACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Error (cleared on read)"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PEC Error (cleared on read)"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus Default Address Match (cleared on read)"]
    #[inline(always)]
    pub fn smbdam(&self) -> SMBDAM_R {
        SMBDAM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus Host Header Address Match (cleared on read)"]
    #[inline(always)]
    pub fn smbhhm(&self) -> SMBHHM_R {
        SMBHHM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - SCL Line Value"]
    #[inline(always)]
    pub fn scl(&self) -> SCL_R {
        SCL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SDA Line Value"]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
