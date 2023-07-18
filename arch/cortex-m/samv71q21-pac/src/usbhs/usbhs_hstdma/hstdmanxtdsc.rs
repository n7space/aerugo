#[doc = "Register `HSTDMANXTDSC` reader"]
pub struct R(crate::R<HSTDMANXTDSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTDMANXTDSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTDMANXTDSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTDMANXTDSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTDMANXTDSC` writer"]
pub struct W(crate::W<HSTDMANXTDSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTDMANXTDSC_SPEC>;
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
impl From<crate::W<HSTDMANXTDSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTDMANXTDSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NXT_DSC_ADD` reader - Next Descriptor Address"]
pub type NXT_DSC_ADD_R = crate::FieldReader<u32>;
#[doc = "Field `NXT_DSC_ADD` writer - Next Descriptor Address"]
pub type NXT_DSC_ADD_W<'a, const O: u8> = crate::FieldWriter<'a, HSTDMANXTDSC_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    pub fn nxt_dsc_add(&self) -> NXT_DSC_ADD_R {
        NXT_DSC_ADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    #[must_use]
    pub fn nxt_dsc_add(&mut self) -> NXT_DSC_ADD_W<0> {
        NXT_DSC_ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host DMA Channel Next Descriptor Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstdmanxtdsc](index.html) module"]
pub struct HSTDMANXTDSC_SPEC;
impl crate::RegisterSpec for HSTDMANXTDSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstdmanxtdsc::R](R) reader structure"]
impl crate::Readable for HSTDMANXTDSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstdmanxtdsc::W](W) writer structure"]
impl crate::Writable for HSTDMANXTDSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTDMANXTDSC to value 0"]
impl crate::Resettable for HSTDMANXTDSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
