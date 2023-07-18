#[doc = "Register `CMUPD1` writer"]
pub struct W(crate::W<CMUPD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMUPD1_SPEC>;
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
impl From<crate::W<CMUPD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMUPD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPOLUP` writer - Channel Polarity Update"]
pub type CPOLUP_W<'a, const O: u8> = crate::BitWriter<'a, CMUPD1_SPEC, O>;
#[doc = "Field `CPOLINVUP` writer - Channel Polarity Inversion Update"]
pub type CPOLINVUP_W<'a, const O: u8> = crate::BitWriter<'a, CMUPD1_SPEC, O>;
impl W {
    #[doc = "Bit 9 - Channel Polarity Update"]
    #[inline(always)]
    #[must_use]
    pub fn cpolup(&mut self) -> CPOLUP_W<9> {
        CPOLUP_W::new(self)
    }
    #[doc = "Bit 13 - Channel Polarity Inversion Update"]
    #[inline(always)]
    #[must_use]
    pub fn cpolinvup(&mut self) -> CPOLINVUP_W<13> {
        CPOLINVUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Mode Update Register (ch_num = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmupd1](index.html) module"]
pub struct CMUPD1_SPEC;
impl crate::RegisterSpec for CMUPD1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmupd1::W](W) writer structure"]
impl crate::Writable for CMUPD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMUPD1 to value 0"]
impl crate::Resettable for CMUPD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
