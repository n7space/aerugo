#[doc = "Register `FSR` reader"]
pub struct R(crate::R<FSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIV` reader - Fault Input Value"]
pub type FIV_R = crate::FieldReader;
#[doc = "Field `FS` reader - Fault Status"]
pub type FS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Fault Input Value"]
    #[inline(always)]
    pub fn fiv(&self) -> FIV_R {
        FIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Status"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "PWM Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsr](index.html) module"]
pub struct FSR_SPEC;
impl crate::RegisterSpec for FSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsr::R](R) reader structure"]
impl crate::Readable for FSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSR to value 0"]
impl crate::Resettable for FSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
