#[doc = "Register `MDWE[%s]` reader"]
pub struct R(crate::R<MDWE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDWE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDWE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDWE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDWE[%s]` writer"]
pub struct W(crate::W<MDWE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDWE_SPEC>;
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
impl From<crate::W<MDWE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDWE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Bitwise Write Enable for CTR Data - bits\\[31:0\\]"]
pub type MASK_R = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - Bitwise Write Enable for CTR Data - bits\\[31:0\\]"]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, MDWE_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Write Enable for CTR Data - bits\\[31:0\\]"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bitwise Write Enable for CTR Data - bits\\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<0> {
        MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIF Data Write Enable 0 Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdwe](index.html) module"]
pub struct MDWE_SPEC;
impl crate::RegisterSpec for MDWE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdwe::R](R) reader structure"]
impl crate::Readable for MDWE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdwe::W](W) writer structure"]
impl crate::Writable for MDWE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDWE[%s]
to value 0"]
impl crate::Resettable for MDWE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
