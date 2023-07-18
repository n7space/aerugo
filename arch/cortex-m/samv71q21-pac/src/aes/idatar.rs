#[doc = "Register `IDATAR[%s]` writer"]
pub struct W(crate::W<IDATAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDATAR_SPEC>;
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
impl From<crate::W<IDATAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDATAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDATA` writer - Input Data Word"]
pub type IDATA_W<'a, const O: u8> = crate::FieldWriter<'a, IDATAR_SPEC, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data Word"]
    #[inline(always)]
    #[must_use]
    pub fn idata(&mut self) -> IDATA_W<0> {
        IDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idatar](index.html) module"]
pub struct IDATAR_SPEC;
impl crate::RegisterSpec for IDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idatar::W](W) writer structure"]
impl crate::Writable for IDATAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDATAR[%s]
to value 0"]
impl crate::Resettable for IDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
