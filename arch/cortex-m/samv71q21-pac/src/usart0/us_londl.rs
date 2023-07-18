#[doc = "Register `US_LONDL` reader"]
pub struct R(crate::R<US_LONDL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_LONDL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_LONDL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_LONDL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_LONDL` writer"]
pub struct W(crate::W<US_LONDL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_LONDL_SPEC>;
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
impl From<crate::W<US_LONDL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_LONDL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LONDL` reader - LON Data Length"]
pub type LONDL_R = crate::FieldReader;
#[doc = "Field `LONDL` writer - LON Data Length"]
pub type LONDL_W<'a, const O: u8> = crate::FieldWriter<'a, US_LONDL_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - LON Data Length"]
    #[inline(always)]
    pub fn londl(&self) -> LONDL_R {
        LONDL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LON Data Length"]
    #[inline(always)]
    #[must_use]
    pub fn londl(&mut self) -> LONDL_W<0> {
        LONDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LON Data Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_londl](index.html) module"]
pub struct US_LONDL_SPEC;
impl crate::RegisterSpec for US_LONDL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_londl::R](R) reader structure"]
impl crate::Readable for US_LONDL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_londl::W](W) writer structure"]
impl crate::Writable for US_LONDL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_LONDL to value 0"]
impl crate::Resettable for US_LONDL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
