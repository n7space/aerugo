#[doc = "Register `ST2ER[%s]` reader"]
pub struct R(crate::R<ST2ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST2ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST2ER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST2ER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST2ER[%s]` writer"]
pub struct W(crate::W<ST2ER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST2ER_SPEC>;
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
impl From<crate::W<ST2ER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST2ER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPVAL` reader - Ethertype Compare Value"]
pub type COMPVAL_R = crate::FieldReader<u16>;
#[doc = "Field `COMPVAL` writer - Ethertype Compare Value"]
pub type COMPVAL_W<'a, const O: u8> = crate::FieldWriter<'a, ST2ER_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Ethertype Compare Value"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Ethertype Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn compval(&mut self) -> COMPVAL_W<0> {
        COMPVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Screening Type 2 Ethertype Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2er](index.html) module"]
pub struct ST2ER_SPEC;
impl crate::RegisterSpec for ST2ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st2er::R](R) reader structure"]
impl crate::Readable for ST2ER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st2er::W](W) writer structure"]
impl crate::Writable for ST2ER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST2ER[%s]
to value 0"]
impl crate::Resettable for ST2ER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
