#[doc = "Register `SLPWK_SR1` reader"]
pub struct R(crate::R<SLPWK_SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLPWK_SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLPWK_SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLPWK_SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PID32` reader - Peripheral 32 SleepWalking Status"]
pub type PID32_R = crate::BitReader;
#[doc = "Field `PID33` reader - Peripheral 33 SleepWalking Status"]
pub type PID33_R = crate::BitReader;
#[doc = "Field `PID34` reader - Peripheral 34 SleepWalking Status"]
pub type PID34_R = crate::BitReader;
#[doc = "Field `PID35` reader - Peripheral 35 SleepWalking Status"]
pub type PID35_R = crate::BitReader;
#[doc = "Field `PID37` reader - Peripheral 37 SleepWalking Status"]
pub type PID37_R = crate::BitReader;
#[doc = "Field `PID39` reader - Peripheral 39 SleepWalking Status"]
pub type PID39_R = crate::BitReader;
#[doc = "Field `PID40` reader - Peripheral 40 SleepWalking Status"]
pub type PID40_R = crate::BitReader;
#[doc = "Field `PID41` reader - Peripheral 41 SleepWalking Status"]
pub type PID41_R = crate::BitReader;
#[doc = "Field `PID42` reader - Peripheral 42 SleepWalking Status"]
pub type PID42_R = crate::BitReader;
#[doc = "Field `PID43` reader - Peripheral 43 SleepWalking Status"]
pub type PID43_R = crate::BitReader;
#[doc = "Field `PID44` reader - Peripheral 44 SleepWalking Status"]
pub type PID44_R = crate::BitReader;
#[doc = "Field `PID45` reader - Peripheral 45 SleepWalking Status"]
pub type PID45_R = crate::BitReader;
#[doc = "Field `PID46` reader - Peripheral 46 SleepWalking Status"]
pub type PID46_R = crate::BitReader;
#[doc = "Field `PID47` reader - Peripheral 47 SleepWalking Status"]
pub type PID47_R = crate::BitReader;
#[doc = "Field `PID48` reader - Peripheral 48 SleepWalking Status"]
pub type PID48_R = crate::BitReader;
#[doc = "Field `PID49` reader - Peripheral 49 SleepWalking Status"]
pub type PID49_R = crate::BitReader;
#[doc = "Field `PID50` reader - Peripheral 50 SleepWalking Status"]
pub type PID50_R = crate::BitReader;
#[doc = "Field `PID51` reader - Peripheral 51 SleepWalking Status"]
pub type PID51_R = crate::BitReader;
#[doc = "Field `PID52` reader - Peripheral 52 SleepWalking Status"]
pub type PID52_R = crate::BitReader;
#[doc = "Field `PID53` reader - Peripheral 53 SleepWalking Status"]
pub type PID53_R = crate::BitReader;
#[doc = "Field `PID56` reader - Peripheral 56 SleepWalking Status"]
pub type PID56_R = crate::BitReader;
#[doc = "Field `PID57` reader - Peripheral 57 SleepWalking Status"]
pub type PID57_R = crate::BitReader;
#[doc = "Field `PID58` reader - Peripheral 58 SleepWalking Status"]
pub type PID58_R = crate::BitReader;
#[doc = "Field `PID59` reader - Peripheral 59 SleepWalking Status"]
pub type PID59_R = crate::BitReader;
#[doc = "Field `PID60` reader - Peripheral 60 SleepWalking Status"]
pub type PID60_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Peripheral 32 SleepWalking Status"]
    #[inline(always)]
    pub fn pid32(&self) -> PID32_R {
        PID32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral 33 SleepWalking Status"]
    #[inline(always)]
    pub fn pid33(&self) -> PID33_R {
        PID33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Peripheral 34 SleepWalking Status"]
    #[inline(always)]
    pub fn pid34(&self) -> PID34_R {
        PID34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral 35 SleepWalking Status"]
    #[inline(always)]
    pub fn pid35(&self) -> PID35_R {
        PID35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral 37 SleepWalking Status"]
    #[inline(always)]
    pub fn pid37(&self) -> PID37_R {
        PID37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral 39 SleepWalking Status"]
    #[inline(always)]
    pub fn pid39(&self) -> PID39_R {
        PID39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral 40 SleepWalking Status"]
    #[inline(always)]
    pub fn pid40(&self) -> PID40_R {
        PID40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral 41 SleepWalking Status"]
    #[inline(always)]
    pub fn pid41(&self) -> PID41_R {
        PID41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral 42 SleepWalking Status"]
    #[inline(always)]
    pub fn pid42(&self) -> PID42_R {
        PID42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral 43 SleepWalking Status"]
    #[inline(always)]
    pub fn pid43(&self) -> PID43_R {
        PID43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral 44 SleepWalking Status"]
    #[inline(always)]
    pub fn pid44(&self) -> PID44_R {
        PID44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral 45 SleepWalking Status"]
    #[inline(always)]
    pub fn pid45(&self) -> PID45_R {
        PID45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral 46 SleepWalking Status"]
    #[inline(always)]
    pub fn pid46(&self) -> PID46_R {
        PID46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral 47 SleepWalking Status"]
    #[inline(always)]
    pub fn pid47(&self) -> PID47_R {
        PID47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral 48 SleepWalking Status"]
    #[inline(always)]
    pub fn pid48(&self) -> PID48_R {
        PID48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Peripheral 49 SleepWalking Status"]
    #[inline(always)]
    pub fn pid49(&self) -> PID49_R {
        PID49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral 50 SleepWalking Status"]
    #[inline(always)]
    pub fn pid50(&self) -> PID50_R {
        PID50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Peripheral 51 SleepWalking Status"]
    #[inline(always)]
    pub fn pid51(&self) -> PID51_R {
        PID51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral 52 SleepWalking Status"]
    #[inline(always)]
    pub fn pid52(&self) -> PID52_R {
        PID52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral 53 SleepWalking Status"]
    #[inline(always)]
    pub fn pid53(&self) -> PID53_R {
        PID53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Peripheral 56 SleepWalking Status"]
    #[inline(always)]
    pub fn pid56(&self) -> PID56_R {
        PID56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral 57 SleepWalking Status"]
    #[inline(always)]
    pub fn pid57(&self) -> PID57_R {
        PID57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Peripheral 58 SleepWalking Status"]
    #[inline(always)]
    pub fn pid58(&self) -> PID58_R {
        PID58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peripheral 59 SleepWalking Status"]
    #[inline(always)]
    pub fn pid59(&self) -> PID59_R {
        PID59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Peripheral 60 SleepWalking Status"]
    #[inline(always)]
    pub fn pid60(&self) -> PID60_R {
        PID60_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "SleepWalking Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_sr1](index.html) module"]
pub struct SLPWK_SR1_SPEC;
impl crate::RegisterSpec for SLPWK_SR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slpwk_sr1::R](R) reader structure"]
impl crate::Readable for SLPWK_SR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLPWK_SR1 to value 0"]
impl crate::Resettable for SLPWK_SR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
