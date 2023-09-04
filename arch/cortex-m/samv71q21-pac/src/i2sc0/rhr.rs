#[doc = "Register `RHR` reader"]
pub type R = crate::R<RHR_SPEC>;
#[doc = "Field `RHR` reader - Receiver Holding Register"]
pub type RHR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receiver Holding Register"]
    #[inline(always)]
    pub fn rhr(&self) -> RHR_R {
        RHR_R::new(self.bits)
    }
}
#[doc = "Receiver Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
