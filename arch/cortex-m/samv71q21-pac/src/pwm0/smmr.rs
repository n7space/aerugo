#[doc = "Register `SMMR` reader"]
pub struct R(crate::R<SMMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMMR` writer"]
pub struct W(crate::W<SMMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMMR_SPEC>;
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
impl From<crate::W<SMMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCEN0` reader - Gray Count ENable"]
pub type GCEN0_R = crate::BitReader;
#[doc = "Field `GCEN0` writer - Gray Count ENable"]
pub type GCEN0_W<'a, const O: u8> = crate::BitWriter<'a, SMMR_SPEC, O>;
#[doc = "Field `GCEN1` reader - Gray Count ENable"]
pub type GCEN1_R = crate::BitReader;
#[doc = "Field `GCEN1` writer - Gray Count ENable"]
pub type GCEN1_W<'a, const O: u8> = crate::BitWriter<'a, SMMR_SPEC, O>;
#[doc = "Field `DOWN0` reader - DOWN Count"]
pub type DOWN0_R = crate::BitReader;
#[doc = "Field `DOWN0` writer - DOWN Count"]
pub type DOWN0_W<'a, const O: u8> = crate::BitWriter<'a, SMMR_SPEC, O>;
#[doc = "Field `DOWN1` reader - DOWN Count"]
pub type DOWN1_R = crate::BitReader;
#[doc = "Field `DOWN1` writer - DOWN Count"]
pub type DOWN1_W<'a, const O: u8> = crate::BitWriter<'a, SMMR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen0(&self) -> GCEN0_R {
        GCEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen1(&self) -> GCEN1_R {
        GCEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - DOWN Count"]
    #[inline(always)]
    pub fn down0(&self) -> DOWN0_R {
        DOWN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DOWN Count"]
    #[inline(always)]
    pub fn down1(&self) -> DOWN1_R {
        DOWN1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gray Count ENable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen0(&mut self) -> GCEN0_W<0> {
        GCEN0_W::new(self)
    }
    #[doc = "Bit 1 - Gray Count ENable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen1(&mut self) -> GCEN1_W<1> {
        GCEN1_W::new(self)
    }
    #[doc = "Bit 16 - DOWN Count"]
    #[inline(always)]
    #[must_use]
    pub fn down0(&mut self) -> DOWN0_W<16> {
        DOWN0_W::new(self)
    }
    #[doc = "Bit 17 - DOWN Count"]
    #[inline(always)]
    #[must_use]
    pub fn down1(&mut self) -> DOWN1_W<17> {
        DOWN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Stepper Motor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smmr](index.html) module"]
pub struct SMMR_SPEC;
impl crate::RegisterSpec for SMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smmr::R](R) reader structure"]
impl crate::Readable for SMMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smmr::W](W) writer structure"]
impl crate::Writable for SMMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMMR to value 0"]
impl crate::Resettable for SMMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
