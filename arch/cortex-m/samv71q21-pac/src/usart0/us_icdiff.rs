#[doc = "Register `US_ICDIFF` reader"]
pub struct R(crate::R<US_ICDIFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_ICDIFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_ICDIFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_ICDIFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_ICDIFF` writer"]
pub struct W(crate::W<US_ICDIFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_ICDIFF_SPEC>;
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
impl From<crate::W<US_ICDIFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_ICDIFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICDIFF` reader - IC Differentiator Number"]
pub type ICDIFF_R = crate::FieldReader;
#[doc = "Field `ICDIFF` writer - IC Differentiator Number"]
pub type ICDIFF_W<'a, const O: u8> = crate::FieldWriter<'a, US_ICDIFF_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - IC Differentiator Number"]
    #[inline(always)]
    pub fn icdiff(&self) -> ICDIFF_R {
        ICDIFF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IC Differentiator Number"]
    #[inline(always)]
    #[must_use]
    pub fn icdiff(&mut self) -> ICDIFF_W<0> {
        ICDIFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IC DIFF Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_icdiff](index.html) module"]
pub struct US_ICDIFF_SPEC;
impl crate::RegisterSpec for US_ICDIFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_icdiff::R](R) reader structure"]
impl crate::Readable for US_ICDIFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_icdiff::W](W) writer structure"]
impl crate::Writable for US_ICDIFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_ICDIFF to value 0"]
impl crate::Resettable for US_ICDIFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
