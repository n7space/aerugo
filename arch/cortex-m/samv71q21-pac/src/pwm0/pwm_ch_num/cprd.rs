#[doc = "Register `CPRD` reader"]
pub type R = crate::R<CPRD_SPEC>;
#[doc = "Register `CPRD` writer"]
pub type W = crate::W<CPRD_SPEC>;
#[doc = "Field `CPRD` reader - Channel Period"]
pub type CPRD_R = crate::FieldReader<u32>;
#[doc = "Field `CPRD` writer - Channel Period"]
pub type CPRD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&self) -> CPRD_R {
        CPRD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    #[must_use]
    pub fn cprd(&mut self) -> CPRD_W<CPRD_SPEC, 0> {
        CPRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Channel Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPRD_SPEC;
impl crate::RegisterSpec for CPRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cprd::R`](R) reader structure"]
impl crate::Readable for CPRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cprd::W`](W) writer structure"]
impl crate::Writable for CPRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPRD to value 0"]
impl crate::Resettable for CPRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
