#[doc = "Register `CCFG_SMCNFCS` reader"]
pub struct R(crate::R<CCFG_SMCNFCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_SMCNFCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_SMCNFCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_SMCNFCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_SMCNFCS` writer"]
pub struct W(crate::W<CCFG_SMCNFCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_SMCNFCS_SPEC>;
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
impl From<crate::W<CCFG_SMCNFCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_SMCNFCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMC_NFCS0` reader - SMC NAND Flash Chip Select 0 Assignment"]
pub type SMC_NFCS0_R = crate::BitReader;
#[doc = "Field `SMC_NFCS0` writer - SMC NAND Flash Chip Select 0 Assignment"]
pub type SMC_NFCS0_W<'a, const O: u8> = crate::BitWriter<'a, CCFG_SMCNFCS_SPEC, O>;
#[doc = "Field `SMC_NFCS1` reader - SMC NAND Flash Chip Select 1 Assignment"]
pub type SMC_NFCS1_R = crate::BitReader;
#[doc = "Field `SMC_NFCS1` writer - SMC NAND Flash Chip Select 1 Assignment"]
pub type SMC_NFCS1_W<'a, const O: u8> = crate::BitWriter<'a, CCFG_SMCNFCS_SPEC, O>;
#[doc = "Field `SMC_NFCS2` reader - SMC NAND Flash Chip Select 2 Assignment"]
pub type SMC_NFCS2_R = crate::BitReader;
#[doc = "Field `SMC_NFCS2` writer - SMC NAND Flash Chip Select 2 Assignment"]
pub type SMC_NFCS2_W<'a, const O: u8> = crate::BitWriter<'a, CCFG_SMCNFCS_SPEC, O>;
#[doc = "Field `SMC_NFCS3` reader - SMC NAND Flash Chip Select 3 Assignment"]
pub type SMC_NFCS3_R = crate::BitReader;
#[doc = "Field `SMC_NFCS3` writer - SMC NAND Flash Chip Select 3 Assignment"]
pub type SMC_NFCS3_W<'a, const O: u8> = crate::BitWriter<'a, CCFG_SMCNFCS_SPEC, O>;
impl R {
    #[doc = "Bit 0 - SMC NAND Flash Chip Select 0 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs0(&self) -> SMC_NFCS0_R {
        SMC_NFCS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMC NAND Flash Chip Select 1 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs1(&self) -> SMC_NFCS1_R {
        SMC_NFCS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMC NAND Flash Chip Select 2 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs2(&self) -> SMC_NFCS2_R {
        SMC_NFCS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMC NAND Flash Chip Select 3 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs3(&self) -> SMC_NFCS3_R {
        SMC_NFCS3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SMC NAND Flash Chip Select 0 Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn smc_nfcs0(&mut self) -> SMC_NFCS0_W<0> {
        SMC_NFCS0_W::new(self)
    }
    #[doc = "Bit 1 - SMC NAND Flash Chip Select 1 Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn smc_nfcs1(&mut self) -> SMC_NFCS1_W<1> {
        SMC_NFCS1_W::new(self)
    }
    #[doc = "Bit 2 - SMC NAND Flash Chip Select 2 Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn smc_nfcs2(&mut self) -> SMC_NFCS2_W<2> {
        SMC_NFCS2_W::new(self)
    }
    #[doc = "Bit 3 - SMC NAND Flash Chip Select 3 Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn smc_nfcs3(&mut self) -> SMC_NFCS3_W<3> {
        SMC_NFCS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC NAND Flash Chip Select Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_smcnfcs](index.html) module"]
pub struct CCFG_SMCNFCS_SPEC;
impl crate::RegisterSpec for CCFG_SMCNFCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_smcnfcs::R](R) reader structure"]
impl crate::Readable for CCFG_SMCNFCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_smcnfcs::W](W) writer structure"]
impl crate::Writable for CCFG_SMCNFCS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_SMCNFCS to value 0"]
impl crate::Resettable for CCFG_SMCNFCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
