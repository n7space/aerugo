#[doc = "Register `ACMR[%s]` reader"]
pub struct R(crate::R<ACMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACMR[%s]` writer"]
pub struct W(crate::W<ACMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMR_SPEC>;
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
impl From<crate::W<ACMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHM` reader - Bitwise Channel Mask Bits 31 to 0"]
pub type CHM_R = crate::FieldReader<u32>;
#[doc = "Field `CHM` writer - Bitwise Channel Mask Bits 31 to 0"]
pub type CHM_W<'a, const O: u8> = crate::FieldWriter<'a, ACMR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bits 31 to 0"]
    #[inline(always)]
    pub fn chm(&self) -> CHM_R {
        CHM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bits 31 to 0"]
    #[inline(always)]
    #[must_use]
    pub fn chm(&mut self) -> CHM_W<0> {
        CHM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Channel Mask 0 Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmr](index.html) module"]
pub struct ACMR_SPEC;
impl crate::RegisterSpec for ACMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acmr::R](R) reader structure"]
impl crate::Readable for ACMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acmr::W](W) writer structure"]
impl crate::Writable for ACMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACMR[%s]
to value 0"]
impl crate::Resettable for ACMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
