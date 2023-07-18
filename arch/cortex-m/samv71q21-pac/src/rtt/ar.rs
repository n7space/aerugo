#[doc = "Register `AR` reader"]
pub struct R(crate::R<AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AR` writer"]
pub struct W(crate::W<AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AR_SPEC>;
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
impl From<crate::W<AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALMV` reader - Alarm Value"]
pub type ALMV_R = crate::FieldReader<u32>;
#[doc = "Field `ALMV` writer - Alarm Value"]
pub type ALMV_W<'a, const O: u8> = crate::FieldWriter<'a, AR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Alarm Value"]
    #[inline(always)]
    pub fn almv(&self) -> ALMV_R {
        ALMV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alarm Value"]
    #[inline(always)]
    #[must_use]
    pub fn almv(&mut self) -> ALMV_W<0> {
        ALMV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ar](index.html) module"]
pub struct AR_SPEC;
impl crate::RegisterSpec for AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ar::R](R) reader structure"]
impl crate::Readable for AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ar::W](W) writer structure"]
impl crate::Writable for AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AR to value 0"]
impl crate::Resettable for AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
