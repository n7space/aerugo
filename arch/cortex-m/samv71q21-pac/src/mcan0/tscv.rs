#[doc = "Register `TSCV` reader"]
pub struct R(crate::R<TSCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSCV` writer"]
pub struct W(crate::W<TSCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSCV_SPEC>;
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
impl From<crate::W<TSCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSC` reader - Timestamp Counter (cleared on write)"]
pub type TSC_R = crate::FieldReader<u16>;
#[doc = "Field `TSC` writer - Timestamp Counter (cleared on write)"]
pub type TSC_W<'a, const O: u8> = crate::FieldWriter<'a, TSCV_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Counter (cleared on write)"]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Counter (cleared on write)"]
    #[inline(always)]
    #[must_use]
    pub fn tsc(&mut self) -> TSC_W<0> {
        TSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscv](index.html) module"]
pub struct TSCV_SPEC;
impl crate::RegisterSpec for TSCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tscv::R](R) reader structure"]
impl crate::Readable for TSCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tscv::W](W) writer structure"]
impl crate::Writable for TSCV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSCV to value 0"]
impl crate::Resettable for TSCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
