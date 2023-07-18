#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS_DONE` writer - Disable Done Interrupt Disable"]
pub type DIS_DONE_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `SRST` writer - Software Reset Interrupt Disable"]
pub type SRST_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `VSYNC` writer - Vertical Synchronization Interrupt Disable"]
pub type VSYNC_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `PXFR_DONE` writer - Preview DMA Transfer Done Interrupt Disable"]
pub type PXFR_DONE_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `CXFR_DONE` writer - Codec DMA Transfer Done Interrupt Disable"]
pub type CXFR_DONE_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `P_OVR` writer - Preview Datapath Overflow Interrupt Disable"]
pub type P_OVR_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `C_OVR` writer - Codec Datapath Overflow Interrupt Disable"]
pub type C_OVR_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `CRC_ERR` writer - Embedded Synchronization CRC Error Interrupt Disable"]
pub type CRC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `FR_OVR` writer - Frame Rate Overflow Interrupt Disable"]
pub type FR_OVR_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
impl W {
    #[doc = "Bit 1 - Disable Done Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dis_done(&mut self) -> DIS_DONE_W<1> {
        DIS_DONE_W::new(self)
    }
    #[doc = "Bit 2 - Software Reset Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn srst(&mut self) -> SRST_W<2> {
        SRST_W::new(self)
    }
    #[doc = "Bit 10 - Vertical Synchronization Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn vsync(&mut self) -> VSYNC_W<10> {
        VSYNC_W::new(self)
    }
    #[doc = "Bit 16 - Preview DMA Transfer Done Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pxfr_done(&mut self) -> PXFR_DONE_W<16> {
        PXFR_DONE_W::new(self)
    }
    #[doc = "Bit 17 - Codec DMA Transfer Done Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cxfr_done(&mut self) -> CXFR_DONE_W<17> {
        CXFR_DONE_W::new(self)
    }
    #[doc = "Bit 24 - Preview Datapath Overflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p_ovr(&mut self) -> P_OVR_W<24> {
        P_OVR_W::new(self)
    }
    #[doc = "Bit 25 - Codec Datapath Overflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn c_ovr(&mut self) -> C_OVR_W<25> {
        C_OVR_W::new(self)
    }
    #[doc = "Bit 26 - Embedded Synchronization CRC Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn crc_err(&mut self) -> CRC_ERR_W<26> {
        CRC_ERR_W::new(self)
    }
    #[doc = "Bit 27 - Frame Rate Overflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fr_ovr(&mut self) -> FR_OVR_W<27> {
        FR_OVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
