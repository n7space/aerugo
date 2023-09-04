#[doc = "Register `RHR` reader"]
pub type R = crate::R<RHR_SPEC>;
#[doc = "Field `RXCHR` reader - Received Character"]
pub type RXCHR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Received Character"]
    #[inline(always)]
    pub fn rxchr(&self) -> RXCHR_R {
        RXCHR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RHR_SPEC;
impl crate::RegisterSpec for RHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rhr::R`](R) reader structure"]
impl crate::Readable for RHR_SPEC {}
#[doc = "`reset()` method sets RHR to value 0"]
impl crate::Resettable for RHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
