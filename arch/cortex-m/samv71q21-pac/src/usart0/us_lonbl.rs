#[doc = "Register `US_LONBL` reader"]
pub type R = crate::R<US_LONBL_SPEC>;
#[doc = "Field `LONBL` reader - LON Node Backlog Value"]
pub type LONBL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - LON Node Backlog Value"]
    #[inline(always)]
    pub fn lonbl(&self) -> LONBL_R {
        LONBL_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "LON Backlog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_lonbl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_LONBL_SPEC;
impl crate::RegisterSpec for US_LONBL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_lonbl::R`](R) reader structure"]
impl crate::Readable for US_LONBL_SPEC {}
#[doc = "`reset()` method sets US_LONBL to value 0"]
impl crate::Resettable for US_LONBL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
