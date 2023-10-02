#[doc = "Register `SCCR` writer"]
pub type W = crate::W<SCCR_SPEC>;
#[doc = "Field `ACKCLR` writer - Acknowledge Clear"]
pub type ACKCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALRCLR` writer - Alarm Clear"]
pub type ALRCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SECCLR` writer - Second Clear"]
pub type SECCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMCLR` writer - Time Clear"]
pub type TIMCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALCLR` writer - Calendar Clear"]
pub type CALCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TDERRCLR` writer - Time and/or Date Free Running Error Clear"]
pub type TDERRCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Acknowledge Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ackclr(&mut self) -> ACKCLR_W<SCCR_SPEC, 0> {
        ACKCLR_W::new(self)
    }
    #[doc = "Bit 1 - Alarm Clear"]
    #[inline(always)]
    #[must_use]
    pub fn alrclr(&mut self) -> ALRCLR_W<SCCR_SPEC, 1> {
        ALRCLR_W::new(self)
    }
    #[doc = "Bit 2 - Second Clear"]
    #[inline(always)]
    #[must_use]
    pub fn secclr(&mut self) -> SECCLR_W<SCCR_SPEC, 2> {
        SECCLR_W::new(self)
    }
    #[doc = "Bit 3 - Time Clear"]
    #[inline(always)]
    #[must_use]
    pub fn timclr(&mut self) -> TIMCLR_W<SCCR_SPEC, 3> {
        TIMCLR_W::new(self)
    }
    #[doc = "Bit 4 - Calendar Clear"]
    #[inline(always)]
    #[must_use]
    pub fn calclr(&mut self) -> CALCLR_W<SCCR_SPEC, 4> {
        CALCLR_W::new(self)
    }
    #[doc = "Bit 5 - Time and/or Date Free Running Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tderrclr(&mut self) -> TDERRCLR_W<SCCR_SPEC, 5> {
        TDERRCLR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status Clear Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCCR_SPEC;
impl crate::RegisterSpec for SCCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sccr::W`](W) writer structure"]
impl crate::Writable for SCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCR to value 0"]
impl crate::Resettable for SCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
