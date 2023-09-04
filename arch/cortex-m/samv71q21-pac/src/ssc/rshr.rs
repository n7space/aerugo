#[doc = "Register `RSHR` reader"]
pub type R = crate::R<RSHR_SPEC>;
#[doc = "Field `RSDAT` reader - Receive Synchronization Data"]
pub type RSDAT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Synchronization Data"]
    #[inline(always)]
    pub fn rsdat(&self) -> RSDAT_R {
        RSDAT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive Sync. Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rshr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSHR_SPEC;
impl crate::RegisterSpec for RSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rshr::R`](R) reader structure"]
impl crate::Readable for RSHR_SPEC {}
#[doc = "`reset()` method sets RSHR to value 0"]
impl crate::Resettable for RSHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
