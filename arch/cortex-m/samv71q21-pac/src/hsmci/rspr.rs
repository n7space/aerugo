#[doc = "Register `RSPR[%s]` reader"]
pub struct R(crate::R<RSPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSP` reader - Response"]
pub type RSP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Response"]
    #[inline(always)]
    pub fn rsp(&self) -> RSP_R {
        RSP_R::new(self.bits)
    }
}
#[doc = "Response Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rspr](index.html) module"]
pub struct RSPR_SPEC;
impl crate::RegisterSpec for RSPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rspr::R](R) reader structure"]
impl crate::Readable for RSPR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSPR[%s]
to value 0"]
impl crate::Resettable for RSPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
