#[doc = "Register `HSTISR` reader"]
pub type R = crate::R<HSTISR_SPEC>;
#[doc = "Field `DCONNI` reader - Device Connection Interrupt"]
pub type DCONNI_R = crate::BitReader;
#[doc = "Field `DDISCI` reader - Device Disconnection Interrupt"]
pub type DDISCI_R = crate::BitReader;
#[doc = "Field `RSTI` reader - USB Reset Sent Interrupt"]
pub type RSTI_R = crate::BitReader;
#[doc = "Field `RSMEDI` reader - Downstream Resume Sent Interrupt"]
pub type RSMEDI_R = crate::BitReader;
#[doc = "Field `RXRSMI` reader - Upstream Resume Received Interrupt"]
pub type RXRSMI_R = crate::BitReader;
#[doc = "Field `HSOFI` reader - Host Start of Frame Interrupt"]
pub type HSOFI_R = crate::BitReader;
#[doc = "Field `HWUPI` reader - Host Wake-Up Interrupt"]
pub type HWUPI_R = crate::BitReader;
#[doc = "Field `PEP_0` reader - Pipe 0 Interrupt"]
pub type PEP_0_R = crate::BitReader;
#[doc = "Field `PEP_1` reader - Pipe 1 Interrupt"]
pub type PEP_1_R = crate::BitReader;
#[doc = "Field `PEP_2` reader - Pipe 2 Interrupt"]
pub type PEP_2_R = crate::BitReader;
#[doc = "Field `PEP_3` reader - Pipe 3 Interrupt"]
pub type PEP_3_R = crate::BitReader;
#[doc = "Field `PEP_4` reader - Pipe 4 Interrupt"]
pub type PEP_4_R = crate::BitReader;
#[doc = "Field `PEP_5` reader - Pipe 5 Interrupt"]
pub type PEP_5_R = crate::BitReader;
#[doc = "Field `PEP_6` reader - Pipe 6 Interrupt"]
pub type PEP_6_R = crate::BitReader;
#[doc = "Field `PEP_7` reader - Pipe 7 Interrupt"]
pub type PEP_7_R = crate::BitReader;
#[doc = "Field `PEP_8` reader - Pipe 8 Interrupt"]
pub type PEP_8_R = crate::BitReader;
#[doc = "Field `PEP_9` reader - Pipe 9 Interrupt"]
pub type PEP_9_R = crate::BitReader;
#[doc = "Field `DMA_1` reader - DMA Channel 0 Interrupt"]
pub type DMA_1_R = crate::BitReader;
#[doc = "Field `DMA_2` reader - DMA Channel 1 Interrupt"]
pub type DMA_2_R = crate::BitReader;
#[doc = "Field `DMA_3` reader - DMA Channel 2 Interrupt"]
pub type DMA_3_R = crate::BitReader;
#[doc = "Field `DMA_4` reader - DMA Channel 3 Interrupt"]
pub type DMA_4_R = crate::BitReader;
#[doc = "Field `DMA_5` reader - DMA Channel 4 Interrupt"]
pub type DMA_5_R = crate::BitReader;
#[doc = "Field `DMA_6` reader - DMA Channel 5 Interrupt"]
pub type DMA_6_R = crate::BitReader;
#[doc = "Field `DMA_7` reader - DMA Channel 6 Interrupt"]
pub type DMA_7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Device Connection Interrupt"]
    #[inline(always)]
    pub fn dconni(&self) -> DCONNI_R {
        DCONNI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt"]
    #[inline(always)]
    pub fn ddisci(&self) -> DDISCI_R {
        DDISCI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt"]
    #[inline(always)]
    pub fn rsti(&self) -> RSTI_R {
        RSTI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt"]
    #[inline(always)]
    pub fn rsmedi(&self) -> RSMEDI_R {
        RSMEDI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt"]
    #[inline(always)]
    pub fn rxrsmi(&self) -> RXRSMI_R {
        RXRSMI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt"]
    #[inline(always)]
    pub fn hsofi(&self) -> HSOFI_R {
        HSOFI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt"]
    #[inline(always)]
    pub fn hwupi(&self) -> HWUPI_R {
        HWUPI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt"]
    #[inline(always)]
    pub fn pep_0(&self) -> PEP_0_R {
        PEP_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt"]
    #[inline(always)]
    pub fn pep_1(&self) -> PEP_1_R {
        PEP_1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt"]
    #[inline(always)]
    pub fn pep_2(&self) -> PEP_2_R {
        PEP_2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt"]
    #[inline(always)]
    pub fn pep_3(&self) -> PEP_3_R {
        PEP_3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt"]
    #[inline(always)]
    pub fn pep_4(&self) -> PEP_4_R {
        PEP_4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt"]
    #[inline(always)]
    pub fn pep_5(&self) -> PEP_5_R {
        PEP_5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt"]
    #[inline(always)]
    pub fn pep_6(&self) -> PEP_6_R {
        PEP_6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt"]
    #[inline(always)]
    pub fn pep_7(&self) -> PEP_7_R {
        PEP_7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt"]
    #[inline(always)]
    pub fn pep_8(&self) -> PEP_8_R {
        PEP_8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt"]
    #[inline(always)]
    pub fn pep_9(&self) -> PEP_9_R {
        PEP_9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 0 Interrupt"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 1 Interrupt"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 2 Interrupt"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 3 Interrupt"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 4 Interrupt"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 5 Interrupt"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Channel 6 Interrupt"]
    #[inline(always)]
    pub fn dma_7(&self) -> DMA_7_R {
        DMA_7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Host Global Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTISR_SPEC;
impl crate::RegisterSpec for HSTISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstisr::R`](R) reader structure"]
impl crate::Readable for HSTISR_SPEC {}
#[doc = "`reset()` method sets HSTISR to value 0"]
impl crate::Resettable for HSTISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
