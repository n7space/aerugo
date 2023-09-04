#[doc = "Register `AADLENR` reader"]
pub type R = crate::R<AADLENR_SPEC>;
#[doc = "Register `AADLENR` writer"]
pub type W = crate::W<AADLENR_SPEC>;
#[doc = "Field `AADLEN` reader - Additional Authenticated Data Length"]
pub type AADLEN_R = crate::FieldReader<u32>;
#[doc = "Field `AADLEN` writer - Additional Authenticated Data Length"]
pub type AADLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Additional Authenticated Data Length"]
    #[inline(always)]
    pub fn aadlen(&self) -> AADLEN_R {
        AADLEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional Authenticated Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn aadlen(&mut self) -> AADLEN_W<AADLENR_SPEC, 0> {
        AADLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Additional Authenticated Data Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aadlenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aadlenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AADLENR_SPEC;
impl crate::RegisterSpec for AADLENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aadlenr::R`](R) reader structure"]
impl crate::Readable for AADLENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aadlenr::W`](W) writer structure"]
impl crate::Writable for AADLENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AADLENR to value 0"]
impl crate::Resettable for AADLENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
