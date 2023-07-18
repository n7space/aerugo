#[doc = "Register `SMR` reader"]
pub struct R(crate::R<SMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMR` writer"]
pub struct W(crate::W<SMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCREN` reader - Scrambling/Unscrambling Enable"]
pub type SCREN_R = crate::BitReader<SCRENSELECT_A>;
#[doc = "Scrambling/Unscrambling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCRENSELECT_A {
    #[doc = "0: The scrambling/unscrambling is disabled."]
    DISABLED = 0,
    #[doc = "1: The scrambling/unscrambling is enabled."]
    ENABLED = 1,
}
impl From<SCRENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SCRENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SCREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCRENSELECT_A {
        match self.bits {
            false => SCRENSELECT_A::DISABLED,
            true => SCRENSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCRENSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCRENSELECT_A::ENABLED
    }
}
#[doc = "Field `SCREN` writer - Scrambling/Unscrambling Enable"]
pub type SCREN_W<'a, const O: u8> = crate::BitWriter<'a, SMR_SPEC, O, SCRENSELECT_A>;
impl<'a, const O: u8> SCREN_W<'a, O> {
    #[doc = "The scrambling/unscrambling is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCRENSELECT_A::DISABLED)
    }
    #[doc = "The scrambling/unscrambling is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCRENSELECT_A::ENABLED)
    }
}
#[doc = "Field `RVDIS` reader - Scrambling/Unscrambling Random Value Disable"]
pub type RVDIS_R = crate::BitReader;
#[doc = "Field `RVDIS` writer - Scrambling/Unscrambling Random Value Disable"]
pub type RVDIS_W<'a, const O: u8> = crate::BitWriter<'a, SMR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    pub fn scren(&self) -> SCREN_R {
        SCREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    pub fn rvdis(&self) -> RVDIS_R {
        RVDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scren(&mut self) -> SCREN_W<0> {
        SCREN_W::new(self)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rvdis(&mut self) -> RVDIS_W<1> {
        RVDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scrambling Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smr](index.html) module"]
pub struct SMR_SPEC;
impl crate::RegisterSpec for SMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smr::R](R) reader structure"]
impl crate::Readable for SMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smr::W](W) writer structure"]
impl crate::Writable for SMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMR to value 0"]
impl crate::Resettable for SMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
