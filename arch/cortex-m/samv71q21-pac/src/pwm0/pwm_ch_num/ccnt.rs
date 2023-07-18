#[doc = "Register `CCNT` reader"]
pub struct R(crate::R<CCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - Channel Counter Register"]
pub type CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PWM Channel Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccnt](index.html) module"]
pub struct CCNT_SPEC;
impl crate::RegisterSpec for CCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccnt::R](R) reader structure"]
impl crate::Readable for CCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCNT to value 0"]
impl crate::Resettable for CCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
