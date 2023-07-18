#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFS` reader - Management Frame Sent"]
pub type MFS_R = crate::BitReader;
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub type RCOMP_R = crate::BitReader;
#[doc = "Field `RXUBR` reader - RX Used Bit Read"]
pub type RXUBR_R = crate::BitReader;
#[doc = "Field `TXUBR` reader - TX Used Bit Read"]
pub type TXUBR_R = crate::BitReader;
#[doc = "Field `TUR` reader - Transmit Underrun"]
pub type TUR_R = crate::BitReader;
#[doc = "Field `RLEX` reader - Retry Limit Exceeded"]
pub type RLEX_R = crate::BitReader;
#[doc = "Field `TFC` reader - Transmit Frame Corruption Due to AHB Error"]
pub type TFC_R = crate::BitReader;
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub type TCOMP_R = crate::BitReader;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type ROVR_R = crate::BitReader;
#[doc = "Field `HRESP` reader - HRESP Not OK"]
pub type HRESP_R = crate::BitReader;
#[doc = "Field `PFNZ` reader - Pause Frame with Non-zero Pause Quantum Received"]
pub type PFNZ_R = crate::BitReader;
#[doc = "Field `PTZ` reader - Pause Time Zero"]
pub type PTZ_R = crate::BitReader;
#[doc = "Field `PFTR` reader - Pause Frame Transmitted"]
pub type PFTR_R = crate::BitReader;
#[doc = "Field `DRQFR` reader - PTP Delay Request Frame Received"]
pub type DRQFR_R = crate::BitReader;
#[doc = "Field `SFR` reader - PTP Sync Frame Received"]
pub type SFR_R = crate::BitReader;
#[doc = "Field `DRQFT` reader - PTP Delay Request Frame Transmitted"]
pub type DRQFT_R = crate::BitReader;
#[doc = "Field `SFT` reader - PTP Sync Frame Transmitted"]
pub type SFT_R = crate::BitReader;
#[doc = "Field `PDRQFR` reader - PDelay Request Frame Received"]
pub type PDRQFR_R = crate::BitReader;
#[doc = "Field `PDRSFR` reader - PDelay Response Frame Received"]
pub type PDRSFR_R = crate::BitReader;
#[doc = "Field `PDRQFT` reader - PDelay Request Frame Transmitted"]
pub type PDRQFT_R = crate::BitReader;
#[doc = "Field `PDRSFT` reader - PDelay Response Frame Transmitted"]
pub type PDRSFT_R = crate::BitReader;
#[doc = "Field `SRI` reader - TSU Seconds Register Increment"]
pub type SRI_R = crate::BitReader;
#[doc = "Field `RXLPISBC` reader - Receive LPI indication Status Bit Change"]
pub type RXLPISBC_R = crate::BitReader;
#[doc = "Field `WOL` reader - Wake On LAN"]
pub type WOL_R = crate::BitReader;
#[doc = "Field `TSUTIMCOMP` reader - TSU Timer Comparison"]
pub type TSUTIMCOMP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Management Frame Sent"]
    #[inline(always)]
    pub fn mfs(&self) -> MFS_R {
        MFS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RCOMP_R {
        RCOMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RXUBR_R {
        RXUBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX Used Bit Read"]
    #[inline(always)]
    pub fn txubr(&self) -> TXUBR_R {
        TXUBR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Underrun"]
    #[inline(always)]
    pub fn tur(&self) -> TUR_R {
        TUR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rlex(&self) -> RLEX_R {
        RLEX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&self) -> TFC_R {
        TFC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TCOMP_R {
        TCOMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HRESP_R {
        HRESP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pause Frame with Non-zero Pause Quantum Received"]
    #[inline(always)]
    pub fn pfnz(&self) -> PFNZ_R {
        PFNZ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    pub fn ptz(&self) -> PTZ_R {
        PTZ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pause Frame Transmitted"]
    #[inline(always)]
    pub fn pftr(&self) -> PFTR_R {
        PFTR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - PTP Delay Request Frame Received"]
    #[inline(always)]
    pub fn drqfr(&self) -> DRQFR_R {
        DRQFR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PTP Sync Frame Received"]
    #[inline(always)]
    pub fn sfr(&self) -> SFR_R {
        SFR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PTP Delay Request Frame Transmitted"]
    #[inline(always)]
    pub fn drqft(&self) -> DRQFT_R {
        DRQFT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PTP Sync Frame Transmitted"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PDelay Request Frame Received"]
    #[inline(always)]
    pub fn pdrqfr(&self) -> PDRQFR_R {
        PDRQFR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PDelay Response Frame Received"]
    #[inline(always)]
    pub fn pdrsfr(&self) -> PDRSFR_R {
        PDRSFR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PDelay Request Frame Transmitted"]
    #[inline(always)]
    pub fn pdrqft(&self) -> PDRQFT_R {
        PDRQFT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PDelay Response Frame Transmitted"]
    #[inline(always)]
    pub fn pdrsft(&self) -> PDRSFT_R {
        PDRSFT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TSU Seconds Register Increment"]
    #[inline(always)]
    pub fn sri(&self) -> SRI_R {
        SRI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive LPI indication Status Bit Change"]
    #[inline(always)]
    pub fn rxlpisbc(&self) -> RXLPISBC_R {
        RXLPISBC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake On LAN"]
    #[inline(always)]
    pub fn wol(&self) -> WOL_R {
        WOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TSU Timer Comparison"]
    #[inline(always)]
    pub fn tsutimcomp(&self) -> TSUTIMCOMP_R {
        TSUTIMCOMP_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
