#[doc = "Register `FMR` reader"]
pub type R = crate::R<FMR_SPEC>;
#[doc = "Register `FMR` writer"]
pub type W = crate::W<FMR_SPEC>;
#[doc = "Field `ENCF0` reader - Enable Compare Fault Channel 0"]
pub type ENCF0_R = crate::BitReader;
#[doc = "Field `ENCF0` writer - Enable Compare Fault Channel 0"]
pub type ENCF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENCF1` reader - Enable Compare Fault Channel 1"]
pub type ENCF1_R = crate::BitReader;
#[doc = "Field `ENCF1` writer - Enable Compare Fault Channel 1"]
pub type ENCF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable Compare Fault Channel 0"]
    #[inline(always)]
    pub fn encf0(&self) -> ENCF0_R {
        ENCF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Compare Fault Channel 1"]
    #[inline(always)]
    pub fn encf1(&self) -> ENCF1_R {
        ENCF1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Compare Fault Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn encf0(&mut self) -> ENCF0_W<FMR_SPEC, 0> {
        ENCF0_W::new(self)
    }
    #[doc = "Bit 1 - Enable Compare Fault Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn encf1(&mut self) -> ENCF1_W<FMR_SPEC, 1> {
        ENCF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Fault Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMR_SPEC;
impl crate::RegisterSpec for FMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmr::R`](R) reader structure"]
impl crate::Readable for FMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmr::W`](W) writer structure"]
impl crate::Writable for FMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMR to value 0"]
impl crate::Resettable for FMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
