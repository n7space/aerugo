#[doc = "Register `CUBC` reader"]
pub struct R(crate::R<CUBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CUBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CUBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CUBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CUBC` writer"]
pub struct W(crate::W<CUBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CUBC_SPEC>;
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
impl From<crate::W<CUBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CUBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UBLEN` reader - Channel x Microblock Length"]
pub type UBLEN_R = crate::FieldReader<u32>;
#[doc = "Field `UBLEN` writer - Channel x Microblock Length"]
pub type UBLEN_W<'a, const O: u8> = crate::FieldWriter<'a, CUBC_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel x Microblock Length"]
    #[inline(always)]
    pub fn ublen(&self) -> UBLEN_R {
        UBLEN_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Microblock Length"]
    #[inline(always)]
    #[must_use]
    pub fn ublen(&mut self) -> UBLEN_W<0> {
        UBLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Microblock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc](index.html) module"]
pub struct CUBC_SPEC;
impl crate::RegisterSpec for CUBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cubc::R](R) reader structure"]
impl crate::Readable for CUBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cubc::W](W) writer structure"]
impl crate::Writable for CUBC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CUBC to value 0"]
impl crate::Resettable for CUBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
