#[doc = "Register `DEVIFR` writer"]
pub type W = crate::W<DEVIFR_SPEC>;
#[doc = "Field `SUSPS` writer - Suspend Interrupt Set"]
pub type SUSPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSOFS` writer - Micro Start of Frame Interrupt Set"]
pub type MSOFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFS` writer - Start of Frame Interrupt Set"]
pub type SOFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EORSTS` writer - End of Reset Interrupt Set"]
pub type EORSTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPS` writer - Wake-Up Interrupt Set"]
pub type WAKEUPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EORSMS` writer - End of Resume Interrupt Set"]
pub type EORSMS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPRSMS` writer - Upstream Resume Interrupt Set"]
pub type UPRSMS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Set"]
pub type DMA_1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Set"]
pub type DMA_2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Set"]
pub type DMA_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Set"]
pub type DMA_4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Set"]
pub type DMA_5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Set"]
pub type DMA_6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_7` writer - DMA Channel 7 Interrupt Set"]
pub type DMA_7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn susps(&mut self) -> SUSPS_W<DEVIFR_SPEC, 0> {
        SUSPS_W::new(self)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn msofs(&mut self) -> MSOFS_W<DEVIFR_SPEC, 1> {
        MSOFS_W::new(self)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn sofs(&mut self) -> SOFS_W<DEVIFR_SPEC, 2> {
        SOFS_W::new(self)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn eorsts(&mut self) -> EORSTS_W<DEVIFR_SPEC, 3> {
        EORSTS_W::new(self)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn wakeups(&mut self) -> WAKEUPS_W<DEVIFR_SPEC, 4> {
        WAKEUPS_W::new(self)
    }
    #[doc = "Bit 5 - End of Resume Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn eorsms(&mut self) -> EORSMS_W<DEVIFR_SPEC, 5> {
        EORSMS_W::new(self)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn uprsms(&mut self) -> UPRSMS_W<DEVIFR_SPEC, 6> {
        UPRSMS_W::new(self)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> DMA_1_W<DEVIFR_SPEC, 25> {
        DMA_1_W::new(self)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> DMA_2_W<DEVIFR_SPEC, 26> {
        DMA_2_W::new(self)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> DMA_3_W<DEVIFR_SPEC, 27> {
        DMA_3_W::new(self)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> DMA_4_W<DEVIFR_SPEC, 28> {
        DMA_4_W::new(self)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> DMA_5_W<DEVIFR_SPEC, 29> {
        DMA_5_W::new(self)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> DMA_6_W<DEVIFR_SPEC, 30> {
        DMA_6_W::new(self)
    }
    #[doc = "Bit 31 - DMA Channel 7 Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma_7(&mut self) -> DMA_7_W<DEVIFR_SPEC, 31> {
        DMA_7_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Global Interrupt Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devifr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVIFR_SPEC;
impl crate::RegisterSpec for DEVIFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devifr::W`](W) writer structure"]
impl crate::Writable for DEVIFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVIFR to value 0"]
impl crate::Resettable for DEVIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
