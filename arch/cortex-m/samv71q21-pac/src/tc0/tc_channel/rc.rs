#[doc = "Register `RC` reader"]
pub struct R(crate::R<RC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC` writer"]
pub struct W(crate::W<RC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC_SPEC>;
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
impl From<crate::W<RC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RC` reader - Register C"]
pub type RC_R = crate::FieldReader<u32>;
#[doc = "Field `RC` writer - Register C"]
pub type RC_W<'a, const O: u8> = crate::FieldWriter<'a, RC_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    #[must_use]
    pub fn rc(&mut self) -> RC_W<0> {
        RC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register C (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc](index.html) module"]
pub struct RC_SPEC;
impl crate::RegisterSpec for RC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc::R](R) reader structure"]
impl crate::Readable for RC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc::W](W) writer structure"]
impl crate::Writable for RC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC to value 0"]
impl crate::Resettable for RC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
