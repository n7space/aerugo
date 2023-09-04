#[doc = "Register `SLPWK_AIPR` reader"]
pub type R = crate::R<SLPWK_AIPR_SPEC>;
#[doc = "Field `AIP` reader - Activity In Progress"]
pub type AIP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Activity In Progress"]
    #[inline(always)]
    pub fn aip(&self) -> AIP_R {
        AIP_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SleepWalking Activity In Progress Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slpwk_aipr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLPWK_AIPR_SPEC;
impl crate::RegisterSpec for SLPWK_AIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slpwk_aipr::R`](R) reader structure"]
impl crate::Readable for SLPWK_AIPR_SPEC {}
#[doc = "`reset()` method sets SLPWK_AIPR to value 0"]
impl crate::Resettable for SLPWK_AIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
