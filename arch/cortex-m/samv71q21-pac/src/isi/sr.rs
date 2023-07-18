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
#[doc = "Field `ENABLE` reader - Module Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `DIS_DONE` reader - Module Disable Request has Terminated (cleared on read)"]
pub type DIS_DONE_R = crate::BitReader;
#[doc = "Field `SRST` reader - Module Software Reset Request has Terminated (cleared on read)"]
pub type SRST_R = crate::BitReader;
#[doc = "Field `CDC_PND` reader - Pending Codec Request"]
pub type CDC_PND_R = crate::BitReader;
#[doc = "Field `VSYNC` reader - Vertical Synchronization (cleared on read)"]
pub type VSYNC_R = crate::BitReader;
#[doc = "Field `PXFR_DONE` reader - Preview DMA Transfer has Terminated (cleared on read)"]
pub type PXFR_DONE_R = crate::BitReader;
#[doc = "Field `CXFR_DONE` reader - Codec DMA Transfer has Terminated (cleared on read)"]
pub type CXFR_DONE_R = crate::BitReader;
#[doc = "Field `SIP` reader - Synchronization in Progress"]
pub type SIP_R = crate::BitReader;
#[doc = "Field `P_OVR` reader - Preview Datapath Overflow (cleared on read)"]
pub type P_OVR_R = crate::BitReader;
#[doc = "Field `C_OVR` reader - Codec Datapath Overflow (cleared on read)"]
pub type C_OVR_R = crate::BitReader;
#[doc = "Field `CRC_ERR` reader - CRC Synchronization Error (cleared on read)"]
pub type CRC_ERR_R = crate::BitReader;
#[doc = "Field `FR_OVR` reader - Frame Rate Overrun (cleared on read)"]
pub type FR_OVR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Disable Request has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn dis_done(&self) -> DIS_DONE_R {
        DIS_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Module Software Reset Request has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending Codec Request"]
    #[inline(always)]
    pub fn cdc_pnd(&self) -> CDC_PND_R {
        CDC_PND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Vertical Synchronization (cleared on read)"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Preview DMA Transfer has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn pxfr_done(&self) -> PXFR_DONE_R {
        PXFR_DONE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Codec DMA Transfer has Terminated (cleared on read)"]
    #[inline(always)]
    pub fn cxfr_done(&self) -> CXFR_DONE_R {
        CXFR_DONE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Synchronization in Progress"]
    #[inline(always)]
    pub fn sip(&self) -> SIP_R {
        SIP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Preview Datapath Overflow (cleared on read)"]
    #[inline(always)]
    pub fn p_ovr(&self) -> P_OVR_R {
        P_OVR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Codec Datapath Overflow (cleared on read)"]
    #[inline(always)]
    pub fn c_ovr(&self) -> C_OVR_R {
        C_OVR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CRC Synchronization Error (cleared on read)"]
    #[inline(always)]
    pub fn crc_err(&self) -> CRC_ERR_R {
        CRC_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Frame Rate Overrun (cleared on read)"]
    #[inline(always)]
    pub fn fr_ovr(&self) -> FR_OVR_R {
        FR_OVR_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "ISI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
