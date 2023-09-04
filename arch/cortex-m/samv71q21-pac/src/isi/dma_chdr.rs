#[doc = "Register `DMA_CHDR` writer"]
pub type W = crate::W<DMA_CHDR_SPEC>;
#[doc = "Field `P_CH_DIS` writer - Preview Channel Disable Request"]
pub type P_CH_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C_CH_DIS` writer - Codec Channel Disable Request"]
pub type C_CH_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Preview Channel Disable Request"]
    #[inline(always)]
    #[must_use]
    pub fn p_ch_dis(&mut self) -> P_CH_DIS_W<DMA_CHDR_SPEC, 0> {
        P_CH_DIS_W::new(self)
    }
    #[doc = "Bit 1 - Codec Channel Disable Request"]
    #[inline(always)]
    #[must_use]
    pub fn c_ch_dis(&mut self) -> C_CH_DIS_W<DMA_CHDR_SPEC, 1> {
        C_CH_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Channel Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_chdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CHDR_SPEC;
impl crate::RegisterSpec for DMA_CHDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_chdr::W`](W) writer structure"]
impl crate::Writable for DMA_CHDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_CHDR to value 0"]
impl crate::Resettable for DMA_CHDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
