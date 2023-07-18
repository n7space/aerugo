#[doc = "Register `DMA_CHER` writer"]
pub struct W(crate::W<DMA_CHER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHER_SPEC>;
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
impl From<crate::W<DMA_CHER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P_CH_EN` writer - Preview Channel Enable"]
pub type P_CH_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CHER_SPEC, O>;
#[doc = "Field `C_CH_EN` writer - Codec Channel Enable"]
pub type C_CH_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CHER_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Preview Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p_ch_en(&mut self) -> P_CH_EN_W<0> {
        P_CH_EN_W::new(self)
    }
    #[doc = "Bit 1 - Codec Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn c_ch_en(&mut self) -> C_CH_EN_W<1> {
        C_CH_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_cher](index.html) module"]
pub struct DMA_CHER_SPEC;
impl crate::RegisterSpec for DMA_CHER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_cher::W](W) writer structure"]
impl crate::Writable for DMA_CHER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_CHER to value 0"]
impl crate::Resettable for DMA_CHER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
