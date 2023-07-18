#[doc = "Register `CDTYUPD` writer"]
pub struct W(crate::W<CDTYUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDTYUPD_SPEC>;
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
impl From<crate::W<CDTYUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDTYUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDTYUPD` writer - Channel Duty-Cycle Update"]
pub type CDTYUPD_W<'a, const O: u8> = crate::FieldWriter<'a, CDTYUPD_SPEC, 24, O, u32>;
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle Update"]
    #[inline(always)]
    #[must_use]
    pub fn cdtyupd(&mut self) -> CDTYUPD_W<0> {
        CDTYUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Duty Cycle Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdtyupd](index.html) module"]
pub struct CDTYUPD_SPEC;
impl crate::RegisterSpec for CDTYUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cdtyupd::W](W) writer structure"]
impl crate::Writable for CDTYUPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDTYUPD to value 0"]
impl crate::Resettable for CDTYUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
