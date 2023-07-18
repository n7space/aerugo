#[doc = "Register `SSR` writer"]
pub struct W(crate::W<SSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSR_SPEC>;
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
impl From<crate::W<SSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXOR` writer - Receive Overrun Status Set"]
pub type RXOR_W<'a, const O: u8> = crate::BitWriter<'a, SSR_SPEC, O>;
#[doc = "Field `TXUR` writer - Transmit Underrun Status Set"]
pub type TXUR_W<'a, const O: u8> = crate::BitWriter<'a, SSR_SPEC, O>;
#[doc = "Field `RXORCH` writer - Receive Overrun Per Channel Status Set"]
pub type RXORCH_W<'a, const O: u8> = crate::FieldWriter<'a, SSR_SPEC, 2, O>;
#[doc = "Field `TXURCH` writer - Transmit Underrun Per Channel Status Set"]
pub type TXURCH_W<'a, const O: u8> = crate::FieldWriter<'a, SSR_SPEC, 2, O>;
impl W {
    #[doc = "Bit 2 - Receive Overrun Status Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxor(&mut self) -> RXOR_W<2> {
        RXOR_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Underrun Status Set"]
    #[inline(always)]
    #[must_use]
    pub fn txur(&mut self) -> TXUR_W<6> {
        TXUR_W::new(self)
    }
    #[doc = "Bits 8:9 - Receive Overrun Per Channel Status Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxorch(&mut self) -> RXORCH_W<8> {
        RXORCH_W::new(self)
    }
    #[doc = "Bits 20:21 - Transmit Underrun Per Channel Status Set"]
    #[inline(always)]
    #[must_use]
    pub fn txurch(&mut self) -> TXURCH_W<20> {
        TXURCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssr](index.html) module"]
pub struct SSR_SPEC;
impl crate::RegisterSpec for SSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ssr::W](W) writer structure"]
impl crate::Writable for SSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSR to value 0"]
impl crate::Resettable for SSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
