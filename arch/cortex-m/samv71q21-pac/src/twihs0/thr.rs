#[doc = "Register `THR` writer"]
pub type W = crate::W<THR_SPEC>;
#[doc = "Field `TXDATA` writer - Master or Slave Transmit Holding Data"]
pub type TXDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Master or Slave Transmit Holding Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<THR_SPEC, 0> {
        TXDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Holding Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THR_SPEC;
impl crate::RegisterSpec for THR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for THR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets THR to value 0"]
impl crate::Resettable for THR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
