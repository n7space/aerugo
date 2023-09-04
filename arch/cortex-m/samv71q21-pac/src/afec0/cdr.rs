#[doc = "Register `CDR` reader"]
pub type R = crate::R<CDR_SPEC>;
#[doc = "Field `DATA` reader - Converted Data"]
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "AFEC Channel Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr::R`](R) reader structure"]
impl crate::Readable for CDR_SPEC {}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
