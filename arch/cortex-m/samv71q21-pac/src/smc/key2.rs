#[doc = "Register `KEY2` writer"]
pub type W = crate::W<KEY2_SPEC>;
#[doc = "Field `KEY2` writer - Off-Chip Memory Scrambling (OCMS) Key Part 2"]
pub type KEY2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Off-Chip Memory Scrambling (OCMS) Key Part 2"]
    #[inline(always)]
    #[must_use]
    pub fn key2(&mut self) -> KEY2_W<KEY2_SPEC, 0> {
        KEY2_W::new(self)
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
#[doc = "SMC Off-Chip Memory Scrambling KEY2 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY2_SPEC;
impl crate::RegisterSpec for KEY2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key2::W`](W) writer structure"]
impl crate::Writable for KEY2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY2 to value 0"]
impl crate::Resettable for KEY2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
