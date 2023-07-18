#[doc = "Register `SMBTR` reader"]
pub struct R(crate::R<SMBTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMBTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMBTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMBTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMBTR` writer"]
pub struct W(crate::W<SMBTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMBTR_SPEC>;
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
impl From<crate::W<SMBTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMBTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC` reader - SMBus Clock Prescaler"]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - SMBus Clock Prescaler"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, SMBTR_SPEC, 4, O>;
#[doc = "Field `TLOWS` reader - Slave Clock Stretch Maximum Cycles"]
pub type TLOWS_R = crate::FieldReader;
#[doc = "Field `TLOWS` writer - Slave Clock Stretch Maximum Cycles"]
pub type TLOWS_W<'a, const O: u8> = crate::FieldWriter<'a, SMBTR_SPEC, 8, O>;
#[doc = "Field `TLOWM` reader - Master Clock Stretch Maximum Cycles"]
pub type TLOWM_R = crate::FieldReader;
#[doc = "Field `TLOWM` writer - Master Clock Stretch Maximum Cycles"]
pub type TLOWM_W<'a, const O: u8> = crate::FieldWriter<'a, SMBTR_SPEC, 8, O>;
#[doc = "Field `THMAX` reader - Clock High Maximum Cycles"]
pub type THMAX_R = crate::FieldReader;
#[doc = "Field `THMAX` writer - Clock High Maximum Cycles"]
pub type THMAX_W<'a, const O: u8> = crate::FieldWriter<'a, SMBTR_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:3 - SMBus Clock Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Slave Clock Stretch Maximum Cycles"]
    #[inline(always)]
    pub fn tlows(&self) -> TLOWS_R {
        TLOWS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Master Clock Stretch Maximum Cycles"]
    #[inline(always)]
    pub fn tlowm(&self) -> TLOWM_R {
        TLOWM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock High Maximum Cycles"]
    #[inline(always)]
    pub fn thmax(&self) -> THMAX_R {
        THMAX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SMBus Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<0> {
        PRESC_W::new(self)
    }
    #[doc = "Bits 8:15 - Slave Clock Stretch Maximum Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn tlows(&mut self) -> TLOWS_W<8> {
        TLOWS_W::new(self)
    }
    #[doc = "Bits 16:23 - Master Clock Stretch Maximum Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn tlowm(&mut self) -> TLOWM_W<16> {
        TLOWM_W::new(self)
    }
    #[doc = "Bits 24:31 - Clock High Maximum Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn thmax(&mut self) -> THMAX_W<24> {
        THMAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMBus Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smbtr](index.html) module"]
pub struct SMBTR_SPEC;
impl crate::RegisterSpec for SMBTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smbtr::R](R) reader structure"]
impl crate::Readable for SMBTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smbtr::W](W) writer structure"]
impl crate::Writable for SMBTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMBTR to value 0"]
impl crate::Resettable for SMBTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
