#[doc = "Register `DMA_CHER` writer"]
pub type W = crate::W<DMA_CHER_SPEC>;
#[doc = "Field `P_CH_EN` writer - Preview Channel Enable"]
pub type P_CH_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C_CH_EN` writer - Codec Channel Enable"]
pub type C_CH_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Preview Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p_ch_en(&mut self) -> P_CH_EN_W<DMA_CHER_SPEC, 0> {
        P_CH_EN_W::new(self)
    }
    #[doc = "Bit 1 - Codec Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn c_ch_en(&mut self) -> C_CH_EN_W<DMA_CHER_SPEC, 1> {
        C_CH_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Channel Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_cher::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CHER_SPEC;
impl crate::RegisterSpec for DMA_CHER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_cher::W`](W) writer structure"]
impl crate::Writable for DMA_CHER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_CHER to value 0"]
impl crate::Resettable for DMA_CHER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
