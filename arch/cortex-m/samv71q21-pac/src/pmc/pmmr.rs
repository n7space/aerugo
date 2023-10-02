#[doc = "Register `PMMR` reader"]
pub type R = crate::R<PMMR_SPEC>;
#[doc = "Register `PMMR` writer"]
pub type W = crate::W<PMMR_SPEC>;
#[doc = "Field `PLLA_MMAX` reader - PLLA Maximum Allowed Multiplier Value"]
pub type PLLA_MMAX_R = crate::FieldReader<u16>;
#[doc = "Field `PLLA_MMAX` writer - PLLA Maximum Allowed Multiplier Value"]
pub type PLLA_MMAX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - PLLA Maximum Allowed Multiplier Value"]
    #[inline(always)]
    pub fn plla_mmax(&self) -> PLLA_MMAX_R {
        PLLA_MMAX_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - PLLA Maximum Allowed Multiplier Value"]
    #[inline(always)]
    #[must_use]
    pub fn plla_mmax(&mut self) -> PLLA_MMAX_W<PMMR_SPEC, 0> {
        PLLA_MMAX_W::new(self)
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
#[doc = "PLL Maximum Multiplier Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMMR_SPEC;
impl crate::RegisterSpec for PMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmmr::R`](R) reader structure"]
impl crate::Readable for PMMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmmr::W`](W) writer structure"]
impl crate::Writable for PMMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMMR to value 0"]
impl crate::Resettable for PMMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
