#[doc = "Register `TXLPITIME` reader"]
pub struct R(crate::R<TXLPITIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXLPITIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXLPITIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXLPITIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPITIME` reader - Time in LPI (cleared on read)"]
pub type LPITIME_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Time in LPI (cleared on read)"]
    #[inline(always)]
    pub fn lpitime(&self) -> LPITIME_R {
        LPITIME_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Transmit LPI Time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txlpitime](index.html) module"]
pub struct TXLPITIME_SPEC;
impl crate::RegisterSpec for TXLPITIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txlpitime::R](R) reader structure"]
impl crate::Readable for TXLPITIME_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXLPITIME to value 0"]
impl crate::Resettable for TXLPITIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
