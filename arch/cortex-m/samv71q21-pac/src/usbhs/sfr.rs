#[doc = "Register `SFR` writer"]
pub type W = crate::W<SFR_SPEC>;
#[doc = "Field `RDERRIS` writer - Remote Device Connection Error Interrupt Set"]
pub type RDERRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VBUSRQS` writer - VBUS Request Set"]
pub type VBUSRQS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rderris(&mut self) -> RDERRIS_W<SFR_SPEC, 4> {
        RDERRIS_W::new(self)
    }
    #[doc = "Bit 9 - VBUS Request Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbusrqs(&mut self) -> VBUSRQS_W<SFR_SPEC, 9> {
        VBUSRQS_W::new(self)
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
#[doc = "General Status Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFR_SPEC;
impl crate::RegisterSpec for SFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sfr::W`](W) writer structure"]
impl crate::Writable for SFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFR to value 0"]
impl crate::Resettable for SFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
