#[doc = "Register `GCMHR[%s]` reader"]
pub struct R(crate::R<GCMHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCMHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCMHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCMHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCMHR[%s]` writer"]
pub struct W(crate::W<GCMHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCMHR_SPEC>;
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
impl From<crate::W<GCMHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCMHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H` reader - GCM H Word x"]
pub type H_R = crate::FieldReader<u32>;
#[doc = "Field `H` writer - GCM H Word x"]
pub type H_W<'a, const O: u8> = crate::FieldWriter<'a, GCMHR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - GCM H Word x"]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GCM H Word x"]
    #[inline(always)]
    #[must_use]
    pub fn h(&mut self) -> H_W<0> {
        H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GCM H Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcmhr](index.html) module"]
pub struct GCMHR_SPEC;
impl crate::RegisterSpec for GCMHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcmhr::R](R) reader structure"]
impl crate::Readable for GCMHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcmhr::W](W) writer structure"]
impl crate::Writable for GCMHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCMHR[%s]
to value 0"]
impl crate::Resettable for GCMHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
