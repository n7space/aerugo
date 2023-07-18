#[doc = "Register `FPV1` reader"]
pub struct R(crate::R<FPV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPV1` writer"]
pub struct W(crate::W<FPV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPV1_SPEC>;
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
impl From<crate::W<FPV1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPVH0` reader - Fault Protection Value for PWMH output on channel 0"]
pub type FPVH0_R = crate::BitReader;
#[doc = "Field `FPVH0` writer - Fault Protection Value for PWMH output on channel 0"]
pub type FPVH0_W<'a, const O: u8> = crate::BitWriter<'a, FPV1_SPEC, O>;
#[doc = "Field `FPVH1` reader - Fault Protection Value for PWMH output on channel 1"]
pub type FPVH1_R = crate::BitReader;
#[doc = "Field `FPVH1` writer - Fault Protection Value for PWMH output on channel 1"]
pub type FPVH1_W<'a, const O: u8> = crate::BitWriter<'a, FPV1_SPEC, O>;
#[doc = "Field `FPVH2` reader - Fault Protection Value for PWMH output on channel 2"]
pub type FPVH2_R = crate::BitReader;
#[doc = "Field `FPVH2` writer - Fault Protection Value for PWMH output on channel 2"]
pub type FPVH2_W<'a, const O: u8> = crate::BitWriter<'a, FPV1_SPEC, O>;
#[doc = "Field `FPVH3` reader - Fault Protection Value for PWMH output on channel 3"]
pub type FPVH3_R = crate::BitReader;
#[doc = "Field `FPVH3` writer - Fault Protection Value for PWMH output on channel 3"]
pub type FPVH3_W<'a, const O: u8> = crate::BitWriter<'a, FPV1_SPEC, O>;
#[doc = "Field `FPVL0` reader - Fault Protection Value for PWML output on channel 0"]
pub type FPVL0_R = crate::BitReader;
#[doc = "Field `FPVL0` writer - Fault Protection Value for PWML output on channel 0"]
pub type FPVL0_W<'a, const O: u8> = crate::BitWriter<'a, FPV1_SPEC, O>;
#[doc = "Field `FPVL1` reader - Fault Protection Value for PWML output on channel 1"]
pub type FPVL1_R = crate::BitReader;
#[doc = "Field `FPVL1` writer - Fault Protection Value for PWML output on channel 1"]
pub type FPVL1_W<'a, const O: u8> = crate::BitWriter<'a, FPV1_SPEC, O>;
#[doc = "Field `FPVL2` reader - Fault Protection Value for PWML output on channel 2"]
pub type FPVL2_R = crate::BitReader;
#[doc = "Field `FPVL2` writer - Fault Protection Value for PWML output on channel 2"]
pub type FPVL2_W<'a, const O: u8> = crate::BitWriter<'a, FPV1_SPEC, O>;
#[doc = "Field `FPVL3` reader - Fault Protection Value for PWML output on channel 3"]
pub type FPVL3_R = crate::BitReader;
#[doc = "Field `FPVL3` writer - Fault Protection Value for PWML output on channel 3"]
pub type FPVL3_W<'a, const O: u8> = crate::BitWriter<'a, FPV1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpvh0(&self) -> FPVH0_R {
        FPVH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpvh1(&self) -> FPVH1_R {
        FPVH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpvh2(&self) -> FPVH2_R {
        FPVH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpvh3(&self) -> FPVH3_R {
        FPVH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpvl0(&self) -> FPVL0_R {
        FPVL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpvl1(&self) -> FPVL1_R {
        FPVL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpvl2(&self) -> FPVL2_R {
        FPVL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpvl3(&self) -> FPVL3_R {
        FPVL3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh0(&mut self) -> FPVH0_W<0> {
        FPVH0_W::new(self)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh1(&mut self) -> FPVH1_W<1> {
        FPVH1_W::new(self)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh2(&mut self) -> FPVH2_W<2> {
        FPVH2_W::new(self)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpvh3(&mut self) -> FPVH3_W<3> {
        FPVH3_W::new(self)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl0(&mut self) -> FPVL0_W<16> {
        FPVL0_W::new(self)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl1(&mut self) -> FPVL1_W<17> {
        FPVL1_W::new(self)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl2(&mut self) -> FPVL2_W<18> {
        FPVL2_W::new(self)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpvl3(&mut self) -> FPVL3_W<19> {
        FPVL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Protection Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpv1](index.html) module"]
pub struct FPV1_SPEC;
impl crate::RegisterSpec for FPV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpv1::R](R) reader structure"]
impl crate::Readable for FPV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpv1::W](W) writer structure"]
impl crate::Writable for FPV1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPV1 to value 0"]
impl crate::Resettable for FPV1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
