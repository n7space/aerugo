#[doc = "Register `SCUPUPD` writer"]
pub type W = crate::W<SCUPUPD_SPEC>;
#[doc = "Field `UPRUPD` writer - Update Period Update"]
pub type UPRUPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl W {
    #[doc = "Bits 0:3 - Update Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn uprupd(&mut self) -> UPRUPD_W<SCUPUPD_SPEC, 0> {
        UPRUPD_W::new(self)
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
#[doc = "PWM Sync Channels Update Period Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scupupd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCUPUPD_SPEC;
impl crate::RegisterSpec for SCUPUPD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scupupd::W`](W) writer structure"]
impl crate::Writable for SCUPUPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCUPUPD to value 0"]
impl crate::Resettable for SCUPUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
