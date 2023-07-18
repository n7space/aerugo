#[doc = "Register `CDUS` reader"]
pub struct R(crate::R<CDUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDUS` writer"]
pub struct W(crate::W<CDUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDUS_SPEC>;
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
impl From<crate::W<CDUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUBS` reader - Channel x Destination Microblock Stride"]
pub type DUBS_R = crate::FieldReader<u32>;
#[doc = "Field `DUBS` writer - Channel x Destination Microblock Stride"]
pub type DUBS_W<'a, const O: u8> = crate::FieldWriter<'a, CDUS_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel x Destination Microblock Stride"]
    #[inline(always)]
    pub fn dubs(&self) -> DUBS_R {
        DUBS_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Destination Microblock Stride"]
    #[inline(always)]
    #[must_use]
    pub fn dubs(&mut self) -> DUBS_W<0> {
        DUBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Destination Microblock Stride\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus](index.html) module"]
pub struct CDUS_SPEC;
impl crate::RegisterSpec for CDUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdus::R](R) reader structure"]
impl crate::Readable for CDUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdus::W](W) writer structure"]
impl crate::Writable for CDUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDUS to value 0"]
impl crate::Resettable for CDUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
