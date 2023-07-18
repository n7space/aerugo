#[doc = "Register `TAGR[%s]` reader"]
pub struct R(crate::R<TAGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAG` reader - GCM Authentication Tag x"]
pub type TAG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GCM Authentication Tag x"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(self.bits)
    }
}
#[doc = "GCM Authentication Tag Word Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tagr](index.html) module"]
pub struct TAGR_SPEC;
impl crate::RegisterSpec for TAGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tagr::R](R) reader structure"]
impl crate::Readable for TAGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAGR[%s]
to value 0"]
impl crate::Resettable for TAGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
