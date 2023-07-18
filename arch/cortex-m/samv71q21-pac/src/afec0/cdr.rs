#[doc = "Register `CDR` reader"]
pub struct R(crate::R<CDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Converted Data"]
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "AFEC Channel Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr](index.html) module"]
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdr::R](R) reader structure"]
impl crate::Readable for CDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
