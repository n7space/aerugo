#[doc = "Register `RC1R` reader"]
pub struct R(crate::R<RC1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC1R` writer"]
pub struct W(crate::W<RC1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC1R_SPEC>;
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
impl From<crate::W<RC1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CP1` reader - Receive Compare Data 1"]
pub type CP1_R = crate::FieldReader<u16>;
#[doc = "Field `CP1` writer - Receive Compare Data 1"]
pub type CP1_W<'a, const O: u8> = crate::FieldWriter<'a, RC1R_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Compare Data 1"]
    #[inline(always)]
    pub fn cp1(&self) -> CP1_R {
        CP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Compare Data 1"]
    #[inline(always)]
    #[must_use]
    pub fn cp1(&mut self) -> CP1_W<0> {
        CP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Compare 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc1r](index.html) module"]
pub struct RC1R_SPEC;
impl crate::RegisterSpec for RC1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc1r::R](R) reader structure"]
impl crate::Readable for RC1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc1r::W](W) writer structure"]
impl crate::Writable for RC1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC1R to value 0"]
impl crate::Resettable for RC1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
