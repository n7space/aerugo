#[doc = "Register `COCR` reader"]
pub type R = crate::R<COCR_SPEC>;
#[doc = "Register `COCR` writer"]
pub type W = crate::W<COCR_SPEC>;
#[doc = "Field `AOFF` reader - Analog Offset"]
pub type AOFF_R = crate::FieldReader<u16>;
#[doc = "Field `AOFF` writer - Analog Offset"]
pub type AOFF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - Analog Offset"]
    #[inline(always)]
    pub fn aoff(&self) -> AOFF_R {
        AOFF_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Analog Offset"]
    #[inline(always)]
    #[must_use]
    pub fn aoff(&mut self) -> AOFF_W<COCR_SPEC, 0> {
        AOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AFEC Channel Offset Compensation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COCR_SPEC;
impl crate::RegisterSpec for COCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cocr::R`](R) reader structure"]
impl crate::Readable for COCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cocr::W`](W) writer structure"]
impl crate::Writable for COCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COCR to value 0"]
impl crate::Resettable for COCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
