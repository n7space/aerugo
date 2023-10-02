#[doc = "Register `DMA_C_CTRL` reader"]
pub type R = crate::R<DMA_C_CTRL_SPEC>;
#[doc = "Register `DMA_C_CTRL` writer"]
pub type W = crate::W<DMA_C_CTRL_SPEC>;
#[doc = "Field `C_FETCH` reader - Descriptor Fetch Control Bit"]
pub type C_FETCH_R = crate::BitReader;
#[doc = "Field `C_FETCH` writer - Descriptor Fetch Control Bit"]
pub type C_FETCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C_WB` reader - Descriptor Writeback Control Bit"]
pub type C_WB_R = crate::BitReader;
#[doc = "Field `C_WB` writer - Descriptor Writeback Control Bit"]
pub type C_WB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C_IEN` reader - Transfer Done Flag Control"]
pub type C_IEN_R = crate::BitReader;
#[doc = "Field `C_IEN` writer - Transfer Done Flag Control"]
pub type C_IEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C_DONE` reader - Codec Transfer Done"]
pub type C_DONE_R = crate::BitReader;
#[doc = "Field `C_DONE` writer - Codec Transfer Done"]
pub type C_DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    pub fn c_fetch(&self) -> C_FETCH_R {
        C_FETCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    pub fn c_wb(&self) -> C_WB_R {
        C_WB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    pub fn c_ien(&self) -> C_IEN_R {
        C_IEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Codec Transfer Done"]
    #[inline(always)]
    pub fn c_done(&self) -> C_DONE_R {
        C_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn c_fetch(&mut self) -> C_FETCH_W<DMA_C_CTRL_SPEC, 0> {
        C_FETCH_W::new(self)
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn c_wb(&mut self) -> C_WB_W<DMA_C_CTRL_SPEC, 1> {
        C_WB_W::new(self)
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    #[must_use]
    pub fn c_ien(&mut self) -> C_IEN_W<DMA_C_CTRL_SPEC, 2> {
        C_IEN_W::new(self)
    }
    #[doc = "Bit 3 - Codec Transfer Done"]
    #[inline(always)]
    #[must_use]
    pub fn c_done(&mut self) -> C_DONE_W<DMA_C_CTRL_SPEC, 3> {
        C_DONE_W::new(self)
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
#[doc = "DMA Codec Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_c_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_c_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_C_CTRL_SPEC;
impl crate::RegisterSpec for DMA_C_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_c_ctrl::R`](R) reader structure"]
impl crate::Readable for DMA_C_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_c_ctrl::W`](W) writer structure"]
impl crate::Writable for DMA_C_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_C_CTRL to value 0"]
impl crate::Resettable for DMA_C_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
