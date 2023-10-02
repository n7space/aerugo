#[doc = "Register `DMA_C_ADDR` reader"]
pub type R = crate::R<DMA_C_ADDR_SPEC>;
#[doc = "Register `DMA_C_ADDR` writer"]
pub type W = crate::W<DMA_C_ADDR_SPEC>;
#[doc = "Field `C_ADDR` reader - Codec Image Base Address"]
pub type C_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `C_ADDR` writer - Codec Image Base Address"]
pub type C_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - Codec Image Base Address"]
    #[inline(always)]
    pub fn c_addr(&self) -> C_ADDR_R {
        C_ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Codec Image Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn c_addr(&mut self) -> C_ADDR_W<DMA_C_ADDR_SPEC, 2> {
        C_ADDR_W::new(self)
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
#[doc = "DMA Codec Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_c_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_c_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_C_ADDR_SPEC;
impl crate::RegisterSpec for DMA_C_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_c_addr::R`](R) reader structure"]
impl crate::Readable for DMA_C_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_c_addr::W`](W) writer structure"]
impl crate::Writable for DMA_C_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_C_ADDR to value 0"]
impl crate::Resettable for DMA_C_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
