#[doc = "Register `IADR` reader"]
pub struct R(crate::R<IADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IADR` writer"]
pub struct W(crate::W<IADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IADR_SPEC>;
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
impl From<crate::W<IADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IADR` reader - Internal Address"]
pub type IADR_R = crate::FieldReader<u32>;
#[doc = "Field `IADR` writer - Internal Address"]
pub type IADR_W<'a, const O: u8> = crate::FieldWriter<'a, IADR_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Internal Address"]
    #[inline(always)]
    pub fn iadr(&self) -> IADR_R {
        IADR_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Internal Address"]
    #[inline(always)]
    #[must_use]
    pub fn iadr(&mut self) -> IADR_W<0> {
        IADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iadr](index.html) module"]
pub struct IADR_SPEC;
impl crate::RegisterSpec for IADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iadr::R](R) reader structure"]
impl crate::Readable for IADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iadr::W](W) writer structure"]
impl crate::Writable for IADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IADR to value 0"]
impl crate::Resettable for IADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
