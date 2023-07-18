#[doc = "Register `SETUP` reader"]
pub struct R(crate::R<SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUP` writer"]
pub struct W(crate::W<SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUP_SPEC>;
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
impl From<crate::W<SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NWE_SETUP` reader - NWE Setup Length"]
pub type NWE_SETUP_R = crate::FieldReader;
#[doc = "Field `NWE_SETUP` writer - NWE Setup Length"]
pub type NWE_SETUP_W<'a, const O: u8> = crate::FieldWriter<'a, SETUP_SPEC, 6, O>;
#[doc = "Field `NCS_WR_SETUP` reader - NCS Setup Length in WRITE Access"]
pub type NCS_WR_SETUP_R = crate::FieldReader;
#[doc = "Field `NCS_WR_SETUP` writer - NCS Setup Length in WRITE Access"]
pub type NCS_WR_SETUP_W<'a, const O: u8> = crate::FieldWriter<'a, SETUP_SPEC, 6, O>;
#[doc = "Field `NRD_SETUP` reader - NRD Setup Length"]
pub type NRD_SETUP_R = crate::FieldReader;
#[doc = "Field `NRD_SETUP` writer - NRD Setup Length"]
pub type NRD_SETUP_W<'a, const O: u8> = crate::FieldWriter<'a, SETUP_SPEC, 6, O>;
#[doc = "Field `NCS_RD_SETUP` reader - NCS Setup Length in READ Access"]
pub type NCS_RD_SETUP_R = crate::FieldReader;
#[doc = "Field `NCS_RD_SETUP` writer - NCS Setup Length in READ Access"]
pub type NCS_RD_SETUP_W<'a, const O: u8> = crate::FieldWriter<'a, SETUP_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:5 - NWE Setup Length"]
    #[inline(always)]
    pub fn nwe_setup(&self) -> NWE_SETUP_R {
        NWE_SETUP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - NCS Setup Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_setup(&self) -> NCS_WR_SETUP_R {
        NCS_WR_SETUP_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - NRD Setup Length"]
    #[inline(always)]
    pub fn nrd_setup(&self) -> NRD_SETUP_R {
        NRD_SETUP_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - NCS Setup Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_setup(&self) -> NCS_RD_SETUP_R {
        NCS_RD_SETUP_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - NWE Setup Length"]
    #[inline(always)]
    #[must_use]
    pub fn nwe_setup(&mut self) -> NWE_SETUP_W<0> {
        NWE_SETUP_W::new(self)
    }
    #[doc = "Bits 8:13 - NCS Setup Length in WRITE Access"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_wr_setup(&mut self) -> NCS_WR_SETUP_W<8> {
        NCS_WR_SETUP_W::new(self)
    }
    #[doc = "Bits 16:21 - NRD Setup Length"]
    #[inline(always)]
    #[must_use]
    pub fn nrd_setup(&mut self) -> NRD_SETUP_W<16> {
        NRD_SETUP_W::new(self)
    }
    #[doc = "Bits 24:29 - NCS Setup Length in READ Access"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_rd_setup(&mut self) -> NCS_RD_SETUP_W<24> {
        NCS_RD_SETUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup](index.html) module"]
pub struct SETUP_SPEC;
impl crate::RegisterSpec for SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setup::R](R) reader structure"]
impl crate::Readable for SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setup::W](W) writer structure"]
impl crate::Writable for SETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETUP to value 0"]
impl crate::Resettable for SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
