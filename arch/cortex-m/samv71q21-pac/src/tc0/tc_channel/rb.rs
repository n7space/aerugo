#[doc = "Register `RB` reader"]
pub type R = crate::R<RB_SPEC>;
#[doc = "Register `RB` writer"]
pub type W = crate::W<RB_SPEC>;
#[doc = "Field `RB` reader - Register B"]
pub type RB_R = crate::FieldReader<u32>;
#[doc = "Field `RB` writer - Register B"]
pub type RB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    #[must_use]
    pub fn rb(&mut self) -> RB_W<RB_SPEC, 0> {
        RB_W::new(self)
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
#[doc = "Register B (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RB_SPEC;
impl crate::RegisterSpec for RB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rb::R`](R) reader structure"]
impl crate::Readable for RB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rb::W`](W) writer structure"]
impl crate::Writable for RB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RB to value 0"]
impl crate::Resettable for RB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
