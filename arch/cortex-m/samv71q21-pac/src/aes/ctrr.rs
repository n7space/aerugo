#[doc = "Register `CTRR` reader"]
pub struct R(crate::R<CTRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTR` reader - GCM Encryption Counter"]
pub type CTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GCM Encryption Counter"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(self.bits)
    }
}
#[doc = "GCM Encryption Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrr](index.html) module"]
pub struct CTRR_SPEC;
impl crate::RegisterSpec for CTRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrr::R](R) reader structure"]
impl crate::Readable for CTRR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTRR to value 0"]
impl crate::Resettable for CTRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
