#[doc = "Register `EEFC_FRR` reader"]
pub struct R(crate::R<EEFC_FRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFC_FRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFC_FRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFC_FRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FVALUE` reader - Flash Result Value"]
pub type FVALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Flash Result Value"]
    #[inline(always)]
    pub fn fvalue(&self) -> FVALUE_R {
        FVALUE_R::new(self.bits)
    }
}
#[doc = "EEFC Flash Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefc_frr](index.html) module"]
pub struct EEFC_FRR_SPEC;
impl crate::RegisterSpec for EEFC_FRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eefc_frr::R](R) reader structure"]
impl crate::Readable for EEFC_FRR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EEFC_FRR to value 0"]
impl crate::Resettable for EEFC_FRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
