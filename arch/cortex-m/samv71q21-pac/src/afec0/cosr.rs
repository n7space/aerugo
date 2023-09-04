#[doc = "Register `COSR` reader"]
pub type R = crate::R<COSR_SPEC>;
#[doc = "Register `COSR` writer"]
pub type W = crate::W<COSR_SPEC>;
#[doc = "Field `CSEL` reader - Sample &amp; Hold unit Correction Select"]
pub type CSEL_R = crate::BitReader;
#[doc = "Field `CSEL` writer - Sample &amp; Hold unit Correction Select"]
pub type CSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Sample &amp; Hold unit Correction Select"]
    #[inline(always)]
    pub fn csel(&self) -> CSEL_R {
        CSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample &amp; Hold unit Correction Select"]
    #[inline(always)]
    #[must_use]
    pub fn csel(&mut self) -> CSEL_W<COSR_SPEC, 0> {
        CSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AFEC Correction Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cosr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cosr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COSR_SPEC;
impl crate::RegisterSpec for COSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cosr::R`](R) reader structure"]
impl crate::Readable for COSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cosr::W`](W) writer structure"]
impl crate::Writable for COSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COSR to value 0"]
impl crate::Resettable for COSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
