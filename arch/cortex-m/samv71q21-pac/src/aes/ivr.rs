#[doc = "Register `IVR[%s]` writer"]
pub type W = crate::W<IVR_SPEC>;
#[doc = "Field `IV` writer - Initialization Vector"]
pub type IV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Initialization Vector"]
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IV_W<IVR_SPEC, 0> {
        IV_W::new(self)
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
#[doc = "Initialization Vector Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IVR_SPEC;
impl crate::RegisterSpec for IVR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ivr::W`](W) writer structure"]
impl crate::Writable for IVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IVR[%s]
to value 0"]
impl crate::Resettable for IVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
