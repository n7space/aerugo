#[doc = "Register `SCUPUPD` writer"]
pub struct W(crate::W<SCUPUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCUPUPD_SPEC>;
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
impl From<crate::W<SCUPUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCUPUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPRUPD` writer - Update Period Update"]
pub type UPRUPD_W<'a, const O: u8> = crate::FieldWriter<'a, SCUPUPD_SPEC, 4, O>;
impl W {
    #[doc = "Bits 0:3 - Update Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn uprupd(&mut self) -> UPRUPD_W<0> {
        UPRUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Sync Channels Update Period Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scupupd](index.html) module"]
pub struct SCUPUPD_SPEC;
impl crate::RegisterSpec for SCUPUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scupupd::W](W) writer structure"]
impl crate::Writable for SCUPUPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCUPUPD to value 0"]
impl crate::Resettable for SCUPUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
