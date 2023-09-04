#[doc = "Register `SKR` writer"]
pub type W = crate::W<SKR_SPEC>;
#[doc = "Field `USRK` writer - User Scrambling Key"]
pub type USRK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - User Scrambling Key"]
    #[inline(always)]
    #[must_use]
    pub fn usrk(&mut self) -> USRK_W<SKR_SPEC, 0> {
        USRK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Scrambling Key Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`skr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SKR_SPEC;
impl crate::RegisterSpec for SKR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`skr::W`](W) writer structure"]
impl crate::Writable for SKR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SKR to value 0"]
impl crate::Resettable for SKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
