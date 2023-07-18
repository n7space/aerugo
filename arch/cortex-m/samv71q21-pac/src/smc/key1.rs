#[doc = "Register `KEY1` writer"]
pub struct W(crate::W<KEY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<KEY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY1` writer - Off-Chip Memory Scrambling (OCMS) Key Part 1"]
pub type KEY1_W<'a, const O: u8> = crate::FieldWriter<'a, KEY1_SPEC, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Off-Chip Memory Scrambling (OCMS) Key Part 1"]
    #[inline(always)]
    #[must_use]
    pub fn key1(&mut self) -> KEY1_W<0> {
        KEY1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Off-Chip Memory Scrambling KEY1 Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1](index.html) module"]
pub struct KEY1_SPEC;
impl crate::RegisterSpec for KEY1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [key1::W](W) writer structure"]
impl crate::Writable for KEY1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY1 to value 0"]
impl crate::Resettable for KEY1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
