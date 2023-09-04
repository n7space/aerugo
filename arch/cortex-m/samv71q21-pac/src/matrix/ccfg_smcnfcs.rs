#[doc = "Register `CCFG_SMCNFCS` reader"]
pub type R = crate::R<CCFG_SMCNFCS_SPEC>;
#[doc = "Register `CCFG_SMCNFCS` writer"]
pub type W = crate::W<CCFG_SMCNFCS_SPEC>;
#[doc = "Field `SMC_NFCS0` reader - SMC NAND Flash Chip Select 0 Assignment"]
pub type SMC_NFCS0_R = crate::BitReader;
#[doc = "Field `SMC_NFCS0` writer - SMC NAND Flash Chip Select 0 Assignment"]
pub type SMC_NFCS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMC_NFCS1` reader - SMC NAND Flash Chip Select 1 Assignment"]
pub type SMC_NFCS1_R = crate::BitReader;
#[doc = "Field `SMC_NFCS1` writer - SMC NAND Flash Chip Select 1 Assignment"]
pub type SMC_NFCS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMC_NFCS2` reader - SMC NAND Flash Chip Select 2 Assignment"]
pub type SMC_NFCS2_R = crate::BitReader;
#[doc = "Field `SMC_NFCS2` writer - SMC NAND Flash Chip Select 2 Assignment"]
pub type SMC_NFCS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMC_NFCS3` reader - SMC NAND Flash Chip Select 3 Assignment"]
pub type SMC_NFCS3_R = crate::BitReader;
#[doc = "Field `SMC_NFCS3` writer - SMC NAND Flash Chip Select 3 Assignment"]
pub type SMC_NFCS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn smc_nfcs0(&mut self) -> SMC_NFCS0_W<CCFG_SMCNFCS_SPEC, 0> {
        SMC_NFCS0_W::new(self)
    }
    #[doc = "Bit 1 - SMC NAND Flash Chip Select 1 Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn smc_nfcs1(&mut self) -> SMC_NFCS1_W<CCFG_SMCNFCS_SPEC, 1> {
        SMC_NFCS1_W::new(self)
    }
    #[doc = "Bit 2 - SMC NAND Flash Chip Select 2 Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn smc_nfcs2(&mut self) -> SMC_NFCS2_W<CCFG_SMCNFCS_SPEC, 2> {
        SMC_NFCS2_W::new(self)
    }
    #[doc = "Bit 3 - SMC NAND Flash Chip Select 3 Assignment"]
    #[inline(always)]
    #[must_use]
    pub fn smc_nfcs3(&mut self) -> SMC_NFCS3_W<CCFG_SMCNFCS_SPEC, 3> {
        SMC_NFCS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SMC NAND Flash Chip Select Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg_smcnfcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg_smcnfcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCFG_SMCNFCS_SPEC;
impl crate::RegisterSpec for CCFG_SMCNFCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg_smcnfcs::R`](R) reader structure"]
impl crate::Readable for CCFG_SMCNFCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccfg_smcnfcs::W`](W) writer structure"]
impl crate::Writable for CCFG_SMCNFCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_SMCNFCS to value 0"]
impl crate::Resettable for CCFG_SMCNFCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
