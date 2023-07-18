#[doc = "Register `RAB` reader"]
pub struct R(crate::R<RAB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAB` reader - Register A or Register B"]
pub type RAB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Register A or Register B"]
    #[inline(always)]
    pub fn rab(&self) -> RAB_R {
        RAB_R::new(self.bits)
    }
}
#[doc = "Register AB (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rab](index.html) module"]
pub struct RAB_SPEC;
impl crate::RegisterSpec for RAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rab::R](R) reader structure"]
impl crate::Readable for RAB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAB to value 0"]
impl crate::Resettable for RAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
