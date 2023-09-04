#[doc = "Register `RHR` reader"]
pub type R = crate::R<RHR_SPEC>;
#[doc = "Field `RXDATA` reader - Master or Slave Receive Holding Data"]
pub type RXDATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Master or Slave Receive Holding Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
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
