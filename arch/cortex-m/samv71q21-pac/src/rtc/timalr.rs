#[doc = "Register `TIMALR` reader"]
pub struct R(crate::R<TIMALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMALR` writer"]
pub struct W(crate::W<TIMALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMALR_SPEC>;
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
impl From<crate::W<TIMALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - Second Alarm"]
pub type SEC_R = crate::FieldReader;
#[doc = "Field `SEC` writer - Second Alarm"]
pub type SEC_W<'a, const O: u8> = crate::FieldWriter<'a, TIMALR_SPEC, 7, O>;
#[doc = "Field `SECEN` reader - Second Alarm Enable"]
pub type SECEN_R = crate::BitReader;
#[doc = "Field `SECEN` writer - Second Alarm Enable"]
pub type SECEN_W<'a, const O: u8> = crate::BitWriter<'a, TIMALR_SPEC, O>;
#[doc = "Field `MIN` reader - Minute Alarm"]
pub type MIN_R = crate::FieldReader;
#[doc = "Field `MIN` writer - Minute Alarm"]
pub type MIN_W<'a, const O: u8> = crate::FieldWriter<'a, TIMALR_SPEC, 7, O>;
#[doc = "Field `MINEN` reader - Minute Alarm Enable"]
pub type MINEN_R = crate::BitReader;
#[doc = "Field `MINEN` writer - Minute Alarm Enable"]
pub type MINEN_W<'a, const O: u8> = crate::BitWriter<'a, TIMALR_SPEC, O>;
#[doc = "Field `HOUR` reader - Hour Alarm"]
pub type HOUR_R = crate::FieldReader;
#[doc = "Field `HOUR` writer - Hour Alarm"]
pub type HOUR_W<'a, const O: u8> = crate::FieldWriter<'a, TIMALR_SPEC, 6, O>;
#[doc = "Field `AMPM` reader - AM/PM Indicator"]
pub type AMPM_R = crate::BitReader;
#[doc = "Field `AMPM` writer - AM/PM Indicator"]
pub type AMPM_W<'a, const O: u8> = crate::BitWriter<'a, TIMALR_SPEC, O>;
#[doc = "Field `HOUREN` reader - Hour Alarm Enable"]
pub type HOUREN_R = crate::BitReader;
#[doc = "Field `HOUREN` writer - Hour Alarm Enable"]
pub type HOUREN_W<'a, const O: u8> = crate::BitWriter<'a, TIMALR_SPEC, O>;
impl R {
    #[doc = "Bits 0:6 - Second Alarm"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Second Alarm Enable"]
    #[inline(always)]
    pub fn secen(&self) -> SECEN_R {
        SECEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Minute Alarm"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Minute Alarm Enable"]
    #[inline(always)]
    pub fn minen(&self) -> MINEN_R {
        MINEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Hour Alarm"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - AM/PM Indicator"]
    #[inline(always)]
    pub fn ampm(&self) -> AMPM_R {
        AMPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Hour Alarm Enable"]
    #[inline(always)]
    pub fn houren(&self) -> HOUREN_R {
        HOUREN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Second Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<0> {
        SEC_W::new(self)
    }
    #[doc = "Bit 7 - Second Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn secen(&mut self) -> SECEN_W<7> {
        SECEN_W::new(self)
    }
    #[doc = "Bits 8:14 - Minute Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MIN_W<8> {
        MIN_W::new(self)
    }
    #[doc = "Bit 15 - Minute Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn minen(&mut self) -> MINEN_W<15> {
        MINEN_W::new(self)
    }
    #[doc = "Bits 16:21 - Hour Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<16> {
        HOUR_W::new(self)
    }
    #[doc = "Bit 22 - AM/PM Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn ampm(&mut self) -> AMPM_W<22> {
        AMPM_W::new(self)
    }
    #[doc = "Bit 23 - Hour Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn houren(&mut self) -> HOUREN_W<23> {
        HOUREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timalr](index.html) module"]
pub struct TIMALR_SPEC;
impl crate::RegisterSpec for TIMALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timalr::R](R) reader structure"]
impl crate::Readable for TIMALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timalr::W](W) writer structure"]
impl crate::Writable for TIMALR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMALR to value 0"]
impl crate::Resettable for TIMALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
