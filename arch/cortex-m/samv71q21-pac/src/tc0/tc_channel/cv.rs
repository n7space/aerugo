#[doc = "Register `CV` reader"]
pub type R = crate::R<CV_SPEC>;
#[doc = "Field `CV` reader - Counter Value"]
pub type CV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Counter Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new(self.bits)
    }
}
#[doc = "Counter Value (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CV_SPEC;
impl crate::RegisterSpec for CV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cv::R`](R) reader structure"]
impl crate::Readable for CV_SPEC {}
#[doc = "`reset()` method sets CV to value 0"]
impl crate::Resettable for CV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
