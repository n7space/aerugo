#[doc = "Register `MS0` reader"]
pub struct R(crate::R<MS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MS0` writer"]
pub struct W(crate::W<MS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MS0_SPEC>;
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
impl From<crate::W<MS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCS` reader - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
pub type MCS_R = crate::FieldReader<u32>;
#[doc = "Field `MCS` writer - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
pub type MCS_W<'a, const O: u8> = crate::FieldWriter<'a, MS0_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
    #[inline(always)]
    pub fn mcs(&self) -> MCS_R {
        MCS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn mcs(&mut self) -> MCS_W<0> {
        MCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MediaLB Channel Status 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms0](index.html) module"]
pub struct MS0_SPEC;
impl crate::RegisterSpec for MS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms0::R](R) reader structure"]
impl crate::Readable for MS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ms0::W](W) writer structure"]
impl crate::Writable for MS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MS0 to value 0"]
impl crate::Resettable for MS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
