#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATRDY` reader - Data Ready Interrupt Mask"]
pub type DATRDY_R = crate::BitReader;
#[doc = "Field `URAD` reader - Unspecified Register Access Detection Interrupt Mask"]
pub type URAD_R = crate::BitReader;
#[doc = "Field `TAGRDY` reader - GCM Tag Ready Interrupt Mask"]
pub type TAGRDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn datrdy(&self) -> DATRDY_R {
        DATRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Interrupt Mask"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - GCM Tag Ready Interrupt Mask"]
    #[inline(always)]
    pub fn tagrdy(&self) -> TAGRDY_R {
        TAGRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
