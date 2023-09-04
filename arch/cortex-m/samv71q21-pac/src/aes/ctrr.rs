#[doc = "Register `CTRR` reader"]
pub type R = crate::R<CTRR_SPEC>;
#[doc = "Field `CTR` reader - GCM Encryption Counter"]
pub type CTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GCM Encryption Counter"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(self.bits)
    }
}
#[doc = "GCM Encryption Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRR_SPEC;
impl crate::RegisterSpec for CTRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrr::R`](R) reader structure"]
impl crate::Readable for CTRR_SPEC {}
#[doc = "`reset()` method sets CTRR to value 0"]
impl crate::Resettable for CTRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
