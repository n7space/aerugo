#[doc = "Register `CDTYUPD` writer"]
pub type W = crate::W<CDTYUPD_SPEC>;
#[doc = "Field `CDTYUPD` writer - Channel Duty-Cycle Update"]
pub type CDTYUPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle Update"]
    #[inline(always)]
    #[must_use]
    pub fn cdtyupd(&mut self) -> CDTYUPD_W<CDTYUPD_SPEC, 0> {
        CDTYUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Channel Duty Cycle Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdtyupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDTYUPD_SPEC;
impl crate::RegisterSpec for CDTYUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cdtyupd::W`](W) writer structure"]
impl crate::Writable for CDTYUPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDTYUPD to value 0"]
impl crate::Resettable for CDTYUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
