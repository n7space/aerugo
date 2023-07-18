#[doc = "Register `SEQ2R` reader"]
pub struct R(crate::R<SEQ2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ2R` writer"]
pub struct W(crate::W<SEQ2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ2R_SPEC>;
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
impl From<crate::W<SEQ2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USCH8` reader - User Sequence Number 8"]
pub type USCH8_R = crate::FieldReader;
#[doc = "Field `USCH8` writer - User Sequence Number 8"]
pub type USCH8_W<'a, const O: u8> = crate::FieldWriter<'a, SEQ2R_SPEC, 4, O>;
#[doc = "Field `USCH9` reader - User Sequence Number 9"]
pub type USCH9_R = crate::FieldReader;
#[doc = "Field `USCH9` writer - User Sequence Number 9"]
pub type USCH9_W<'a, const O: u8> = crate::FieldWriter<'a, SEQ2R_SPEC, 4, O>;
#[doc = "Field `USCH10` reader - User Sequence Number 10"]
pub type USCH10_R = crate::FieldReader;
#[doc = "Field `USCH10` writer - User Sequence Number 10"]
pub type USCH10_W<'a, const O: u8> = crate::FieldWriter<'a, SEQ2R_SPEC, 4, O>;
#[doc = "Field `USCH11` reader - User Sequence Number 11"]
pub type USCH11_R = crate::FieldReader;
#[doc = "Field `USCH11` writer - User Sequence Number 11"]
pub type USCH11_W<'a, const O: u8> = crate::FieldWriter<'a, SEQ2R_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - User Sequence Number 8"]
    #[inline(always)]
    pub fn usch8(&self) -> USCH8_R {
        USCH8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&self) -> USCH9_R {
        USCH9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&self) -> USCH10_R {
        USCH10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&self) -> USCH11_R {
        USCH11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Sequence Number 8"]
    #[inline(always)]
    #[must_use]
    pub fn usch8(&mut self) -> USCH8_W<0> {
        USCH8_W::new(self)
    }
    #[doc = "Bits 4:7 - User Sequence Number 9"]
    #[inline(always)]
    #[must_use]
    pub fn usch9(&mut self) -> USCH9_W<4> {
        USCH9_W::new(self)
    }
    #[doc = "Bits 8:11 - User Sequence Number 10"]
    #[inline(always)]
    #[must_use]
    pub fn usch10(&mut self) -> USCH10_W<8> {
        USCH10_W::new(self)
    }
    #[doc = "Bits 12:15 - User Sequence Number 11"]
    #[inline(always)]
    #[must_use]
    pub fn usch11(&mut self) -> USCH11_W<12> {
        USCH11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Sequence 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq2r](index.html) module"]
pub struct SEQ2R_SPEC;
impl crate::RegisterSpec for SEQ2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq2r::R](R) reader structure"]
impl crate::Readable for SEQ2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq2r::W](W) writer structure"]
impl crate::Writable for SEQ2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ2R to value 0"]
impl crate::Resettable for SEQ2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
