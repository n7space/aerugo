#[doc = "Register `CBSISQA` reader"]
pub struct R(crate::R<CBSISQA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBSISQA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBSISQA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBSISQA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBSISQA` writer"]
pub struct W(crate::W<CBSISQA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBSISQA_SPEC>;
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
impl From<crate::W<CBSISQA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBSISQA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IS` reader - IdleSlope"]
pub type IS_R = crate::FieldReader<u32>;
#[doc = "Field `IS` writer - IdleSlope"]
pub type IS_W<'a, const O: u8> = crate::FieldWriter<'a, CBSISQA_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - IdleSlope"]
    #[inline(always)]
    pub fn is(&self) -> IS_R {
        IS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IdleSlope"]
    #[inline(always)]
    #[must_use]
    pub fn is(&mut self) -> IS_W<0> {
        IS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Credit-Based Shaping IdleSlope Register for Queue A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbsisqa](index.html) module"]
pub struct CBSISQA_SPEC;
impl crate::RegisterSpec for CBSISQA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cbsisqa::R](R) reader structure"]
impl crate::Readable for CBSISQA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbsisqa::W](W) writer structure"]
impl crate::Writable for CBSISQA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBSISQA to value 0"]
impl crate::Resettable for CBSISQA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
