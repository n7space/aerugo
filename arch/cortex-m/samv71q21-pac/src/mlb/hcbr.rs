#[doc = "Register `HCBR[%s]` reader"]
pub struct R(crate::R<HCBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHB` reader - Bitwise Channel Busy Bit \\[31:0\\]"]
pub type CHB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Busy Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn chb(&self) -> CHB_R {
        CHB_R::new(self.bits)
    }
}
#[doc = "HBI Channel Busy 0 Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcbr](index.html) module"]
pub struct HCBR_SPEC;
impl crate::RegisterSpec for HCBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcbr::R](R) reader structure"]
impl crate::Readable for HCBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCBR[%s]
to value 0"]
impl crate::Resettable for HCBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
