#[doc = "Register `BCR` writer"]
pub type W = crate::W<BCR_SPEC>;
#[doc = "Field `SYNC` writer - Synchro Command"]
pub type SYNC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Synchro Command"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<BCR_SPEC, 0> {
        SYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Block Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCR_SPEC;
impl crate::RegisterSpec for BCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bcr::W`](W) writer structure"]
impl crate::Writable for BCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
