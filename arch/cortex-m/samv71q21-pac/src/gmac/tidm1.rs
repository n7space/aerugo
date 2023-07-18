#[doc = "Register `TIDM1` reader"]
pub struct R(crate::R<TIDM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIDM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIDM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIDM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIDM1` writer"]
pub struct W(crate::W<TIDM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIDM1_SPEC>;
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
impl From<crate::W<TIDM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIDM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TID` reader - Type ID Match 1"]
pub type TID_R = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID Match 1"]
pub type TID_W<'a, const O: u8> = crate::FieldWriter<'a, TIDM1_SPEC, 16, O, u16>;
#[doc = "Field `ENID1` reader - Enable Copying of TID Matched Frames"]
pub type ENID1_R = crate::BitReader;
#[doc = "Field `ENID1` writer - Enable Copying of TID Matched Frames"]
pub type ENID1_W<'a, const O: u8> = crate::BitWriter<'a, TIDM1_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - Type ID Match 1"]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid1(&self) -> ENID1_R {
        ENID1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 1"]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TID_W<0> {
        TID_W::new(self)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    #[must_use]
    pub fn enid1(&mut self) -> ENID1_W<31> {
        ENID1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Type ID Match 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tidm1](index.html) module"]
pub struct TIDM1_SPEC;
impl crate::RegisterSpec for TIDM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tidm1::R](R) reader structure"]
impl crate::Readable for TIDM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tidm1::W](W) writer structure"]
impl crate::Writable for TIDM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIDM1 to value 0"]
impl crate::Resettable for TIDM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
