#[doc = "Register `CV` reader"]
pub struct R(crate::R<CV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CV` reader - Counter Value"]
pub type CV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Counter Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new(self.bits)
    }
}
#[doc = "Counter Value (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](index.html) module"]
pub struct CV_SPEC;
impl crate::RegisterSpec for CV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cv::R](R) reader structure"]
impl crate::Readable for CV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CV to value 0"]
impl crate::Resettable for CV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
