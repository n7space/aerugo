#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTPRES` reader - Real-time Timer Prescaler Value"]
pub type RTPRES_R = crate::FieldReader<u16>;
#[doc = "Field `RTPRES` writer - Real-time Timer Prescaler Value"]
pub type RTPRES_W<'a, const O: u8> = crate::FieldWriter<'a, MR_SPEC, 16, O, u16>;
#[doc = "Field `ALMIEN` reader - Alarm Interrupt Enable"]
pub type ALMIEN_R = crate::BitReader;
#[doc = "Field `ALMIEN` writer - Alarm Interrupt Enable"]
pub type ALMIEN_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `RTTINCIEN` reader - Real-time Timer Increment Interrupt Enable"]
pub type RTTINCIEN_R = crate::BitReader;
#[doc = "Field `RTTINCIEN` writer - Real-time Timer Increment Interrupt Enable"]
pub type RTTINCIEN_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `RTTRST` reader - Real-time Timer Restart"]
pub type RTTRST_R = crate::BitReader;
#[doc = "Field `RTTRST` writer - Real-time Timer Restart"]
pub type RTTRST_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `RTTDIS` reader - Real-time Timer Disable"]
pub type RTTDIS_R = crate::BitReader;
#[doc = "Field `RTTDIS` writer - Real-time Timer Disable"]
pub type RTTDIS_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `RTC1HZ` reader - Real-Time Clock 1Hz Clock Selection"]
pub type RTC1HZ_R = crate::BitReader;
#[doc = "Field `RTC1HZ` writer - Real-Time Clock 1Hz Clock Selection"]
pub type RTC1HZ_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - Real-time Timer Prescaler Value"]
    #[inline(always)]
    pub fn rtpres(&self) -> RTPRES_R {
        RTPRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn almien(&self) -> ALMIEN_R {
        ALMIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Real-time Timer Increment Interrupt Enable"]
    #[inline(always)]
    pub fn rttincien(&self) -> RTTINCIEN_R {
        RTTINCIEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Real-time Timer Restart"]
    #[inline(always)]
    pub fn rttrst(&self) -> RTTRST_R {
        RTTRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Real-time Timer Disable"]
    #[inline(always)]
    pub fn rttdis(&self) -> RTTDIS_R {
        RTTDIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Real-Time Clock 1Hz Clock Selection"]
    #[inline(always)]
    pub fn rtc1hz(&self) -> RTC1HZ_R {
        RTC1HZ_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Real-time Timer Prescaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn rtpres(&mut self) -> RTPRES_W<0> {
        RTPRES_W::new(self)
    }
    #[doc = "Bit 16 - Alarm Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn almien(&mut self) -> ALMIEN_W<16> {
        ALMIEN_W::new(self)
    }
    #[doc = "Bit 17 - Real-time Timer Increment Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rttincien(&mut self) -> RTTINCIEN_W<17> {
        RTTINCIEN_W::new(self)
    }
    #[doc = "Bit 18 - Real-time Timer Restart"]
    #[inline(always)]
    #[must_use]
    pub fn rttrst(&mut self) -> RTTRST_W<18> {
        RTTRST_W::new(self)
    }
    #[doc = "Bit 20 - Real-time Timer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rttdis(&mut self) -> RTTDIS_W<20> {
        RTTDIS_W::new(self)
    }
    #[doc = "Bit 24 - Real-Time Clock 1Hz Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtc1hz(&mut self) -> RTC1HZ_W<24> {
        RTC1HZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
