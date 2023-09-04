#[doc = "Register `CMUPD2` writer"]
pub type W = crate::W<CMUPD2_SPEC>;
#[doc = "Field `CPOLUP` writer - Channel Polarity Update"]
pub type CPOLUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPOLINVUP` writer - Channel Polarity Inversion Update"]
pub type CPOLINVUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 9 - Channel Polarity Update"]
    #[inline(always)]
    #[must_use]
    pub fn cpolup(&mut self) -> CPOLUP_W<CMUPD2_SPEC, 9> {
        CPOLUP_W::new(self)
    }
    #[doc = "Bit 13 - Channel Polarity Inversion Update"]
    #[inline(always)]
    #[must_use]
    pub fn cpolinvup(&mut self) -> CPOLINVUP_W<CMUPD2_SPEC, 13> {
        CPOLINVUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Channel Mode Update Register (ch_num = 2)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmupd2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMUPD2_SPEC;
impl crate::RegisterSpec for CMUPD2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmupd2::W`](W) writer structure"]
impl crate::Writable for CMUPD2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMUPD2 to value 0"]
impl crate::Resettable for CMUPD2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
