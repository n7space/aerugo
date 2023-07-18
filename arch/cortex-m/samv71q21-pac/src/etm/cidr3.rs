#[doc = "Register `CIDR3` reader"]
pub struct R(crate::R<CIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR3_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CIDR3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Component Identification Register #3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr3](index.html) module"]
pub struct CIDR3_SPEC;
impl crate::RegisterSpec for CIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr3::R](R) reader structure"]
impl crate::Readable for CIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIDR3 to value 0xb1"]
impl crate::Resettable for CIDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0xb1;
}