#[doc = "Register `HSTIFR` writer"]
pub struct W(crate::W<HSTIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTIFR_SPEC>;
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
impl From<crate::W<HSTIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCONNIS` writer - Device Connection Interrupt Set"]
pub type DCONNIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `DDISCIS` writer - Device Disconnection Interrupt Set"]
pub type DDISCIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `RSTIS` writer - USB Reset Sent Interrupt Set"]
pub type RSTIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `RSMEDIS` writer - Downstream Resume Sent Interrupt Set"]
pub type RSMEDIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `RXRSMIS` writer - Upstream Resume Received Interrupt Set"]
pub type RXRSMIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `HSOFIS` writer - Host Start of Frame Interrupt Set"]
pub type HSOFIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `HWUPIS` writer - Host Wake-Up Interrupt Set"]
pub type HWUPIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `DMA_1` writer - DMA Channel 0 Interrupt Set"]
pub type DMA_1_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `DMA_2` writer - DMA Channel 1 Interrupt Set"]
pub type DMA_2_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `DMA_3` writer - DMA Channel 2 Interrupt Set"]
pub type DMA_3_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `DMA_4` writer - DMA Channel 3 Interrupt Set"]
pub type DMA_4_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `DMA_5` writer - DMA Channel 4 Interrupt Set"]
pub type DMA_5_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `DMA_6` writer - DMA Channel 5 Interrupt Set"]
pub type DMA_6_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
#[doc = "Field `DMA_7` writer - DMA Channel 6 Interrupt Set"]
pub type DMA_7_W<'a, const O: u8> = crate::BitWriter<'a, HSTIFR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Device Connection Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dconnis(&mut self) -> DCONNIS_W<0> {
        DCONNIS_W::new(self)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn ddiscis(&mut self) -> DDISCIS_W<1> {
        DDISCIS_W::new(self)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rstis(&mut self) -> RSTIS_W<2> {
        RSTIS_W::new(self)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rsmedis(&mut self) -> RSMEDIS_W<3> {
        RSMEDIS_W::new(self)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmis(&mut self) -> RXRSMIS_W<4> {
        RXRSMIS_W::new(self)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hsofis(&mut self) -> HSOFIS_W<5> {
        HSOFIS_W::new(self)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hwupis(&mut self) -> HWUPIS_W<6> {
        HWUPIS_W::new(self)
    }
    #[doc = "Bit 25 - DMA Channel 0 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> DMA_1_W<25> {
        DMA_1_W::new(self)
    }
    #[doc = "Bit 26 - DMA Channel 1 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> DMA_2_W<26> {
        DMA_2_W::new(self)
    }
    #[doc = "Bit 27 - DMA Channel 2 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> DMA_3_W<27> {
        DMA_3_W::new(self)
    }
    #[doc = "Bit 28 - DMA Channel 3 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> DMA_4_W<28> {
        DMA_4_W::new(self)
    }
    #[doc = "Bit 29 - DMA Channel 4 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> DMA_5_W<29> {
        DMA_5_W::new(self)
    }
    #[doc = "Bit 30 - DMA Channel 5 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> DMA_6_W<30> {
        DMA_6_W::new(self)
    }
    #[doc = "Bit 31 - DMA Channel 6 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_7(&mut self) -> DMA_7_W<31> {
        DMA_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Global Interrupt Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstifr](index.html) module"]
pub struct HSTIFR_SPEC;
impl crate::RegisterSpec for HSTIFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hstifr::W](W) writer structure"]
impl crate::Writable for HSTIFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTIFR to value 0"]
impl crate::Resettable for HSTIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
