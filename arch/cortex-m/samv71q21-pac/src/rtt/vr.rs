#[doc = "Register `VR` reader"]
pub type R = crate::R<VR_SPEC>;
#[doc = "Field `CRTV` reader - Current Real-time Value"]
pub type CRTV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current Real-time Value"]
    #[inline(always)]
    pub fn crtv(&self) -> CRTV_R {
        CRTV_R::new(self.bits)
    }
}
#[doc = "Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VR_SPEC;
impl crate::RegisterSpec for VR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vr::R`](R) reader structure"]
impl crate::Readable for VR_SPEC {}
#[doc = "`reset()` method sets VR to value 0"]
impl crate::Resettable for VR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
