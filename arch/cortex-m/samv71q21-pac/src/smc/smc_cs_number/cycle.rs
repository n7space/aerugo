#[doc = "Register `CYCLE` reader"]
pub struct R(crate::R<CYCLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CYCLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CYCLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CYCLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CYCLE` writer"]
pub struct W(crate::W<CYCLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CYCLE_SPEC>;
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
impl From<crate::W<CYCLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CYCLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NWE_CYCLE` reader - Total Write Cycle Length"]
pub type NWE_CYCLE_R = crate::FieldReader<u16>;
#[doc = "Field `NWE_CYCLE` writer - Total Write Cycle Length"]
pub type NWE_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, CYCLE_SPEC, 9, O, u16>;
#[doc = "Field `NRD_CYCLE` reader - Total Read Cycle Length"]
pub type NRD_CYCLE_R = crate::FieldReader<u16>;
#[doc = "Field `NRD_CYCLE` writer - Total Read Cycle Length"]
pub type NRD_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, CYCLE_SPEC, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    pub fn nwe_cycle(&self) -> NWE_CYCLE_R {
        NWE_CYCLE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    pub fn nrd_cycle(&self) -> NRD_CYCLE_R {
        NRD_CYCLE_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    #[must_use]
    pub fn nwe_cycle(&mut self) -> NWE_CYCLE_W<0> {
        NWE_CYCLE_W::new(self)
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    #[must_use]
    pub fn nrd_cycle(&mut self) -> NRD_CYCLE_W<16> {
        NRD_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cycle](index.html) module"]
pub struct CYCLE_SPEC;
impl crate::RegisterSpec for CYCLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cycle::R](R) reader structure"]
impl crate::Readable for CYCLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cycle::W](W) writer structure"]
impl crate::Writable for CYCLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CYCLE to value 0"]
impl crate::Resettable for CYCLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
