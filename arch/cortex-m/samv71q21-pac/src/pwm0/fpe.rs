#[doc = "Register `FPE` reader"]
pub type R = crate::R<FPE_SPEC>;
#[doc = "Register `FPE` writer"]
pub type W = crate::W<FPE_SPEC>;
#[doc = "Field `FPE0` reader - Fault Protection Enable for channel 0"]
pub type FPE0_R = crate::FieldReader;
#[doc = "Field `FPE0` writer - Fault Protection Enable for channel 0"]
pub type FPE0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FPE1` reader - Fault Protection Enable for channel 1"]
pub type FPE1_R = crate::FieldReader;
#[doc = "Field `FPE1` writer - Fault Protection Enable for channel 1"]
pub type FPE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FPE2` reader - Fault Protection Enable for channel 2"]
pub type FPE2_R = crate::FieldReader;
#[doc = "Field `FPE2` writer - Fault Protection Enable for channel 2"]
pub type FPE2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FPE3` reader - Fault Protection Enable for channel 3"]
pub type FPE3_R = crate::FieldReader;
#[doc = "Field `FPE3` writer - Fault Protection Enable for channel 3"]
pub type FPE3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 0"]
    #[inline(always)]
    pub fn fpe0(&self) -> FPE0_R {
        FPE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 1"]
    #[inline(always)]
    pub fn fpe1(&self) -> FPE1_R {
        FPE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 2"]
    #[inline(always)]
    pub fn fpe2(&self) -> FPE2_R {
        FPE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 3"]
    #[inline(always)]
    pub fn fpe3(&self) -> FPE3_R {
        FPE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpe0(&mut self) -> FPE0_W<FPE_SPEC, 0> {
        FPE0_W::new(self)
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpe1(&mut self) -> FPE1_W<FPE_SPEC, 8> {
        FPE1_W::new(self)
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpe2(&mut self) -> FPE2_W<FPE_SPEC, 16> {
        FPE2_W::new(self)
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpe3(&mut self) -> FPE3_W<FPE_SPEC, 24> {
        FPE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Fault Protection Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPE_SPEC;
impl crate::RegisterSpec for FPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpe::R`](R) reader structure"]
impl crate::Readable for FPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpe::W`](W) writer structure"]
impl crate::Writable for FPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPE to value 0"]
impl crate::Resettable for FPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
