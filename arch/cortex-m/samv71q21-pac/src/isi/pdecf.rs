#[doc = "Register `PDECF` reader"]
pub struct R(crate::R<PDECF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDECF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDECF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDECF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDECF` writer"]
pub struct W(crate::W<PDECF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDECF_SPEC>;
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
impl From<crate::W<PDECF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDECF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEC_FACTOR` reader - Decimation Factor"]
pub type DEC_FACTOR_R = crate::FieldReader;
#[doc = "Field `DEC_FACTOR` writer - Decimation Factor"]
pub type DEC_FACTOR_W<'a, const O: u8> = crate::FieldWriter<'a, PDECF_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Decimation Factor"]
    #[inline(always)]
    pub fn dec_factor(&self) -> DEC_FACTOR_R {
        DEC_FACTOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Decimation Factor"]
    #[inline(always)]
    #[must_use]
    pub fn dec_factor(&mut self) -> DEC_FACTOR_W<0> {
        DEC_FACTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Preview Decimation Factor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdecf](index.html) module"]
pub struct PDECF_SPEC;
impl crate::RegisterSpec for PDECF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdecf::R](R) reader structure"]
impl crate::Readable for PDECF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdecf::W](W) writer structure"]
impl crate::Writable for PDECF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDECF to value 0"]
impl crate::Resettable for PDECF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
