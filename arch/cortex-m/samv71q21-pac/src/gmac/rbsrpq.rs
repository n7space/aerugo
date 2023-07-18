#[doc = "Register `RBSRPQ[%s]` reader"]
pub struct R(crate::R<RBSRPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBSRPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBSRPQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBSRPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBSRPQ[%s]` writer"]
pub struct W(crate::W<RBSRPQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBSRPQ_SPEC>;
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
impl From<crate::W<RBSRPQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBSRPQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBS` reader - Receive Buffer Size"]
pub type RBS_R = crate::FieldReader<u16>;
#[doc = "Field `RBS` writer - Receive Buffer Size"]
pub type RBS_W<'a, const O: u8> = crate::FieldWriter<'a, RBSRPQ_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Buffer Size"]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Buffer Size"]
    #[inline(always)]
    #[must_use]
    pub fn rbs(&mut self) -> RBS_W<0> {
        RBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Buffer Size Register Priority Queue (1..5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbsrpq](index.html) module"]
pub struct RBSRPQ_SPEC;
impl crate::RegisterSpec for RBSRPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbsrpq::R](R) reader structure"]
impl crate::Readable for RBSRPQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbsrpq::W](W) writer structure"]
impl crate::Writable for RBSRPQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RBSRPQ[%s]
to value 0"]
impl crate::Resettable for RBSRPQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
