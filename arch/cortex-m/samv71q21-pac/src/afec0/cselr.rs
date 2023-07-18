#[doc = "Register `CSELR` reader"]
pub struct R(crate::R<CSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSELR` writer"]
pub struct W(crate::W<CSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSELR_SPEC>;
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
impl From<crate::W<CSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSEL` reader - Channel Selection"]
pub type CSEL_R = crate::FieldReader;
#[doc = "Field `CSEL` writer - Channel Selection"]
pub type CSEL_W<'a, const O: u8> = crate::FieldWriter<'a, CSELR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    pub fn csel(&self) -> CSEL_R {
        CSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel(&mut self) -> CSEL_W<0> {
        CSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cselr](index.html) module"]
pub struct CSELR_SPEC;
impl crate::RegisterSpec for CSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cselr::R](R) reader structure"]
impl crate::Readable for CSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cselr::W](W) writer structure"]
impl crate::Writable for CSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSELR to value 0"]
impl crate::Resettable for CSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
