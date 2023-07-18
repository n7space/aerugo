#[doc = "Register `BCR` writer"]
pub struct W(crate::W<BCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCR_SPEC>;
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
impl From<crate::W<BCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNC` writer - Synchro Command"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, BCR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Synchro Command"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<0> {
        SYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr](index.html) module"]
pub struct BCR_SPEC;
impl crate::RegisterSpec for BCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bcr::W](W) writer structure"]
impl crate::Writable for BCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
