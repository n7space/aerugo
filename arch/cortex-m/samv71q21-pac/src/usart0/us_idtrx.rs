#[doc = "Register `US_IDTRX` reader"]
pub struct R(crate::R<US_IDTRX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_IDTRX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_IDTRX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_IDTRX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_IDTRX` writer"]
pub struct W(crate::W<US_IDTRX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_IDTRX_SPEC>;
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
impl From<crate::W<US_IDTRX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_IDTRX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDTRX` reader - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
pub type IDTRX_R = crate::FieldReader<u32>;
#[doc = "Field `IDTRX` writer - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
pub type IDTRX_W<'a, const O: u8> = crate::FieldWriter<'a, US_IDTRX_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idtrx(&self) -> IDTRX_R {
        IDTRX_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn idtrx(&mut self) -> IDTRX_W<0> {
        IDTRX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LON IDT Rx Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_idtrx](index.html) module"]
pub struct US_IDTRX_SPEC;
impl crate::RegisterSpec for US_IDTRX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_idtrx::R](R) reader structure"]
impl crate::Readable for US_IDTRX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_idtrx::W](W) writer structure"]
impl crate::Writable for US_IDTRX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_IDTRX to value 0"]
impl crate::Resettable for US_IDTRX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
