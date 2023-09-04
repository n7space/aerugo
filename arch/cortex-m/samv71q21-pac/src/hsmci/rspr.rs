#[doc = "Register `RSPR[%s]` reader"]
pub type R = crate::R<RSPR_SPEC>;
#[doc = "Field `RSP` reader - Response"]
pub type RSP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Response"]
    #[inline(always)]
    pub fn rsp(&self) -> RSP_R {
        RSP_R::new(self.bits)
    }
}
#[doc = "Response Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSPR_SPEC;
impl crate::RegisterSpec for RSPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rspr::R`](R) reader structure"]
impl crate::Readable for RSPR_SPEC {}
#[doc = "`reset()` method sets RSPR[%s]
to value 0"]
impl crate::Resettable for RSPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
