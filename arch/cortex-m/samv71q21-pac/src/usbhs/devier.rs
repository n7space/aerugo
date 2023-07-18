#[doc = "Register `DEVIER` writer"]
pub struct W(crate::W<DEVIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVIER_SPEC>;
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
impl From<crate::W<DEVIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPES` writer - Suspend Interrupt Enable"]
pub type SUSPES_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `MSOFES` writer - Micro Start of Frame Interrupt Enable"]
pub type MSOFES_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `SOFES` writer - Start of Frame Interrupt Enable"]
pub type SOFES_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `EORSTES` writer - End of Reset Interrupt Enable"]
pub type EORSTES_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `WAKEUPES` writer - Wake-Up Interrupt Enable"]
pub type WAKEUPES_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `EORSMES` writer - End of Resume Interrupt Enable"]
pub type EORSMES_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `UPRSMES` writer - Upstream Resume Interrupt Enable"]
pub type UPRSMES_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `PEP_0` writer - Endpoint 0 Interrupt Enable"]
pub type PEP_0_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `PEP_1` writer - Endpoint 1 Interrupt Enable"]
pub type PEP_1_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `PEP_2` writer - Endpoint 2 Interrupt Enable"]
pub type PEP_2_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `PEP_3` writer - Endpoint 3 Interrupt Enable"]
pub type PEP_3_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `PEP_4` writer - Endpoint 4 Interrupt Enable"]
pub type PEP_4_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `PEP_5` writer - Endpoint 5 Interrupt Enable"]
pub type PEP_5_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `PEP_6` writer - Endpoint 6 Interrupt Enable"]
pub type PEP_6_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `PEP_7` writer - Endpoint 7 Interrupt Enable"]
pub type PEP_7_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `PEP_8` writer - Endpoint 8 Interrupt Enable"]
pub type PEP_8_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `PEP_9` writer - Endpoint 9 Interrupt Enable"]
pub type PEP_9_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `DMA_1` writer - DMA Channel 1 Interrupt Enable"]
pub type DMA_1_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `DMA_2` writer - DMA Channel 2 Interrupt Enable"]
pub type DMA_2_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `DMA_3` writer - DMA Channel 3 Interrupt Enable"]
pub type DMA_3_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `DMA_4` writer - DMA Channel 4 Interrupt Enable"]
pub type DMA_4_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `DMA_5` writer - DMA Channel 5 Interrupt Enable"]
pub type DMA_5_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `DMA_6` writer - DMA Channel 6 Interrupt Enable"]
pub type DMA_6_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
#[doc = "Field `DMA_7` writer - DMA Channel 7 Interrupt Enable"]
pub type DMA_7_W<'a, const O: u8> = crate::BitWriter<'a, DEVIER_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn suspes(&mut self) -> SUSPES_W<0> {
        SUSPES_W::new(self)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn msofes(&mut self) -> MSOFES_W<1> {
        MSOFES_W::new(self)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofes(&mut self) -> SOFES_W<2> {
        SOFES_W::new(self)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eorstes(&mut self) -> EORSTES_W<3> {
        EORSTES_W::new(self)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupes(&mut self) -> WAKEUPES_W<4> {
        WAKEUPES_W::new(self)
    }
    #[doc = "Bit 5 - End of Resume Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eorsmes(&mut self) -> EORSMES_W<5> {
        EORSMES_W::new(self)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uprsmes(&mut self) -> UPRSMES_W<6> {
        UPRSMES_W::new(self)
    }
    #[doc = "Bit 12 - Endpoint 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_0(&mut self) -> PEP_0_W<12> {
        PEP_0_W::new(self)
    }
    #[doc = "Bit 13 - Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_1(&mut self) -> PEP_1_W<13> {
        PEP_1_W::new(self)
    }
    #[doc = "Bit 14 - Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_2(&mut self) -> PEP_2_W<14> {
        PEP_2_W::new(self)
    }
    #[doc = "Bit 15 - Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_3(&mut self) -> PEP_3_W<15> {
        PEP_3_W::new(self)
    }
    #[doc = "Bit 16 - Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_4(&mut self) -> PEP_4_W<16> {
        PEP_4_W::new(self)
    }
    #[doc = "Bit 17 - Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_5(&mut self) -> PEP_5_W<17> {
        PEP_5_W::new(self)
    }
    #[doc = "Bit 18 - Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_6(&mut self) -> PEP_6_W<18> {
        PEP_6_W::new(self)
    }
    #[doc = "Bit 19 - Endpoint 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_7(&mut self) -> PEP_7_W<19> {
        PEP_7_W::new(self)
    }
    #[doc = "Bit 20 - Endpoint 8 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_8(&mut self) -> PEP_8_W<20> {
        PEP_8_W::new(self)
    }
    #[doc = "Bit 21 - Endpoint 9 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_9(&mut self) -> PEP_9_W<21> {
        PEP_9_W::new(self)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> DMA_1_W<25> {
        DMA_1_W::new(self)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> DMA_2_W<26> {
        DMA_2_W::new(self)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> DMA_3_W<27> {
        DMA_3_W::new(self)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> DMA_4_W<28> {
        DMA_4_W::new(self)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> DMA_5_W<29> {
        DMA_5_W::new(self)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> DMA_6_W<30> {
        DMA_6_W::new(self)
    }
    #[doc = "Bit 31 - DMA Channel 7 Interrupt Enable"]
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
#[doc = "Device Global Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devier](index.html) module"]
pub struct DEVIER_SPEC;
impl crate::RegisterSpec for DEVIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [devier::W](W) writer structure"]
impl crate::Writable for DEVIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVIER to value 0"]
impl crate::Resettable for DEVIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
