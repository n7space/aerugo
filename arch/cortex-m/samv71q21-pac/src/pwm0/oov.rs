#[doc = "Register `OOV` reader"]
pub struct R(crate::R<OOV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OOV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OOV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OOV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OOV` writer"]
pub struct W(crate::W<OOV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OOV_SPEC>;
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
impl From<crate::W<OOV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OOV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OOVH0` reader - Output Override Value for PWMH output of the channel 0"]
pub type OOVH0_R = crate::BitReader;
#[doc = "Field `OOVH0` writer - Output Override Value for PWMH output of the channel 0"]
pub type OOVH0_W<'a, const O: u8> = crate::BitWriter<'a, OOV_SPEC, O>;
#[doc = "Field `OOVH1` reader - Output Override Value for PWMH output of the channel 1"]
pub type OOVH1_R = crate::BitReader;
#[doc = "Field `OOVH1` writer - Output Override Value for PWMH output of the channel 1"]
pub type OOVH1_W<'a, const O: u8> = crate::BitWriter<'a, OOV_SPEC, O>;
#[doc = "Field `OOVH2` reader - Output Override Value for PWMH output of the channel 2"]
pub type OOVH2_R = crate::BitReader;
#[doc = "Field `OOVH2` writer - Output Override Value for PWMH output of the channel 2"]
pub type OOVH2_W<'a, const O: u8> = crate::BitWriter<'a, OOV_SPEC, O>;
#[doc = "Field `OOVH3` reader - Output Override Value for PWMH output of the channel 3"]
pub type OOVH3_R = crate::BitReader;
#[doc = "Field `OOVH3` writer - Output Override Value for PWMH output of the channel 3"]
pub type OOVH3_W<'a, const O: u8> = crate::BitWriter<'a, OOV_SPEC, O>;
#[doc = "Field `OOVL0` reader - Output Override Value for PWML output of the channel 0"]
pub type OOVL0_R = crate::BitReader;
#[doc = "Field `OOVL0` writer - Output Override Value for PWML output of the channel 0"]
pub type OOVL0_W<'a, const O: u8> = crate::BitWriter<'a, OOV_SPEC, O>;
#[doc = "Field `OOVL1` reader - Output Override Value for PWML output of the channel 1"]
pub type OOVL1_R = crate::BitReader;
#[doc = "Field `OOVL1` writer - Output Override Value for PWML output of the channel 1"]
pub type OOVL1_W<'a, const O: u8> = crate::BitWriter<'a, OOV_SPEC, O>;
#[doc = "Field `OOVL2` reader - Output Override Value for PWML output of the channel 2"]
pub type OOVL2_R = crate::BitReader;
#[doc = "Field `OOVL2` writer - Output Override Value for PWML output of the channel 2"]
pub type OOVL2_W<'a, const O: u8> = crate::BitWriter<'a, OOV_SPEC, O>;
#[doc = "Field `OOVL3` reader - Output Override Value for PWML output of the channel 3"]
pub type OOVL3_R = crate::BitReader;
#[doc = "Field `OOVL3` writer - Output Override Value for PWML output of the channel 3"]
pub type OOVL3_W<'a, const O: u8> = crate::BitWriter<'a, OOV_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Output Override Value for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn oovh0(&self) -> OOVH0_R {
        OOVH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Override Value for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn oovh1(&self) -> OOVH1_R {
        OOVH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Override Value for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn oovh2(&self) -> OOVH2_R {
        OOVH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Override Value for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn oovh3(&self) -> OOVH3_R {
        OOVH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Override Value for PWML output of the channel 0"]
    #[inline(always)]
    pub fn oovl0(&self) -> OOVL0_R {
        OOVL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output Override Value for PWML output of the channel 1"]
    #[inline(always)]
    pub fn oovl1(&self) -> OOVL1_R {
        OOVL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Override Value for PWML output of the channel 2"]
    #[inline(always)]
    pub fn oovl2(&self) -> OOVL2_R {
        OOVL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output Override Value for PWML output of the channel 3"]
    #[inline(always)]
    pub fn oovl3(&self) -> OOVL3_R {
        OOVL3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Override Value for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oovh0(&mut self) -> OOVH0_W<0> {
        OOVH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Override Value for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oovh1(&mut self) -> OOVH1_W<1> {
        OOVH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Override Value for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oovh2(&mut self) -> OOVH2_W<2> {
        OOVH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Override Value for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oovh3(&mut self) -> OOVH3_W<3> {
        OOVH3_W::new(self)
    }
    #[doc = "Bit 16 - Output Override Value for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn oovl0(&mut self) -> OOVL0_W<16> {
        OOVL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Override Value for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn oovl1(&mut self) -> OOVL1_W<17> {
        OOVL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Override Value for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn oovl2(&mut self) -> OOVL2_W<18> {
        OOVL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Override Value for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn oovl3(&mut self) -> OOVL3_W<19> {
        OOVL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Override Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oov](index.html) module"]
pub struct OOV_SPEC;
impl crate::RegisterSpec for OOV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oov::R](R) reader structure"]
impl crate::Readable for OOV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oov::W](W) writer structure"]
impl crate::Writable for OOV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OOV to value 0"]
impl crate::Resettable for OOV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
