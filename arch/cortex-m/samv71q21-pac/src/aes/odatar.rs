#[doc = "Register `ODATAR[%s]` reader"]
pub type R = crate::R<ODATAR_SPEC>;
#[doc = "Field `ODATA` reader - Output Data"]
pub type ODATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Output Data"]
    #[inline(always)]
    pub fn odata(&self) -> ODATA_R {
        ODATA_R::new(self.bits)
    }
}
#[doc = "Output Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odatar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODATAR_SPEC;
impl crate::RegisterSpec for ODATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odatar::R`](R) reader structure"]
impl crate::Readable for ODATAR_SPEC {}
#[doc = "`reset()` method sets ODATAR[%s]
to value 0"]
impl crate::Resettable for ODATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
