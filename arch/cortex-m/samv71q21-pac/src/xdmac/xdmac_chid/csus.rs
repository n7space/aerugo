#[doc = "Register `CSUS` reader"]
pub struct R(crate::R<CSUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSUS` writer"]
pub struct W(crate::W<CSUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSUS_SPEC>;
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
impl From<crate::W<CSUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBS` reader - Channel x Source Microblock Stride"]
pub type SUBS_R = crate::FieldReader<u32>;
#[doc = "Field `SUBS` writer - Channel x Source Microblock Stride"]
pub type SUBS_W<'a, const O: u8> = crate::FieldWriter<'a, CSUS_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel x Source Microblock Stride"]
    #[inline(always)]
    pub fn subs(&self) -> SUBS_R {
        SUBS_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Source Microblock Stride"]
    #[inline(always)]
    #[must_use]
    pub fn subs(&mut self) -> SUBS_W<0> {
        SUBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Source Microblock Stride\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus](index.html) module"]
pub struct CSUS_SPEC;
impl crate::RegisterSpec for CSUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csus::R](R) reader structure"]
impl crate::Readable for CSUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csus::W](W) writer structure"]
impl crate::Writable for CSUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSUS to value 0"]
impl crate::Resettable for CSUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
