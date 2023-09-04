#[doc = "Register `RAB` reader"]
pub type R = crate::R<RAB_SPEC>;
#[doc = "Field `RAB` reader - Register A or Register B"]
pub type RAB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register A or Register B"]
    #[inline(always)]
    pub fn rab(&self) -> RAB_R {
        RAB_R::new(self.bits)
    }
}
#[doc = "Register AB (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rab::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAB_SPEC;
impl crate::RegisterSpec for RAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rab::R`](R) reader structure"]
impl crate::Readable for RAB_SPEC {}
#[doc = "`reset()` method sets RAB to value 0"]
impl crate::Resettable for RAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
