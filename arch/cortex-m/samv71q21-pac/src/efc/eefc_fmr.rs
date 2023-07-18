#[doc = "Register `EEFC_FMR` reader"]
pub struct R(crate::R<EEFC_FMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFC_FMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFC_FMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFC_FMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEFC_FMR` writer"]
pub struct W(crate::W<EEFC_FMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEFC_FMR_SPEC>;
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
impl From<crate::W<EEFC_FMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEFC_FMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRDY` reader - Flash Ready Interrupt Enable"]
pub type FRDY_R = crate::BitReader;
#[doc = "Field `FRDY` writer - Flash Ready Interrupt Enable"]
pub type FRDY_W<'a, const O: u8> = crate::BitWriter<'a, EEFC_FMR_SPEC, O>;
#[doc = "Field `FWS` reader - Flash Wait State"]
pub type FWS_R = crate::FieldReader;
#[doc = "Field `FWS` writer - Flash Wait State"]
pub type FWS_W<'a, const O: u8> = crate::FieldWriter<'a, EEFC_FMR_SPEC, 4, O>;
#[doc = "Field `SCOD` reader - Sequential Code Optimization Disable"]
pub type SCOD_R = crate::BitReader;
#[doc = "Field `SCOD` writer - Sequential Code Optimization Disable"]
pub type SCOD_W<'a, const O: u8> = crate::BitWriter<'a, EEFC_FMR_SPEC, O>;
#[doc = "Field `CLOE` reader - Code Loop Optimization Enable"]
pub type CLOE_R = crate::BitReader;
#[doc = "Field `CLOE` writer - Code Loop Optimization Enable"]
pub type CLOE_W<'a, const O: u8> = crate::BitWriter<'a, EEFC_FMR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Sequential Code Optimization Disable"]
    #[inline(always)]
    pub fn scod(&self) -> SCOD_R {
        SCOD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 26 - Code Loop Optimization Enable"]
    #[inline(always)]
    pub fn cloe(&self) -> CLOE_R {
        CLOE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frdy(&mut self) -> FRDY_W<0> {
        FRDY_W::new(self)
    }
    #[doc = "Bits 8:11 - Flash Wait State"]
    #[inline(always)]
    #[must_use]
    pub fn fws(&mut self) -> FWS_W<8> {
        FWS_W::new(self)
    }
    #[doc = "Bit 16 - Sequential Code Optimization Disable"]
    #[inline(always)]
    #[must_use]
    pub fn scod(&mut self) -> SCOD_W<16> {
        SCOD_W::new(self)
    }
    #[doc = "Bit 26 - Code Loop Optimization Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cloe(&mut self) -> CLOE_W<26> {
        CLOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEFC Flash Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefc_fmr](index.html) module"]
pub struct EEFC_FMR_SPEC;
impl crate::RegisterSpec for EEFC_FMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eefc_fmr::R](R) reader structure"]
impl crate::Readable for EEFC_FMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eefc_fmr::W](W) writer structure"]
impl crate::Writable for EEFC_FMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEFC_FMR to value 0"]
impl crate::Resettable for EEFC_FMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
