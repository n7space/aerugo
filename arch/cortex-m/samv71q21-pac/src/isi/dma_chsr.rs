#[doc = "Register `DMA_CHSR` reader"]
pub type R = crate::R<DMA_CHSR_SPEC>;
#[doc = "Field `P_CH_S` reader - Preview DMA Channel Status"]
pub type P_CH_S_R = crate::BitReader;
#[doc = "Field `C_CH_S` reader - Code DMA Channel Status"]
pub type C_CH_S_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Preview DMA Channel Status"]
    #[inline(always)]
    pub fn p_ch_s(&self) -> P_CH_S_R {
        P_CH_S_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Code DMA Channel Status"]
    #[inline(always)]
    pub fn c_ch_s(&self) -> C_CH_S_R {
        C_CH_S_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "DMA Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_chsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CHSR_SPEC;
impl crate::RegisterSpec for DMA_CHSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_chsr::R`](R) reader structure"]
impl crate::Readable for DMA_CHSR_SPEC {}
#[doc = "`reset()` method sets DMA_CHSR to value 0"]
impl crate::Resettable for DMA_CHSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
