#[doc = "Register `PCISR` reader"]
pub struct R(crate::R<PCISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DRDY` reader - Parallel Capture Mode Data Ready"]
pub type DRDY_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Parallel Capture Mode Overrun Error"]
pub type OVRE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Parallel Capture Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcisr](index.html) module"]
pub struct PCISR_SPEC;
impl crate::RegisterSpec for PCISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcisr::R](R) reader structure"]
impl crate::Readable for PCISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCISR to value 0"]
impl crate::Resettable for PCISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
