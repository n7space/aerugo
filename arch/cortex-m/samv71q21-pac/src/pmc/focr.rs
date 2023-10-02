#[doc = "Register `FOCR` writer"]
pub type W = crate::W<FOCR_SPEC>;
#[doc = "Field `FOCLR` writer - Fault Output Clear"]
pub type FOCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Fault Output Clear"]
    #[inline(always)]
    #[must_use]
    pub fn foclr(&mut self) -> FOCLR_W<FOCR_SPEC, 0> {
        FOCLR_W::new(self)
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
#[doc = "Fault Output Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`focr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FOCR_SPEC;
impl crate::RegisterSpec for FOCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`focr::W`](W) writer structure"]
impl crate::Writable for FOCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FOCR to value 0"]
impl crate::Resettable for FOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
