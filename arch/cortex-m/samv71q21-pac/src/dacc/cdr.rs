#[doc = "Register `CDR[%s]` writer"]
pub type W = crate::W<CDR_SPEC>;
#[doc = "Field `DATA0` writer - Data to Convert for channel 0"]
pub type DATA0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `DATA1` writer - Data to Convert for channel 1"]
pub type DATA1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - Data to Convert for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<CDR_SPEC, 0> {
        DATA0_W::new(self)
    }
    #[doc = "Bits 16:31 - Data to Convert for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<CDR_SPEC, 16> {
        DATA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Conversion Data Register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cdr::W`](W) writer structure"]
impl crate::Writable for CDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDR[%s]
to value 0"]
impl crate::Resettable for CDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
