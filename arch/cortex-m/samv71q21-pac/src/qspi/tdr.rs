#[doc = "Register `TDR` writer"]
pub type W = crate::W<TDR_SPEC>;
#[doc = "Field `TD` writer - Transmit Data"]
pub type TD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    #[must_use]
    pub fn td(&mut self) -> TD_W<TDR_SPEC, 0> {
        TD_W::new(self)
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
#[doc = "Transmit Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDR_SPEC;
impl crate::RegisterSpec for TDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
