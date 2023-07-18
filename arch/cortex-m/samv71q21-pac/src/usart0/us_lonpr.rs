#[doc = "Register `US_LONPR` reader"]
pub struct R(crate::R<US_LONPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_LONPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_LONPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_LONPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_LONPR` writer"]
pub struct W(crate::W<US_LONPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_LONPR_SPEC>;
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
impl From<crate::W<US_LONPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_LONPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LONPL` reader - LON Preamble Length"]
pub type LONPL_R = crate::FieldReader<u16>;
#[doc = "Field `LONPL` writer - LON Preamble Length"]
pub type LONPL_W<'a, const O: u8> = crate::FieldWriter<'a, US_LONPR_SPEC, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - LON Preamble Length"]
    #[inline(always)]
    pub fn lonpl(&self) -> LONPL_R {
        LONPL_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - LON Preamble Length"]
    #[inline(always)]
    #[must_use]
    pub fn lonpl(&mut self) -> LONPL_W<0> {
        LONPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LON Preamble Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_lonpr](index.html) module"]
pub struct US_LONPR_SPEC;
impl crate::RegisterSpec for US_LONPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_lonpr::R](R) reader structure"]
impl crate::Readable for US_LONPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_lonpr::W](W) writer structure"]
impl crate::Writable for US_LONPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_LONPR to value 0"]
impl crate::Resettable for US_LONPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
