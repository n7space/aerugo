#[doc = "Register `TAGR[%s]` reader"]
pub type R = crate::R<TAGR_SPEC>;
#[doc = "Field `TAG` reader - GCM Authentication Tag x"]
pub type TAG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GCM Authentication Tag x"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(self.bits)
    }
}
#[doc = "GCM Authentication Tag Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tagr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAGR_SPEC;
impl crate::RegisterSpec for TAGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tagr::R`](R) reader structure"]
impl crate::Readable for TAGR_SPEC {}
#[doc = "`reset()` method sets TAGR[%s]
to value 0"]
impl crate::Resettable for TAGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
