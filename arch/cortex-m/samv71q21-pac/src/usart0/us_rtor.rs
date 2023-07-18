#[doc = "Register `US_RTOR` reader"]
pub struct R(crate::R<US_RTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_RTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_RTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_RTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_RTOR` writer"]
pub struct W(crate::W<US_RTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_RTOR_SPEC>;
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
impl From<crate::W<US_RTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_RTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO` reader - Timeout Value"]
pub type TO_R = crate::FieldReader<u32>;
#[doc = "Field `TO` writer - Timeout Value"]
pub type TO_W<'a, const O: u8> = crate::FieldWriter<'a, US_RTOR_SPEC, 17, O, u32>;
impl R {
    #[doc = "Bits 0:16 - Timeout Value"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Timeout Value"]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> TO_W<0> {
        TO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_rtor](index.html) module"]
pub struct US_RTOR_SPEC;
impl crate::RegisterSpec for US_RTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_rtor::R](R) reader structure"]
impl crate::Readable for US_RTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_rtor::W](W) writer structure"]
impl crate::Writable for US_RTOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_RTOR to value 0"]
impl crate::Resettable for US_RTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
