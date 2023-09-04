#[doc = "Register `PULSE` reader"]
pub type R = crate::R<PULSE_SPEC>;
#[doc = "Register `PULSE` writer"]
pub type W = crate::W<PULSE_SPEC>;
#[doc = "Field `NWE_PULSE` reader - NWE Pulse Length"]
pub type NWE_PULSE_R = crate::FieldReader;
#[doc = "Field `NWE_PULSE` writer - NWE Pulse Length"]
pub type NWE_PULSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `NCS_WR_PULSE` reader - NCS Pulse Length in WRITE Access"]
pub type NCS_WR_PULSE_R = crate::FieldReader;
#[doc = "Field `NCS_WR_PULSE` writer - NCS Pulse Length in WRITE Access"]
pub type NCS_WR_PULSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `NRD_PULSE` reader - NRD Pulse Length"]
pub type NRD_PULSE_R = crate::FieldReader;
#[doc = "Field `NRD_PULSE` writer - NRD Pulse Length"]
pub type NRD_PULSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `NCS_RD_PULSE` reader - NCS Pulse Length in READ Access"]
pub type NCS_RD_PULSE_R = crate::FieldReader;
#[doc = "Field `NCS_RD_PULSE` writer - NCS Pulse Length in READ Access"]
pub type NCS_RD_PULSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - NWE Pulse Length"]
    #[inline(always)]
    pub fn nwe_pulse(&self) -> NWE_PULSE_R {
        NWE_PULSE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - NCS Pulse Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_pulse(&self) -> NCS_WR_PULSE_R {
        NCS_WR_PULSE_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - NRD Pulse Length"]
    #[inline(always)]
    pub fn nrd_pulse(&self) -> NRD_PULSE_R {
        NRD_PULSE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - NCS Pulse Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_pulse(&self) -> NCS_RD_PULSE_R {
        NCS_RD_PULSE_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - NWE Pulse Length"]
    #[inline(always)]
    #[must_use]
    pub fn nwe_pulse(&mut self) -> NWE_PULSE_W<PULSE_SPEC, 0> {
        NWE_PULSE_W::new(self)
    }
    #[doc = "Bits 8:14 - NCS Pulse Length in WRITE Access"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_wr_pulse(&mut self) -> NCS_WR_PULSE_W<PULSE_SPEC, 8> {
        NCS_WR_PULSE_W::new(self)
    }
    #[doc = "Bits 16:22 - NRD Pulse Length"]
    #[inline(always)]
    #[must_use]
    pub fn nrd_pulse(&mut self) -> NRD_PULSE_W<PULSE_SPEC, 16> {
        NRD_PULSE_W::new(self)
    }
    #[doc = "Bits 24:30 - NCS Pulse Length in READ Access"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_rd_pulse(&mut self) -> NCS_RD_PULSE_W<PULSE_SPEC, 24> {
        NCS_RD_PULSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SMC Pulse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PULSE_SPEC;
impl crate::RegisterSpec for PULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pulse::R`](R) reader structure"]
impl crate::Readable for PULSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pulse::W`](W) writer structure"]
impl crate::Writable for PULSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PULSE to value 0"]
impl crate::Resettable for PULSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
