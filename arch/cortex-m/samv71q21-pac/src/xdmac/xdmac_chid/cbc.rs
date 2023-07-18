#[doc = "Register `CBC` reader"]
pub struct R(crate::R<CBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBC` writer"]
pub struct W(crate::W<CBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBC_SPEC>;
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
impl From<crate::W<CBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLEN` reader - Channel x Block Length"]
pub type BLEN_R = crate::FieldReader<u16>;
#[doc = "Field `BLEN` writer - Channel x Block Length"]
pub type BLEN_W<'a, const O: u8> = crate::FieldWriter<'a, CBC_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Channel x Block Length"]
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel x Block Length"]
    #[inline(always)]
    #[must_use]
    pub fn blen(&mut self) -> BLEN_W<0> {
        BLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Block Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbc](index.html) module"]
pub struct CBC_SPEC;
impl crate::RegisterSpec for CBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cbc::R](R) reader structure"]
impl crate::Readable for CBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbc::W](W) writer structure"]
impl crate::Writable for CBC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBC to value 0"]
impl crate::Resettable for CBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
