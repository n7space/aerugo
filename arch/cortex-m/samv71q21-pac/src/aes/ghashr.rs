#[doc = "Register `GHASHR[%s]` reader"]
pub type R = crate::R<GHASHR_SPEC>;
#[doc = "Register `GHASHR[%s]` writer"]
pub type W = crate::W<GHASHR_SPEC>;
#[doc = "Field `GHASH` reader - Intermediate GCM Hash Word x"]
pub type GHASH_R = crate::FieldReader<u32>;
#[doc = "Field `GHASH` writer - Intermediate GCM Hash Word x"]
pub type GHASH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Intermediate GCM Hash Word x"]
    #[inline(always)]
    pub fn ghash(&self) -> GHASH_R {
        GHASH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Intermediate GCM Hash Word x"]
    #[inline(always)]
    #[must_use]
    pub fn ghash(&mut self) -> GHASH_W<GHASHR_SPEC, 0> {
        GHASH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GCM Intermediate Hash Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghashr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ghashr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GHASHR_SPEC;
impl crate::RegisterSpec for GHASHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghashr::R`](R) reader structure"]
impl crate::Readable for GHASHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ghashr::W`](W) writer structure"]
impl crate::Writable for GHASHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GHASHR[%s]
to value 0"]
impl crate::Resettable for GHASHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
