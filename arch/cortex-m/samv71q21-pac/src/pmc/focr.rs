#[doc = "Register `FOCR` writer"]
pub struct W(crate::W<FOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FOCR_SPEC>;
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
impl From<crate::W<FOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FOCLR` writer - Fault Output Clear"]
pub type FOCLR_W<'a, const O: u8> = crate::BitWriter<'a, FOCR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Fault Output Clear"]
    #[inline(always)]
    #[must_use]
    pub fn foclr(&mut self) -> FOCLR_W<0> {
        FOCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Output Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [focr](index.html) module"]
pub struct FOCR_SPEC;
impl crate::RegisterSpec for FOCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [focr::W](W) writer structure"]
impl crate::Writable for FOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FOCR to value 0"]
impl crate::Resettable for FOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
