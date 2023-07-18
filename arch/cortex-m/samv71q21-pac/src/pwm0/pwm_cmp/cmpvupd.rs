#[doc = "Register `CMPVUPD` writer"]
pub struct W(crate::W<CMPVUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPVUPD_SPEC>;
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
impl From<crate::W<CMPVUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPVUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVUPD` writer - Comparison x Value Update"]
pub type CVUPD_W<'a, const O: u8> = crate::FieldWriter<'a, CMPVUPD_SPEC, 24, O, u32>;
#[doc = "Field `CVMUPD` writer - Comparison x Value Mode Update"]
pub type CVMUPD_W<'a, const O: u8> = crate::BitWriter<'a, CMPVUPD_SPEC, O>;
impl W {
    #[doc = "Bits 0:23 - Comparison x Value Update"]
    #[inline(always)]
    #[must_use]
    pub fn cvupd(&mut self) -> CVUPD_W<0> {
        CVUPD_W::new(self)
    }
    #[doc = "Bit 24 - Comparison x Value Mode Update"]
    #[inline(always)]
    #[must_use]
    pub fn cvmupd(&mut self) -> CVMUPD_W<24> {
        CVMUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 0 Value Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpvupd](index.html) module"]
pub struct CMPVUPD_SPEC;
impl crate::RegisterSpec for CMPVUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmpvupd::W](W) writer structure"]
impl crate::Writable for CMPVUPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPVUPD to value 0"]
impl crate::Resettable for CMPVUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
