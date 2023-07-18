#[doc = "Register `CALALR` reader"]
pub struct R(crate::R<CALALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALALR` writer"]
pub struct W(crate::W<CALALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALALR_SPEC>;
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
impl From<crate::W<CALALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MONTH` reader - Month Alarm"]
pub type MONTH_R = crate::FieldReader;
#[doc = "Field `MONTH` writer - Month Alarm"]
pub type MONTH_W<'a, const O: u8> = crate::FieldWriter<'a, CALALR_SPEC, 5, O>;
#[doc = "Field `MTHEN` reader - Month Alarm Enable"]
pub type MTHEN_R = crate::BitReader;
#[doc = "Field `MTHEN` writer - Month Alarm Enable"]
pub type MTHEN_W<'a, const O: u8> = crate::BitWriter<'a, CALALR_SPEC, O>;
#[doc = "Field `DATE` reader - Date Alarm"]
pub type DATE_R = crate::FieldReader;
#[doc = "Field `DATE` writer - Date Alarm"]
pub type DATE_W<'a, const O: u8> = crate::FieldWriter<'a, CALALR_SPEC, 6, O>;
#[doc = "Field `DATEEN` reader - Date Alarm Enable"]
pub type DATEEN_R = crate::BitReader;
#[doc = "Field `DATEEN` writer - Date Alarm Enable"]
pub type DATEEN_W<'a, const O: u8> = crate::BitWriter<'a, CALALR_SPEC, O>;
impl R {
    #[doc = "Bits 16:20 - Month Alarm"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Month Alarm Enable"]
    #[inline(always)]
    pub fn mthen(&self) -> MTHEN_R {
        MTHEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Date Alarm"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Date Alarm Enable"]
    #[inline(always)]
    pub fn dateen(&self) -> DATEEN_R {
        DATEEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - Month Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MONTH_W<16> {
        MONTH_W::new(self)
    }
    #[doc = "Bit 23 - Month Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mthen(&mut self) -> MTHEN_W<23> {
        MTHEN_W::new(self)
    }
    #[doc = "Bits 24:29 - Date Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<24> {
        DATE_W::new(self)
    }
    #[doc = "Bit 31 - Date Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dateen(&mut self) -> DATEEN_W<31> {
        DATEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calendar Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calalr](index.html) module"]
pub struct CALALR_SPEC;
impl crate::RegisterSpec for CALALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calalr::R](R) reader structure"]
impl crate::Readable for CALALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calalr::W](W) writer structure"]
impl crate::Writable for CALALR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALALR to value 0"]
impl crate::Resettable for CALALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
