#[doc = "Register `CPRDUPD` writer"]
pub struct W(crate::W<CPRDUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPRDUPD_SPEC>;
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
impl From<crate::W<CPRDUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPRDUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPRDUPD` writer - Channel Period Update"]
pub type CPRDUPD_W<'a, const O: u8> = crate::FieldWriter<'a, CPRDUPD_SPEC, 24, O, u32>;
impl W {
    #[doc = "Bits 0:23 - Channel Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn cprdupd(&mut self) -> CPRDUPD_W<0> {
        CPRDUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Period Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprdupd](index.html) module"]
pub struct CPRDUPD_SPEC;
impl crate::RegisterSpec for CPRDUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cprdupd::W](W) writer structure"]
impl crate::Writable for CPRDUPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPRDUPD to value 0"]
impl crate::Resettable for CPRDUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
