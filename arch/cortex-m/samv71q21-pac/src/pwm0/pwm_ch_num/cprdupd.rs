#[doc = "Register `CPRDUPD` writer"]
pub type W = crate::W<CPRDUPD_SPEC>;
#[doc = "Field `CPRDUPD` writer - Channel Period Update"]
pub type CPRDUPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl W {
    #[doc = "Bits 0:23 - Channel Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn cprdupd(&mut self) -> CPRDUPD_W<CPRDUPD_SPEC, 0> {
        CPRDUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Channel Period Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprdupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPRDUPD_SPEC;
impl crate::RegisterSpec for CPRDUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cprdupd::W`](W) writer structure"]
impl crate::Writable for CPRDUPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPRDUPD to value 0"]
impl crate::Resettable for CPRDUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
