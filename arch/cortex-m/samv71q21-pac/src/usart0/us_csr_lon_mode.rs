#[doc = "Register `US_CSR_LON_MODE` reader"]
pub struct R(crate::R<US_CSR_LON_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_CSR_LON_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_CSR_LON_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_CSR_LON_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - Receiver Ready (cleared by reading US_RHR)"]
pub type RXRDY_R = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmitter Ready (cleared by writing US_THR)"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
pub type OVRE_R = crate::BitReader;
#[doc = "Field `LSFE` reader - LON Short Frame Error"]
pub type LSFE_R = crate::BitReader;
#[doc = "Field `LCRCE` reader - LON CRC Error"]
pub type LCRCE_R = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty (cleared by writing US_THR)"]
pub type TXEMPTY_R = crate::BitReader;
#[doc = "Field `UNRE` reader - Underrun Error"]
pub type UNRE_R = crate::BitReader;
#[doc = "Field `LTXD` reader - LON Transmission End Flag"]
pub type LTXD_R = crate::BitReader;
#[doc = "Field `LCOL` reader - LON Collision Detected Flag"]
pub type LCOL_R = crate::BitReader;
#[doc = "Field `LFET` reader - LON Frame Early Termination"]
pub type LFET_R = crate::BitReader;
#[doc = "Field `LRXD` reader - LON Reception End Flag"]
pub type LRXD_R = crate::BitReader;
#[doc = "Field `LBLOVFE` reader - LON Backlog Overflow Error"]
pub type LBLOVFE_R = crate::BitReader;
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
    #[doc = "Bit 5 - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LON Short Frame Error"]
    #[inline(always)]
    pub fn lsfe(&self) -> LSFE_R {
        LSFE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LON CRC Error"]
    #[inline(always)]
    pub fn lcrce(&self) -> LCRCE_R {
        LCRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Underrun Error"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 24 - LON Transmission End Flag"]
    #[inline(always)]
    pub fn ltxd(&self) -> LTXD_R {
        LTXD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LON Collision Detected Flag"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LON Frame Early Termination"]
    #[inline(always)]
    pub fn lfet(&self) -> LFET_R {
        LFET_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LON Reception End Flag"]
    #[inline(always)]
    pub fn lrxd(&self) -> LRXD_R {
        LRXD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LON Backlog Overflow Error"]
    #[inline(always)]
    pub fn lblovfe(&self) -> LBLOVFE_R {
        LBLOVFE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_csr_lon_mode](index.html) module"]
pub struct US_CSR_LON_MODE_SPEC;
impl crate::RegisterSpec for US_CSR_LON_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_csr_lon_mode::R](R) reader structure"]
impl crate::Readable for US_CSR_LON_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_CSR_LON_MODE to value 0"]
impl crate::Resettable for US_CSR_LON_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
