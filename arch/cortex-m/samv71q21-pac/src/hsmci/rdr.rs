#[doc = "Register `RDR` reader"]
pub type R = crate::R<RDR_SPEC>;
#[doc = "Field `DATA` reader - Data to Read"]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data to Read"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RDR_SPEC {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
