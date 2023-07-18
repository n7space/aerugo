#[doc = "Register `OSSUPD` writer"]
pub struct W(crate::W<OSSUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSSUPD_SPEC>;
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
impl From<crate::W<OSSUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSSUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSSUPH0` writer - Output Selection Set for PWMH output of the channel 0"]
pub type OSSUPH0_W<'a, const O: u8> = crate::BitWriter<'a, OSSUPD_SPEC, O>;
#[doc = "Field `OSSUPH1` writer - Output Selection Set for PWMH output of the channel 1"]
pub type OSSUPH1_W<'a, const O: u8> = crate::BitWriter<'a, OSSUPD_SPEC, O>;
#[doc = "Field `OSSUPH2` writer - Output Selection Set for PWMH output of the channel 2"]
pub type OSSUPH2_W<'a, const O: u8> = crate::BitWriter<'a, OSSUPD_SPEC, O>;
#[doc = "Field `OSSUPH3` writer - Output Selection Set for PWMH output of the channel 3"]
pub type OSSUPH3_W<'a, const O: u8> = crate::BitWriter<'a, OSSUPD_SPEC, O>;
#[doc = "Field `OSSUPL0` writer - Output Selection Set for PWML output of the channel 0"]
pub type OSSUPL0_W<'a, const O: u8> = crate::BitWriter<'a, OSSUPD_SPEC, O>;
#[doc = "Field `OSSUPL1` writer - Output Selection Set for PWML output of the channel 1"]
pub type OSSUPL1_W<'a, const O: u8> = crate::BitWriter<'a, OSSUPD_SPEC, O>;
#[doc = "Field `OSSUPL2` writer - Output Selection Set for PWML output of the channel 2"]
pub type OSSUPL2_W<'a, const O: u8> = crate::BitWriter<'a, OSSUPD_SPEC, O>;
#[doc = "Field `OSSUPL3` writer - Output Selection Set for PWML output of the channel 3"]
pub type OSSUPL3_W<'a, const O: u8> = crate::BitWriter<'a, OSSUPD_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Output Selection Set for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph0(&mut self) -> OSSUPH0_W<0> {
        OSSUPH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Selection Set for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph1(&mut self) -> OSSUPH1_W<1> {
        OSSUPH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Selection Set for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph2(&mut self) -> OSSUPH2_W<2> {
        OSSUPH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Selection Set for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossuph3(&mut self) -> OSSUPH3_W<3> {
        OSSUPH3_W::new(self)
    }
    #[doc = "Bit 16 - Output Selection Set for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl0(&mut self) -> OSSUPL0_W<16> {
        OSSUPL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Selection Set for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl1(&mut self) -> OSSUPL1_W<17> {
        OSSUPL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Selection Set for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl2(&mut self) -> OSSUPL2_W<18> {
        OSSUPL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Selection Set for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossupl3(&mut self) -> OSSUPL3_W<19> {
        OSSUPL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Selection Set Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ossupd](index.html) module"]
pub struct OSSUPD_SPEC;
impl crate::RegisterSpec for OSSUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ossupd::W](W) writer structure"]
impl crate::Writable for OSSUPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSSUPD to value 0"]
impl crate::Resettable for OSSUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
