#[doc = "Register `OSS` writer"]
pub type W = crate::W<OSS_SPEC>;
#[doc = "Field `OSSH0` writer - Output Selection Set for PWMH output of the channel 0"]
pub type OSSH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSH1` writer - Output Selection Set for PWMH output of the channel 1"]
pub type OSSH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSH2` writer - Output Selection Set for PWMH output of the channel 2"]
pub type OSSH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSH3` writer - Output Selection Set for PWMH output of the channel 3"]
pub type OSSH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSL0` writer - Output Selection Set for PWML output of the channel 0"]
pub type OSSL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSL1` writer - Output Selection Set for PWML output of the channel 1"]
pub type OSSL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSL2` writer - Output Selection Set for PWML output of the channel 2"]
pub type OSSL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSSL3` writer - Output Selection Set for PWML output of the channel 3"]
pub type OSSL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Output Selection Set for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossh0(&mut self) -> OSSH0_W<OSS_SPEC, 0> {
        OSSH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Selection Set for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossh1(&mut self) -> OSSH1_W<OSS_SPEC, 1> {
        OSSH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Selection Set for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossh2(&mut self) -> OSSH2_W<OSS_SPEC, 2> {
        OSSH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Selection Set for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossh3(&mut self) -> OSSH3_W<OSS_SPEC, 3> {
        OSSH3_W::new(self)
    }
    #[doc = "Bit 16 - Output Selection Set for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ossl0(&mut self) -> OSSL0_W<OSS_SPEC, 16> {
        OSSL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Selection Set for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ossl1(&mut self) -> OSSL1_W<OSS_SPEC, 17> {
        OSSL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Selection Set for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ossl2(&mut self) -> OSSL2_W<OSS_SPEC, 18> {
        OSSL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Selection Set for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ossl3(&mut self) -> OSSL3_W<OSS_SPEC, 19> {
        OSSL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Output Selection Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oss::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSS_SPEC;
impl crate::RegisterSpec for OSS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`oss::W`](W) writer structure"]
impl crate::Writable for OSS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSS to value 0"]
impl crate::Resettable for OSS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
