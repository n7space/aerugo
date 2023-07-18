#[doc = "Register `HSTPIPERR[%s]` reader"]
pub struct R(crate::R<HSTPIPERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTPIPERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTPIPERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTPIPERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTPIPERR[%s]` writer"]
pub struct W(crate::W<HSTPIPERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTPIPERR_SPEC>;
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
impl From<crate::W<HSTPIPERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTPIPERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATATGL` reader - Data Toggle Error"]
pub type DATATGL_R = crate::BitReader;
#[doc = "Field `DATATGL` writer - Data Toggle Error"]
pub type DATATGL_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPERR_SPEC, O>;
#[doc = "Field `DATAPID` reader - Data PID Error"]
pub type DATAPID_R = crate::BitReader;
#[doc = "Field `DATAPID` writer - Data PID Error"]
pub type DATAPID_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPERR_SPEC, O>;
#[doc = "Field `PID` reader - Data PID Error"]
pub type PID_R = crate::BitReader;
#[doc = "Field `PID` writer - Data PID Error"]
pub type PID_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPERR_SPEC, O>;
#[doc = "Field `TIMEOUT` reader - Time-Out Error"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - Time-Out Error"]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPERR_SPEC, O>;
#[doc = "Field `CRC16` reader - CRC16 Error"]
pub type CRC16_R = crate::BitReader;
#[doc = "Field `CRC16` writer - CRC16 Error"]
pub type CRC16_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPERR_SPEC, O>;
#[doc = "Field `COUNTER` reader - Error Counter"]
pub type COUNTER_R = crate::FieldReader;
#[doc = "Field `COUNTER` writer - Error Counter"]
pub type COUNTER_W<'a, const O: u8> = crate::FieldWriter<'a, HSTPIPERR_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - Data Toggle Error"]
    #[inline(always)]
    pub fn datatgl(&self) -> DATATGL_R {
        DATATGL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data PID Error"]
    #[inline(always)]
    pub fn datapid(&self) -> DATAPID_R {
        DATAPID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data PID Error"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time-Out Error"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC16 Error"]
    #[inline(always)]
    pub fn crc16(&self) -> CRC16_R {
        CRC16_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Error Counter"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data Toggle Error"]
    #[inline(always)]
    #[must_use]
    pub fn datatgl(&mut self) -> DATATGL_W<0> {
        DATATGL_W::new(self)
    }
    #[doc = "Bit 1 - Data PID Error"]
    #[inline(always)]
    #[must_use]
    pub fn datapid(&mut self) -> DATAPID_W<1> {
        DATAPID_W::new(self)
    }
    #[doc = "Bit 2 - Data PID Error"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<2> {
        PID_W::new(self)
    }
    #[doc = "Bit 3 - Time-Out Error"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<3> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 4 - CRC16 Error"]
    #[inline(always)]
    #[must_use]
    pub fn crc16(&mut self) -> CRC16_W<4> {
        CRC16_W::new(self)
    }
    #[doc = "Bits 5:6 - Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<5> {
        COUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpiperr](index.html) module"]
pub struct HSTPIPERR_SPEC;
impl crate::RegisterSpec for HSTPIPERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstpiperr::R](R) reader structure"]
impl crate::Readable for HSTPIPERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstpiperr::W](W) writer structure"]
impl crate::Writable for HSTPIPERR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTPIPERR[%s]
to value 0"]
impl crate::Resettable for HSTPIPERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
