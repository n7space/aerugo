#[doc = "Register `CMPM` reader"]
pub type R = crate::R<CMPM_SPEC>;
#[doc = "Register `CMPM` writer"]
pub type W = crate::W<CMPM_SPEC>;
#[doc = "Field `CEN` reader - Comparison x Enable"]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - Comparison x Enable"]
pub type CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTR` reader - Comparison x Trigger"]
pub type CTR_R = crate::FieldReader;
#[doc = "Field `CTR` writer - Comparison x Trigger"]
pub type CTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CPR` reader - Comparison x Period"]
pub type CPR_R = crate::FieldReader;
#[doc = "Field `CPR` writer - Comparison x Period"]
pub type CPR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CPRCNT` reader - Comparison x Period Counter"]
pub type CPRCNT_R = crate::FieldReader;
#[doc = "Field `CPRCNT` writer - Comparison x Period Counter"]
pub type CPRCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CUPR` reader - Comparison x Update Period"]
pub type CUPR_R = crate::FieldReader;
#[doc = "Field `CUPR` writer - Comparison x Update Period"]
pub type CUPR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CUPRCNT` reader - Comparison x Update Period Counter"]
pub type CUPRCNT_R = crate::FieldReader;
#[doc = "Field `CUPRCNT` writer - Comparison x Update Period Counter"]
pub type CUPRCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Comparison x Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Comparison x Period"]
    #[inline(always)]
    pub fn cpr(&self) -> CPR_R {
        CPR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Comparison x Period Counter"]
    #[inline(always)]
    pub fn cprcnt(&self) -> CPRCNT_R {
        CPRCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period"]
    #[inline(always)]
    pub fn cupr(&self) -> CUPR_R {
        CUPR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Comparison x Update Period Counter"]
    #[inline(always)]
    pub fn cuprcnt(&self) -> CUPRCNT_R {
        CUPRCNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison x Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<CMPM_SPEC, 0> {
        CEN_W::new(self)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<CMPM_SPEC, 4> {
        CTR_W::new(self)
    }
    #[doc = "Bits 8:11 - Comparison x Period"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CPR_W<CMPM_SPEC, 8> {
        CPR_W::new(self)
    }
    #[doc = "Bits 12:15 - Comparison x Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cprcnt(&mut self) -> CPRCNT_W<CMPM_SPEC, 12> {
        CPRCNT_W::new(self)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period"]
    #[inline(always)]
    #[must_use]
    pub fn cupr(&mut self) -> CUPR_W<CMPM_SPEC, 16> {
        CUPR_W::new(self)
    }
    #[doc = "Bits 20:23 - Comparison x Update Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cuprcnt(&mut self) -> CUPRCNT_W<CMPM_SPEC, 20> {
        CUPRCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Comparison 0 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPM_SPEC;
impl crate::RegisterSpec for CMPM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpm::R`](R) reader structure"]
impl crate::Readable for CMPM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpm::W`](W) writer structure"]
impl crate::Writable for CMPM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPM to value 0"]
impl crate::Resettable for CMPM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
