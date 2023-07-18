#[doc = "Register `DMA_CHSR` reader"]
pub struct R(crate::R<DMA_CHSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHSR_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "DMA Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chsr](index.html) module"]
pub struct DMA_CHSR_SPEC;
impl crate::RegisterSpec for DMA_CHSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chsr::R](R) reader structure"]
impl crate::Readable for DMA_CHSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_CHSR to value 0"]
impl crate::Resettable for DMA_CHSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
