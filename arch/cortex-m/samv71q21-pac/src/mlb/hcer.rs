#[doc = "Register `HCER[%s]` reader"]
pub struct R(crate::R<HCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CERR` reader - Bitwise Channel Error Bit \\[31:0\\]"]
pub type CERR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Error Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn cerr(&self) -> CERR_R {
        CERR_R::new(self.bits)
    }
}
#[doc = "HBI Channel Error 0 Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcer](index.html) module"]
pub struct HCER_SPEC;
impl crate::RegisterSpec for HCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcer::R](R) reader structure"]
impl crate::Readable for HCER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCER[%s]
to value 0"]
impl crate::Resettable for HCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
