#[doc = "Register `DMA_P_CTRL` reader"]
pub type R = crate::R<DMA_P_CTRL_SPEC>;
#[doc = "Register `DMA_P_CTRL` writer"]
pub type W = crate::W<DMA_P_CTRL_SPEC>;
#[doc = "Field `P_FETCH` reader - Descriptor Fetch Control Bit"]
pub type P_FETCH_R = crate::BitReader;
#[doc = "Field `P_FETCH` writer - Descriptor Fetch Control Bit"]
pub type P_FETCH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P_WB` reader - Descriptor Writeback Control Bit"]
pub type P_WB_R = crate::BitReader;
#[doc = "Field `P_WB` writer - Descriptor Writeback Control Bit"]
pub type P_WB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P_IEN` reader - Transfer Done Flag Control"]
pub type P_IEN_R = crate::BitReader;
#[doc = "Field `P_IEN` writer - Transfer Done Flag Control"]
pub type P_IEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P_DONE` reader - Preview Transfer Done"]
pub type P_DONE_R = crate::BitReader;
#[doc = "Field `P_DONE` writer - Preview Transfer Done"]
pub type P_DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    pub fn p_fetch(&self) -> P_FETCH_R {
        P_FETCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    pub fn p_wb(&self) -> P_WB_R {
        P_WB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    pub fn p_ien(&self) -> P_IEN_R {
        P_IEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Preview Transfer Done"]
    #[inline(always)]
    pub fn p_done(&self) -> P_DONE_R {
        P_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn p_fetch(&mut self) -> P_FETCH_W<DMA_P_CTRL_SPEC, 0> {
        P_FETCH_W::new(self)
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn p_wb(&mut self) -> P_WB_W<DMA_P_CTRL_SPEC, 1> {
        P_WB_W::new(self)
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    #[must_use]
    pub fn p_ien(&mut self) -> P_IEN_W<DMA_P_CTRL_SPEC, 2> {
        P_IEN_W::new(self)
    }
    #[doc = "Bit 3 - Preview Transfer Done"]
    #[inline(always)]
    #[must_use]
    pub fn p_done(&mut self) -> P_DONE_W<DMA_P_CTRL_SPEC, 3> {
        P_DONE_W::new(self)
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
#[doc = "DMA Preview Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_p_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_p_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_P_CTRL_SPEC;
impl crate::RegisterSpec for DMA_P_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_p_ctrl::R`](R) reader structure"]
impl crate::Readable for DMA_P_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_p_ctrl::W`](W) writer structure"]
impl crate::Writable for DMA_P_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_P_CTRL to value 0"]
impl crate::Resettable for DMA_P_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
