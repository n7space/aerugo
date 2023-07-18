#[doc = "Register `US_THR` writer"]
pub struct W(crate::W<US_THR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_THR_SPEC>;
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
impl From<crate::W<US_THR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_THR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCHR` writer - Character to be Transmitted"]
pub type TXCHR_W<'a, const O: u8> = crate::FieldWriter<'a, US_THR_SPEC, 9, O, u16>;
#[doc = "Field `TXSYNH` writer - Sync Field to be Transmitted"]
pub type TXSYNH_W<'a, const O: u8> = crate::BitWriter<'a, US_THR_SPEC, O>;
impl W {
    #[doc = "Bits 0:8 - Character to be Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn txchr(&mut self) -> TXCHR_W<0> {
        TXCHR_W::new(self)
    }
    #[doc = "Bit 15 - Sync Field to be Transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn txsynh(&mut self) -> TXSYNH_W<15> {
        TXSYNH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_thr](index.html) module"]
pub struct US_THR_SPEC;
impl crate::RegisterSpec for US_THR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [us_thr::W](W) writer structure"]
impl crate::Writable for US_THR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_THR to value 0"]
impl crate::Resettable for US_THR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
