#[doc = "Register `DMA_P_CTRL` reader"]
pub struct R(crate::R<DMA_P_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_P_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_P_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_P_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_P_CTRL` writer"]
pub struct W(crate::W<DMA_P_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_P_CTRL_SPEC>;
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
impl From<crate::W<DMA_P_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_P_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P_FETCH` reader - Descriptor Fetch Control Bit"]
pub type P_FETCH_R = crate::BitReader;
#[doc = "Field `P_FETCH` writer - Descriptor Fetch Control Bit"]
pub type P_FETCH_W<'a, const O: u8> = crate::BitWriter<'a, DMA_P_CTRL_SPEC, O>;
#[doc = "Field `P_WB` reader - Descriptor Writeback Control Bit"]
pub type P_WB_R = crate::BitReader;
#[doc = "Field `P_WB` writer - Descriptor Writeback Control Bit"]
pub type P_WB_W<'a, const O: u8> = crate::BitWriter<'a, DMA_P_CTRL_SPEC, O>;
#[doc = "Field `P_IEN` reader - Transfer Done Flag Control"]
pub type P_IEN_R = crate::BitReader;
#[doc = "Field `P_IEN` writer - Transfer Done Flag Control"]
pub type P_IEN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_P_CTRL_SPEC, O>;
#[doc = "Field `P_DONE` reader - Preview Transfer Done"]
pub type P_DONE_R = crate::BitReader;
#[doc = "Field `P_DONE` writer - Preview Transfer Done"]
pub type P_DONE_W<'a, const O: u8> = crate::BitWriter<'a, DMA_P_CTRL_SPEC, O>;
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
    pub fn p_fetch(&mut self) -> P_FETCH_W<0> {
        P_FETCH_W::new(self)
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn p_wb(&mut self) -> P_WB_W<1> {
        P_WB_W::new(self)
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    #[must_use]
    pub fn p_ien(&mut self) -> P_IEN_W<2> {
        P_IEN_W::new(self)
    }
    #[doc = "Bit 3 - Preview Transfer Done"]
    #[inline(always)]
    #[must_use]
    pub fn p_done(&mut self) -> P_DONE_W<3> {
        P_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Preview Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_p_ctrl](index.html) module"]
pub struct DMA_P_CTRL_SPEC;
impl crate::RegisterSpec for DMA_P_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_p_ctrl::R](R) reader structure"]
impl crate::Readable for DMA_P_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_p_ctrl::W](W) writer structure"]
impl crate::Writable for DMA_P_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_P_CTRL to value 0"]
impl crate::Resettable for DMA_P_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
