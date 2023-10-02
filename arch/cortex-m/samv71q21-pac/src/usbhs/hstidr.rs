#[doc = "Register `HSTIDR` writer"]
pub type W = crate::W<HSTIDR_SPEC>;
#[doc = "Field `DCONNIEC` writer - Device Connection Interrupt Disable"]
pub type DCONNIEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DDISCIEC` writer - Device Disconnection Interrupt Disable"]
pub type DDISCIEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTIEC` writer - USB Reset Sent Interrupt Disable"]
pub type RSTIEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSMEDIEC` writer - Downstream Resume Sent Interrupt Disable"]
pub type RSMEDIEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXRSMIEC` writer - Upstream Resume Received Interrupt Disable"]
pub type RXRSMIEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSOFIEC` writer - Host Start of Frame Interrupt Disable"]
pub type HSOFIEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HWUPIEC` writer - Host Wake-Up Interrupt Disable"]
pub type HWUPIEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_0` writer - Pipe 0 Interrupt Disable"]
pub type PEP_0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_1` writer - Pipe 1 Interrupt Disable"]
pub type PEP_1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_2` writer - Pipe 2 Interrupt Disable"]
pub type PEP_2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_3` writer - Pipe 3 Interrupt Disable"]
pub type PEP_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_4` writer - Pipe 4 Interrupt Disable"]
pub type PEP_4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_5` writer - Pipe 5 Interrupt Disable"]
pub type PEP_5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_6` writer - Pipe 6 Interrupt Disable"]
pub type PEP_6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_7` writer - Pipe 7 Interrupt Disable"]
pub type PEP_7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_8` writer - Pipe 8 Interrupt Disable"]
pub type PEP_8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEP_9` writer - Pipe 9 Interrupt Disable"]
pub type PEP_9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_1` writer - DMA Channel 0 Interrupt Disable"]
pub type DMA_1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_2` writer - DMA Channel 1 Interrupt Disable"]
pub type DMA_2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_3` writer - DMA Channel 2 Interrupt Disable"]
pub type DMA_3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_4` writer - DMA Channel 3 Interrupt Disable"]
pub type DMA_4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_5` writer - DMA Channel 4 Interrupt Disable"]
pub type DMA_5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_6` writer - DMA Channel 5 Interrupt Disable"]
pub type DMA_6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_7` writer - DMA Channel 6 Interrupt Disable"]
pub type DMA_7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Device Connection Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dconniec(&mut self) -> DCONNIEC_W<HSTIDR_SPEC, 0> {
        DCONNIEC_W::new(self)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ddisciec(&mut self) -> DDISCIEC_W<HSTIDR_SPEC, 1> {
        DDISCIEC_W::new(self)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rstiec(&mut self) -> RSTIEC_W<HSTIDR_SPEC, 2> {
        RSTIEC_W::new(self)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rsmediec(&mut self) -> RSMEDIEC_W<HSTIDR_SPEC, 3> {
        RSMEDIEC_W::new(self)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmiec(&mut self) -> RXRSMIEC_W<HSTIDR_SPEC, 4> {
        RXRSMIEC_W::new(self)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hsofiec(&mut self) -> HSOFIEC_W<HSTIDR_SPEC, 5> {
        HSOFIEC_W::new(self)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn hwupiec(&mut self) -> HWUPIEC_W<HSTIDR_SPEC, 6> {
        HWUPIEC_W::new(self)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_0(&mut self) -> PEP_0_W<HSTIDR_SPEC, 8> {
        PEP_0_W::new(self)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_1(&mut self) -> PEP_1_W<HSTIDR_SPEC, 9> {
        PEP_1_W::new(self)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_2(&mut self) -> PEP_2_W<HSTIDR_SPEC, 10> {
        PEP_2_W::new(self)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_3(&mut self) -> PEP_3_W<HSTIDR_SPEC, 11> {
        PEP_3_W::new(self)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_4(&mut self) -> PEP_4_W<HSTIDR_SPEC, 12> {
        PEP_4_W::new(self)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_5(&mut self) -> PEP_5_W<HSTIDR_SPEC, 13> {
        PEP_5_W::new(self)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_6(&mut self) -> PEP_6_W<HSTIDR_SPEC, 14> {
        PEP_6_W::new(self)
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_7(&mut self) -> PEP_7_W<HSTIDR_SPEC, 15> {
        PEP_7_W::new(self)
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_8(&mut self) -> PEP_8_W<HSTIDR_SPEC, 16> {
        PEP_8_W::new(self)
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pep_9(&mut self) -> PEP_9_W<HSTIDR_SPEC, 17> {
        PEP_9_W::new(self)
    }
    #[doc = "Bit 25 - DMA Channel 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> DMA_1_W<HSTIDR_SPEC, 25> {
        DMA_1_W::new(self)
    }
    #[doc = "Bit 26 - DMA Channel 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> DMA_2_W<HSTIDR_SPEC, 26> {
        DMA_2_W::new(self)
    }
    #[doc = "Bit 27 - DMA Channel 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> DMA_3_W<HSTIDR_SPEC, 27> {
        DMA_3_W::new(self)
    }
    #[doc = "Bit 28 - DMA Channel 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> DMA_4_W<HSTIDR_SPEC, 28> {
        DMA_4_W::new(self)
    }
    #[doc = "Bit 29 - DMA Channel 4 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> DMA_5_W<HSTIDR_SPEC, 29> {
        DMA_5_W::new(self)
    }
    #[doc = "Bit 30 - DMA Channel 5 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> DMA_6_W<HSTIDR_SPEC, 30> {
        DMA_6_W::new(self)
    }
    #[doc = "Bit 31 - DMA Channel 6 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_7(&mut self) -> DMA_7_W<HSTIDR_SPEC, 31> {
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
#[doc = "Host Global Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTIDR_SPEC;
impl crate::RegisterSpec for HSTIDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstidr::W`](W) writer structure"]
impl crate::Writable for HSTIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTIDR to value 0"]
impl crate::Resettable for HSTIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
