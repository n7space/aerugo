#[doc = "Register `TEMPCWR` reader"]
pub struct R(crate::R<TEMPCWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPCWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPCWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPCWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMPCWR` writer"]
pub struct W(crate::W<TEMPCWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMPCWR_SPEC>;
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
impl From<crate::W<TEMPCWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMPCWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLOWTHRES` reader - Temperature Low Threshold"]
pub type TLOWTHRES_R = crate::FieldReader<u16>;
#[doc = "Field `TLOWTHRES` writer - Temperature Low Threshold"]
pub type TLOWTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, TEMPCWR_SPEC, 16, O, u16>;
#[doc = "Field `THIGHTHRES` reader - Temperature High Threshold"]
pub type THIGHTHRES_R = crate::FieldReader<u16>;
#[doc = "Field `THIGHTHRES` writer - Temperature High Threshold"]
pub type THIGHTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, TEMPCWR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Temperature Low Threshold"]
    #[inline(always)]
    pub fn tlowthres(&self) -> TLOWTHRES_R {
        TLOWTHRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Temperature High Threshold"]
    #[inline(always)]
    pub fn thighthres(&self) -> THIGHTHRES_R {
        THIGHTHRES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Temperature Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tlowthres(&mut self) -> TLOWTHRES_W<0> {
        TLOWTHRES_W::new(self)
    }
    #[doc = "Bits 16:31 - Temperature High Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn thighthres(&mut self) -> THIGHTHRES_W<16> {
        THIGHTHRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Temperature Compare Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempcwr](index.html) module"]
pub struct TEMPCWR_SPEC;
impl crate::RegisterSpec for TEMPCWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tempcwr::R](R) reader structure"]
impl crate::Readable for TEMPCWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tempcwr::W](W) writer structure"]
impl crate::Writable for TEMPCWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEMPCWR to value 0"]
impl crate::Resettable for TEMPCWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
