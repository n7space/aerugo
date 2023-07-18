#[doc = "Register `AADLENR` reader"]
pub struct R(crate::R<AADLENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AADLENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AADLENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AADLENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AADLENR` writer"]
pub struct W(crate::W<AADLENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AADLENR_SPEC>;
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
impl From<crate::W<AADLENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AADLENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AADLEN` reader - Additional Authenticated Data Length"]
pub type AADLEN_R = crate::FieldReader<u32>;
#[doc = "Field `AADLEN` writer - Additional Authenticated Data Length"]
pub type AADLEN_W<'a, const O: u8> = crate::FieldWriter<'a, AADLENR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Additional Authenticated Data Length"]
    #[inline(always)]
    pub fn aadlen(&self) -> AADLEN_R {
        AADLEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional Authenticated Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn aadlen(&mut self) -> AADLEN_W<0> {
        AADLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Additional Authenticated Data Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aadlenr](index.html) module"]
pub struct AADLENR_SPEC;
impl crate::RegisterSpec for AADLENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aadlenr::R](R) reader structure"]
impl crate::Readable for AADLENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aadlenr::W](W) writer structure"]
impl crate::Writable for AADLENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AADLENR to value 0"]
impl crate::Resettable for AADLENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
