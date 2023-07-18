#[doc = "Register `RXLPITIME` reader"]
pub struct R(crate::R<RXLPITIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXLPITIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXLPITIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXLPITIME_SPEC>) -> Self {
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
#[doc = "Received LPI Time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxlpitime](index.html) module"]
pub struct RXLPITIME_SPEC;
impl crate::RegisterSpec for RXLPITIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxlpitime::R](R) reader structure"]
impl crate::Readable for RXLPITIME_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXLPITIME to value 0"]
impl crate::Resettable for RXLPITIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
