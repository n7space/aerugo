#[doc = "Register `CWR` reader"]
pub struct R(crate::R<CWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWR` writer"]
pub struct W(crate::W<CWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWR_SPEC>;
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
impl From<crate::W<CWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWTHRES` reader - Low Threshold"]
pub type LOWTHRES_R = crate::FieldReader<u16>;
#[doc = "Field `LOWTHRES` writer - Low Threshold"]
pub type LOWTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, CWR_SPEC, 16, O, u16>;
#[doc = "Field `HIGHTHRES` reader - High Threshold"]
pub type HIGHTHRES_R = crate::FieldReader<u16>;
#[doc = "Field `HIGHTHRES` writer - High Threshold"]
pub type HIGHTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, CWR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Threshold"]
    #[inline(always)]
    pub fn lowthres(&self) -> LOWTHRES_R {
        LOWTHRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Threshold"]
    #[inline(always)]
    pub fn highthres(&self) -> HIGHTHRES_R {
        HIGHTHRES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lowthres(&mut self) -> LOWTHRES_W<0> {
        LOWTHRES_W::new(self)
    }
    #[doc = "Bits 16:31 - High Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn highthres(&mut self) -> HIGHTHRES_W<16> {
        HIGHTHRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Compare Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwr](index.html) module"]
pub struct CWR_SPEC;
impl crate::RegisterSpec for CWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwr::R](R) reader structure"]
impl crate::Readable for CWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwr::W](W) writer structure"]
impl crate::Writable for CWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWR to value 0"]
impl crate::Resettable for CWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
