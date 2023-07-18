#[doc = "Register `CSA` reader"]
pub struct R(crate::R<CSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSA` writer"]
pub struct W(crate::W<CSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSA_SPEC>;
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
impl From<crate::W<CSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SA` reader - Channel x Source Address"]
pub type SA_R = crate::FieldReader<u32>;
#[doc = "Field `SA` writer - Channel x Source Address"]
pub type SA_W<'a, const O: u8> = crate::FieldWriter<'a, CSA_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<0> {
        SA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa](index.html) module"]
pub struct CSA_SPEC;
impl crate::RegisterSpec for CSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csa::R](R) reader structure"]
impl crate::Readable for CSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csa::W](W) writer structure"]
impl crate::Writable for CSA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSA to value 0"]
impl crate::Resettable for CSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
