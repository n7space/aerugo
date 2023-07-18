#[doc = "Register `SFR` writer"]
pub struct W(crate::W<SFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFR_SPEC>;
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
impl From<crate::W<SFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDERRIS` writer - Remote Device Connection Error Interrupt Set"]
pub type RDERRIS_W<'a, const O: u8> = crate::BitWriter<'a, SFR_SPEC, O>;
#[doc = "Field `VBUSRQS` writer - VBUS Request Set"]
pub type VBUSRQS_W<'a, const O: u8> = crate::BitWriter<'a, SFR_SPEC, O>;
impl W {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rderris(&mut self) -> RDERRIS_W<4> {
        RDERRIS_W::new(self)
    }
    #[doc = "Bit 9 - VBUS Request Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbusrqs(&mut self) -> VBUSRQS_W<9> {
        VBUSRQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](index.html) module"]
pub struct SFR_SPEC;
impl crate::RegisterSpec for SFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sfr::W](W) writer structure"]
impl crate::Writable for SFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFR to value 0"]
impl crate::Resettable for SFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
