#[doc = "Register `THR` writer"]
pub struct W(crate::W<THR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THR_SPEC>;
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
impl From<crate::W<THR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` writer - Master or Slave Transmit Holding Data"]
pub type TXDATA_W<'a, const O: u8> = crate::FieldWriter<'a, THR_SPEC, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Master or Slave Transmit Holding Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<0> {
        TXDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](index.html) module"]
pub struct THR_SPEC;
impl crate::RegisterSpec for THR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [thr::W](W) writer structure"]
impl crate::Writable for THR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets THR to value 0"]
impl crate::Resettable for THR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
