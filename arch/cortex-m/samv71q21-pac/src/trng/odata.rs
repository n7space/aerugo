#[doc = "Register `ODATA` reader"]
pub type R = crate::R<ODATA_SPEC>;
#[doc = "Field `ODATA` reader - Output Data"]
pub type ODATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Output Data"]
    #[inline(always)]
    pub fn odata(&self) -> ODATA_R {
        ODATA_R::new(self.bits)
    }
}
#[doc = "Output Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODATA_SPEC;
impl crate::RegisterSpec for ODATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odata::R`](R) reader structure"]
impl crate::Readable for ODATA_SPEC {}
#[doc = "`reset()` method sets ODATA to value 0"]
impl crate::Resettable for ODATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
