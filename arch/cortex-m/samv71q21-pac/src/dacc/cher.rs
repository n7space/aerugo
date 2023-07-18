#[doc = "Register `CHER` writer"]
pub struct W(crate::W<CHER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHER_SPEC>;
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
impl From<crate::W<CHER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` writer - Channel 0 Enable"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
#[doc = "Field `CH1` writer - Channel 1 Enable"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cher](index.html) module"]
pub struct CHER_SPEC;
impl crate::RegisterSpec for CHER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cher::W](W) writer structure"]
impl crate::Writable for CHER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHER to value 0"]
impl crate::Resettable for CHER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
