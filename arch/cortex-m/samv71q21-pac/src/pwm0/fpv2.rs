#[doc = "Register `FPV2` reader"]
pub struct R(crate::R<FPV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPV2` writer"]
pub struct W(crate::W<FPV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPV2_SPEC>;
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
impl From<crate::W<FPV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPZH0` reader - Fault Protection to Hi-Z for PWMH output on channel 0"]
pub type FPZH0_R = crate::BitReader;
#[doc = "Field `FPZH0` writer - Fault Protection to Hi-Z for PWMH output on channel 0"]
pub type FPZH0_W<'a, const O: u8> = crate::BitWriter<'a, FPV2_SPEC, O>;
#[doc = "Field `FPZH1` reader - Fault Protection to Hi-Z for PWMH output on channel 1"]
pub type FPZH1_R = crate::BitReader;
#[doc = "Field `FPZH1` writer - Fault Protection to Hi-Z for PWMH output on channel 1"]
pub type FPZH1_W<'a, const O: u8> = crate::BitWriter<'a, FPV2_SPEC, O>;
#[doc = "Field `FPZH2` reader - Fault Protection to Hi-Z for PWMH output on channel 2"]
pub type FPZH2_R = crate::BitReader;
#[doc = "Field `FPZH2` writer - Fault Protection to Hi-Z for PWMH output on channel 2"]
pub type FPZH2_W<'a, const O: u8> = crate::BitWriter<'a, FPV2_SPEC, O>;
#[doc = "Field `FPZH3` reader - Fault Protection to Hi-Z for PWMH output on channel 3"]
pub type FPZH3_R = crate::BitReader;
#[doc = "Field `FPZH3` writer - Fault Protection to Hi-Z for PWMH output on channel 3"]
pub type FPZH3_W<'a, const O: u8> = crate::BitWriter<'a, FPV2_SPEC, O>;
#[doc = "Field `FPZL0` reader - Fault Protection to Hi-Z for PWML output on channel 0"]
pub type FPZL0_R = crate::BitReader;
#[doc = "Field `FPZL0` writer - Fault Protection to Hi-Z for PWML output on channel 0"]
pub type FPZL0_W<'a, const O: u8> = crate::BitWriter<'a, FPV2_SPEC, O>;
#[doc = "Field `FPZL1` reader - Fault Protection to Hi-Z for PWML output on channel 1"]
pub type FPZL1_R = crate::BitReader;
#[doc = "Field `FPZL1` writer - Fault Protection to Hi-Z for PWML output on channel 1"]
pub type FPZL1_W<'a, const O: u8> = crate::BitWriter<'a, FPV2_SPEC, O>;
#[doc = "Field `FPZL2` reader - Fault Protection to Hi-Z for PWML output on channel 2"]
pub type FPZL2_R = crate::BitReader;
#[doc = "Field `FPZL2` writer - Fault Protection to Hi-Z for PWML output on channel 2"]
pub type FPZL2_W<'a, const O: u8> = crate::BitWriter<'a, FPV2_SPEC, O>;
#[doc = "Field `FPZL3` reader - Fault Protection to Hi-Z for PWML output on channel 3"]
pub type FPZL3_R = crate::BitReader;
#[doc = "Field `FPZL3` writer - Fault Protection to Hi-Z for PWML output on channel 3"]
pub type FPZL3_W<'a, const O: u8> = crate::BitWriter<'a, FPV2_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Fault Protection to Hi-Z for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpzh0(&self) -> FPZH0_R {
        FPZH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Protection to Hi-Z for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpzh1(&self) -> FPZH1_R {
        FPZH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Protection to Hi-Z for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpzh2(&self) -> FPZH2_R {
        FPZH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Protection to Hi-Z for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpzh3(&self) -> FPZH3_R {
        FPZH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Fault Protection to Hi-Z for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpzl0(&self) -> FPZL0_R {
        FPZL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fault Protection to Hi-Z for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpzl1(&self) -> FPZL1_R {
        FPZL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault Protection to Hi-Z for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpzl2(&self) -> FPZL2_R {
        FPZL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fault Protection to Hi-Z for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpzl3(&self) -> FPZL3_R {
        FPZL3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Protection to Hi-Z for PWMH output on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpzh0(&mut self) -> FPZH0_W<0> {
        FPZH0_W::new(self)
    }
    #[doc = "Bit 1 - Fault Protection to Hi-Z for PWMH output on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpzh1(&mut self) -> FPZH1_W<1> {
        FPZH1_W::new(self)
    }
    #[doc = "Bit 2 - Fault Protection to Hi-Z for PWMH output on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpzh2(&mut self) -> FPZH2_W<2> {
        FPZH2_W::new(self)
    }
    #[doc = "Bit 3 - Fault Protection to Hi-Z for PWMH output on channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpzh3(&mut self) -> FPZH3_W<3> {
        FPZH3_W::new(self)
    }
    #[doc = "Bit 16 - Fault Protection to Hi-Z for PWML output on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpzl0(&mut self) -> FPZL0_W<16> {
        FPZL0_W::new(self)
    }
    #[doc = "Bit 17 - Fault Protection to Hi-Z for PWML output on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpzl1(&mut self) -> FPZL1_W<17> {
        FPZL1_W::new(self)
    }
    #[doc = "Bit 18 - Fault Protection to Hi-Z for PWML output on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpzl2(&mut self) -> FPZL2_W<18> {
        FPZL2_W::new(self)
    }
    #[doc = "Bit 19 - Fault Protection to Hi-Z for PWML output on channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpzl3(&mut self) -> FPZL3_W<19> {
        FPZL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Protection Value 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpv2](index.html) module"]
pub struct FPV2_SPEC;
impl crate::RegisterSpec for FPV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpv2::R](R) reader structure"]
impl crate::Readable for FPV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpv2::W](W) writer structure"]
impl crate::Writable for FPV2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPV2 to value 0"]
impl crate::Resettable for FPV2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
