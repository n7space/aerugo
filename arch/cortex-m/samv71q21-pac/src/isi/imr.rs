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
#[doc = "Field `DIS_DONE` reader - Module Disable Operation Completed"]
pub type DIS_DONE_R = crate::BitReader;
#[doc = "Field `SRST` reader - Software Reset Completed"]
pub type SRST_R = crate::BitReader;
#[doc = "Field `VSYNC` reader - Vertical Synchronization"]
pub type VSYNC_R = crate::BitReader;
#[doc = "Field `PXFR_DONE` reader - Preview DMA Transfer Completed"]
pub type PXFR_DONE_R = crate::BitReader;
#[doc = "Field `CXFR_DONE` reader - Codec DMA Transfer Completed"]
pub type CXFR_DONE_R = crate::BitReader;
#[doc = "Field `P_OVR` reader - Preview FIFO Overflow"]
pub type P_OVR_R = crate::BitReader;
#[doc = "Field `C_OVR` reader - Codec FIFO Overflow"]
pub type C_OVR_R = crate::BitReader;
#[doc = "Field `CRC_ERR` reader - CRC Synchronization Error"]
pub type CRC_ERR_R = crate::BitReader;
#[doc = "Field `FR_OVR` reader - Frame Rate Overrun"]
pub type FR_OVR_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Module Disable Operation Completed"]
    #[inline(always)]
    pub fn dis_done(&self) -> DIS_DONE_R {
        DIS_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset Completed"]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Vertical Synchronization"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Preview DMA Transfer Completed"]
    #[inline(always)]
    pub fn pxfr_done(&self) -> PXFR_DONE_R {
        PXFR_DONE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Codec DMA Transfer Completed"]
    #[inline(always)]
    pub fn cxfr_done(&self) -> CXFR_DONE_R {
        CXFR_DONE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Preview FIFO Overflow"]
    #[inline(always)]
    pub fn p_ovr(&self) -> P_OVR_R {
        P_OVR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Codec FIFO Overflow"]
    #[inline(always)]
    pub fn c_ovr(&self) -> C_OVR_R {
        C_OVR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CRC Synchronization Error"]
    #[inline(always)]
    pub fn crc_err(&self) -> CRC_ERR_R {
        CRC_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Frame Rate Overrun"]
    #[inline(always)]
    pub fn fr_ovr(&self) -> FR_OVR_R {
        FR_OVR_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "ISI Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
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
