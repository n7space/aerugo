#[doc = "Register `RHR` reader"]
pub struct R(crate::R<RHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXCHR` reader - Received Character"]
pub type RXCHR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Received Character"]
    #[inline(always)]
    pub fn rxchr(&self) -> RXCHR_R {
        RXCHR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rhr](index.html) module"]
pub struct RHR_SPEC;
impl crate::RegisterSpec for RHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rhr::R](R) reader structure"]
impl crate::Readable for RHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RHR to value 0"]
impl crate::Resettable for RHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
