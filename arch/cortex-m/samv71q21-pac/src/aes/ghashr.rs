#[doc = "Register `GHASHR[%s]` reader"]
pub struct R(crate::R<GHASHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GHASHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GHASHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GHASHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GHASHR[%s]` writer"]
pub struct W(crate::W<GHASHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GHASHR_SPEC>;
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
impl From<crate::W<GHASHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GHASHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GHASH` reader - Intermediate GCM Hash Word x"]
pub type GHASH_R = crate::FieldReader<u32>;
#[doc = "Field `GHASH` writer - Intermediate GCM Hash Word x"]
pub type GHASH_W<'a, const O: u8> = crate::FieldWriter<'a, GHASHR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Intermediate GCM Hash Word x"]
    #[inline(always)]
    pub fn ghash(&self) -> GHASH_R {
        GHASH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Intermediate GCM Hash Word x"]
    #[inline(always)]
    #[must_use]
    pub fn ghash(&mut self) -> GHASH_W<0> {
        GHASH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GCM Intermediate Hash Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ghashr](index.html) module"]
pub struct GHASHR_SPEC;
impl crate::RegisterSpec for GHASHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ghashr::R](R) reader structure"]
impl crate::Readable for GHASHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ghashr::W](W) writer structure"]
impl crate::Writable for GHASHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GHASHR[%s]
to value 0"]
impl crate::Resettable for GHASHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
