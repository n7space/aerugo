#[doc = "Register `EXID` reader"]
pub type R = crate::R<EXID_SPEC>;
#[doc = "Field `EXID` reader - Chip ID Extension"]
pub type EXID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Chip ID Extension"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new(self.bits)
    }
}
#[doc = "Chip ID Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXID_SPEC;
impl crate::RegisterSpec for EXID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exid::R`](R) reader structure"]
impl crate::Readable for EXID_SPEC {}
#[doc = "`reset()` method sets EXID to value 0"]
impl crate::Resettable for EXID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
