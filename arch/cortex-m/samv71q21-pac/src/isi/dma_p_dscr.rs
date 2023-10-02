#[doc = "Register `DMA_P_DSCR` reader"]
pub type R = crate::R<DMA_P_DSCR_SPEC>;
#[doc = "Register `DMA_P_DSCR` writer"]
pub type W = crate::W<DMA_P_DSCR_SPEC>;
#[doc = "Field `P_DSCR` reader - Preview Descriptor Base Address"]
pub type P_DSCR_R = crate::FieldReader<u32>;
#[doc = "Field `P_DSCR` writer - Preview Descriptor Base Address"]
pub type P_DSCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - Preview Descriptor Base Address"]
    #[inline(always)]
    pub fn p_dscr(&self) -> P_DSCR_R {
        P_DSCR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Preview Descriptor Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn p_dscr(&mut self) -> P_DSCR_W<DMA_P_DSCR_SPEC, 2> {
        P_DSCR_W::new(self)
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
#[doc = "DMA Preview Descriptor Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_p_dscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_p_dscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_P_DSCR_SPEC;
impl crate::RegisterSpec for DMA_P_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_p_dscr::R`](R) reader structure"]
impl crate::Readable for DMA_P_DSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_p_dscr::W`](W) writer structure"]
impl crate::Writable for DMA_P_DSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_P_DSCR to value 0"]
impl crate::Resettable for DMA_P_DSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
