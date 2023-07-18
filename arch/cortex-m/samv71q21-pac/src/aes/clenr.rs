#[doc = "Register `CLENR` reader"]
pub struct R(crate::R<CLENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLENR` writer"]
pub struct W(crate::W<CLENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLENR_SPEC>;
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
impl From<crate::W<CLENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLEN` reader - Plaintext/Ciphertext Length"]
pub type CLEN_R = crate::FieldReader<u32>;
#[doc = "Field `CLEN` writer - Plaintext/Ciphertext Length"]
pub type CLEN_W<'a, const O: u8> = crate::FieldWriter<'a, CLENR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Plaintext/Ciphertext Length"]
    #[inline(always)]
    pub fn clen(&self) -> CLEN_R {
        CLEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Plaintext/Ciphertext Length"]
    #[inline(always)]
    #[must_use]
    pub fn clen(&mut self) -> CLEN_W<0> {
        CLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Plaintext/Ciphertext Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clenr](index.html) module"]
pub struct CLENR_SPEC;
impl crate::RegisterSpec for CLENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clenr::R](R) reader structure"]
impl crate::Readable for CLENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clenr::W](W) writer structure"]
impl crate::Writable for CLENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLENR to value 0"]
impl crate::Resettable for CLENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
