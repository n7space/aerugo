#[doc = "Register `ACSR[%s]` reader"]
pub struct R(crate::R<ACSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACSR[%s]` writer"]
pub struct W(crate::W<ACSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACSR_SPEC>;
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
impl From<crate::W<ACSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHS` reader - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
pub type CHS_R = crate::FieldReader<u32>;
#[doc = "Field `CHS` writer - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
pub type CHS_W<'a, const O: u8> = crate::FieldWriter<'a, ACSR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
    #[inline(always)]
    pub fn chs(&self) -> CHS_R {
        CHS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
    #[inline(always)]
    #[must_use]
    pub fn chs(&mut self) -> CHS_W<0> {
        CHS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Channel Status 0 Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acsr](index.html) module"]
pub struct ACSR_SPEC;
impl crate::RegisterSpec for ACSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acsr::R](R) reader structure"]
impl crate::Readable for ACSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acsr::W](W) writer structure"]
impl crate::Writable for ACSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSR[%s]
to value 0"]
impl crate::Resettable for ACSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
