#[doc = "Register `COCR` reader"]
pub struct R(crate::R<COCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COCR` writer"]
pub struct W(crate::W<COCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COCR_SPEC>;
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
impl From<crate::W<COCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AOFF` reader - Analog Offset"]
pub type AOFF_R = crate::FieldReader<u16>;
#[doc = "Field `AOFF` writer - Analog Offset"]
pub type AOFF_W<'a, const O: u8> = crate::FieldWriter<'a, COCR_SPEC, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - Analog Offset"]
    #[inline(always)]
    pub fn aoff(&self) -> AOFF_R {
        AOFF_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Analog Offset"]
    #[inline(always)]
    #[must_use]
    pub fn aoff(&mut self) -> AOFF_W<0> {
        AOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Offset Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cocr](index.html) module"]
pub struct COCR_SPEC;
impl crate::RegisterSpec for COCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cocr::R](R) reader structure"]
impl crate::Readable for COCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cocr::W](W) writer structure"]
impl crate::Writable for COCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COCR to value 0"]
impl crate::Resettable for COCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
