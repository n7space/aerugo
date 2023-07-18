#[doc = "Register `SCCR` writer"]
pub struct W(crate::W<SCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCCR_SPEC>;
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
impl From<crate::W<SCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACKCLR` writer - Acknowledge Clear"]
pub type ACKCLR_W<'a, const O: u8> = crate::BitWriter<'a, SCCR_SPEC, O>;
#[doc = "Field `ALRCLR` writer - Alarm Clear"]
pub type ALRCLR_W<'a, const O: u8> = crate::BitWriter<'a, SCCR_SPEC, O>;
#[doc = "Field `SECCLR` writer - Second Clear"]
pub type SECCLR_W<'a, const O: u8> = crate::BitWriter<'a, SCCR_SPEC, O>;
#[doc = "Field `TIMCLR` writer - Time Clear"]
pub type TIMCLR_W<'a, const O: u8> = crate::BitWriter<'a, SCCR_SPEC, O>;
#[doc = "Field `CALCLR` writer - Calendar Clear"]
pub type CALCLR_W<'a, const O: u8> = crate::BitWriter<'a, SCCR_SPEC, O>;
#[doc = "Field `TDERRCLR` writer - Time and/or Date Free Running Error Clear"]
pub type TDERRCLR_W<'a, const O: u8> = crate::BitWriter<'a, SCCR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Acknowledge Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ackclr(&mut self) -> ACKCLR_W<0> {
        ACKCLR_W::new(self)
    }
    #[doc = "Bit 1 - Alarm Clear"]
    #[inline(always)]
    #[must_use]
    pub fn alrclr(&mut self) -> ALRCLR_W<1> {
        ALRCLR_W::new(self)
    }
    #[doc = "Bit 2 - Second Clear"]
    #[inline(always)]
    #[must_use]
    pub fn secclr(&mut self) -> SECCLR_W<2> {
        SECCLR_W::new(self)
    }
    #[doc = "Bit 3 - Time Clear"]
    #[inline(always)]
    #[must_use]
    pub fn timclr(&mut self) -> TIMCLR_W<3> {
        TIMCLR_W::new(self)
    }
    #[doc = "Bit 4 - Calendar Clear"]
    #[inline(always)]
    #[must_use]
    pub fn calclr(&mut self) -> CALCLR_W<4> {
        CALCLR_W::new(self)
    }
    #[doc = "Bit 5 - Time and/or Date Free Running Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tderrclr(&mut self) -> TDERRCLR_W<5> {
        TDERRCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Clear Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sccr](index.html) module"]
pub struct SCCR_SPEC;
impl crate::RegisterSpec for SCCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sccr::W](W) writer structure"]
impl crate::Writable for SCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCR to value 0"]
impl crate::Resettable for SCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
