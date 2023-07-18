#[doc = "Register `CMR` reader"]
pub struct R(crate::R<CMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMR` writer"]
pub struct W(crate::W<CMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMR_SPEC>;
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
impl From<crate::W<CMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Clock Divider"]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Clock Divider"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, CMR_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr](index.html) module"]
pub struct CMR_SPEC;
impl crate::RegisterSpec for CMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmr::R](R) reader structure"]
impl crate::Readable for CMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmr::W](W) writer structure"]
impl crate::Writable for CMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMR to value 0"]
impl crate::Resettable for CMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
