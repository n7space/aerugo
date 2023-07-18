#[doc = "Register `OSS` writer"]
pub struct W(crate::W<OSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSS_SPEC>;
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
impl From<crate::W<OSS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSSH0` writer - Output Selection Set for PWMH output of the channel 0"]
pub type OSSH0_W<'a, const O: u8> = crate::BitWriter<'a, OSS_SPEC, O>;
#[doc = "Field `OSSH1` writer - Output Selection Set for PWMH output of the channel 1"]
pub type OSSH1_W<'a, const O: u8> = crate::BitWriter<'a, OSS_SPEC, O>;
#[doc = "Field `OSSH2` writer - Output Selection Set for PWMH output of the channel 2"]
pub type OSSH2_W<'a, const O: u8> = crate::BitWriter<'a, OSS_SPEC, O>;
#[doc = "Field `OSSH3` writer - Output Selection Set for PWMH output of the channel 3"]
pub type OSSH3_W<'a, const O: u8> = crate::BitWriter<'a, OSS_SPEC, O>;
#[doc = "Field `OSSL0` writer - Output Selection Set for PWML output of the channel 0"]
pub type OSSL0_W<'a, const O: u8> = crate::BitWriter<'a, OSS_SPEC, O>;
#[doc = "Field `OSSL1` writer - Output Selection Set for PWML output of the channel 1"]
pub type OSSL1_W<'a, const O: u8> = crate::BitWriter<'a, OSS_SPEC, O>;
#[doc = "Field `OSSL2` writer - Output Selection Set for PWML output of the channel 2"]
pub type OSSL2_W<'a, const O: u8> = crate::BitWriter<'a, OSS_SPEC, O>;
#[doc = "Field `OSSL3` writer - Output Selection Set for PWML output of the channel 3"]
pub type OSSL3_W<'a, const O: u8> = crate::BitWriter<'a, OSS_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Output Selection Set for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossh0(&mut self) -> OSSH0_W<0> {
        OSSH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Selection Set for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossh1(&mut self) -> OSSH1_W<1> {
        OSSH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Selection Set for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossh2(&mut self) -> OSSH2_W<2> {
        OSSH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Selection Set for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossh3(&mut self) -> OSSH3_W<3> {
        OSSH3_W::new(self)
    }
    #[doc = "Bit 16 - Output Selection Set for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossl0(&mut self) -> OSSL0_W<16> {
        OSSL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Selection Set for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossl1(&mut self) -> OSSL1_W<17> {
        OSSL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Selection Set for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossl2(&mut self) -> OSSL2_W<18> {
        OSSL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Selection Set for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossl3(&mut self) -> OSSL3_W<19> {
        OSSL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Selection Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oss](index.html) module"]
pub struct OSS_SPEC;
impl crate::RegisterSpec for OSS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [oss::W](W) writer structure"]
impl crate::Writable for OSS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSS to value 0"]
impl crate::Resettable for OSS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
