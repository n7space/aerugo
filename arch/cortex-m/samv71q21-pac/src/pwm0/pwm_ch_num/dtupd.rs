#[doc = "Register `DTUPD` writer"]
pub type W = crate::W<DTUPD_SPEC>;
#[doc = "Field `DTHUPD` writer - Dead-Time Value Update for PWMHx Output"]
pub type DTHUPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `DTLUPD` writer - Dead-Time Value Update for PWMLx Output"]
pub type DTLUPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - Dead-Time Value Update for PWMHx Output"]
    #[inline(always)]
    #[must_use]
    pub fn dthupd(&mut self) -> DTHUPD_W<DTUPD_SPEC, 0> {
        DTHUPD_W::new(self)
    }
    #[doc = "Bits 16:31 - Dead-Time Value Update for PWMLx Output"]
    #[inline(always)]
    #[must_use]
    pub fn dtlupd(&mut self) -> DTLUPD_W<DTUPD_SPEC, 16> {
        DTLUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Channel Dead Time Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTUPD_SPEC;
impl crate::RegisterSpec for DTUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dtupd::W`](W) writer structure"]
impl crate::Writable for DTUPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTUPD to value 0"]
impl crate::Resettable for DTUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
