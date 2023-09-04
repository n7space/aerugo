#[doc = "Register `SMMR` reader"]
pub type R = crate::R<SMMR_SPEC>;
#[doc = "Register `SMMR` writer"]
pub type W = crate::W<SMMR_SPEC>;
#[doc = "Field `GCEN` reader - Gray Count Enable"]
pub type GCEN_R = crate::BitReader;
#[doc = "Field `GCEN` writer - Gray Count Enable"]
pub type GCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOWN` reader - Down Count"]
pub type DOWN_R = crate::BitReader;
#[doc = "Field `DOWN` writer - Down Count"]
pub type DOWN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Gray Count Enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Down Count"]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gray Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<SMMR_SPEC, 0> {
        GCEN_W::new(self)
    }
    #[doc = "Bit 1 - Down Count"]
    #[inline(always)]
    #[must_use]
    pub fn down(&mut self) -> DOWN_W<SMMR_SPEC, 1> {
        DOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Stepper Motor Mode Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMMR_SPEC;
impl crate::RegisterSpec for SMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smmr::R`](R) reader structure"]
impl crate::Readable for SMMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smmr::W`](W) writer structure"]
impl crate::Writable for SMMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMMR to value 0"]
impl crate::Resettable for SMMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
