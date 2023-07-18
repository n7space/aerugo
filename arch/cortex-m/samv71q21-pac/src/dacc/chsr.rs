#[doc = "Register `CHSR` reader"]
pub struct R(crate::R<CHSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0` reader - Channel 0 Status"]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH1` reader - Channel 1 Status"]
pub type CH1_R = crate::BitReader;
#[doc = "Field `DACRDY0` reader - DAC Ready Flag"]
pub type DACRDY0_R = crate::BitReader;
#[doc = "Field `DACRDY1` reader - DAC Ready Flag"]
pub type DACRDY1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Status"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Status"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC Ready Flag"]
    #[inline(always)]
    pub fn dacrdy0(&self) -> DACRDY0_R {
        DACRDY0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAC Ready Flag"]
    #[inline(always)]
    pub fn dacrdy1(&self) -> DACRDY1_R {
        DACRDY1_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chsr](index.html) module"]
pub struct CHSR_SPEC;
impl crate::RegisterSpec for CHSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chsr::R](R) reader structure"]
impl crate::Readable for CHSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHSR to value 0"]
impl crate::Resettable for CHSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
