#[doc = "Register `CSELR` reader"]
pub type R = crate::R<CSELR_SPEC>;
#[doc = "Register `CSELR` writer"]
pub type W = crate::W<CSELR_SPEC>;
#[doc = "Field `CSEL` reader - Channel Selection"]
pub type CSEL_R = crate::FieldReader;
#[doc = "Field `CSEL` writer - Channel Selection"]
pub type CSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    pub fn csel(&self) -> CSEL_R {
        CSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel(&mut self) -> CSEL_W<CSELR_SPEC, 0> {
        CSEL_W::new(self)
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
#[doc = "AFEC Channel Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSELR_SPEC;
impl crate::RegisterSpec for CSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cselr::R`](R) reader structure"]
impl crate::Readable for CSELR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cselr::W`](W) writer structure"]
impl crate::Writable for CSELR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSELR to value 0"]
impl crate::Resettable for CSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
