#[doc = "Register `EEFC_FRR` reader"]
pub type R = crate::R<EEFC_FRR_SPEC>;
#[doc = "Field `FVALUE` reader - Flash Result Value"]
pub type FVALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Flash Result Value"]
    #[inline(always)]
    pub fn fvalue(&self) -> FVALUE_R {
        FVALUE_R::new(self.bits)
    }
}
#[doc = "EEFC Flash Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefc_frr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEFC_FRR_SPEC;
impl crate::RegisterSpec for EEFC_FRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eefc_frr::R`](R) reader structure"]
impl crate::Readable for EEFC_FRR_SPEC {}
#[doc = "`reset()` method sets EEFC_FRR to value 0"]
impl crate::Resettable for EEFC_FRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
