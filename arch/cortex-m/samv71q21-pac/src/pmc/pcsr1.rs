#[doc = "Register `PCSR1` reader"]
pub struct R(crate::R<PCSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PID32` reader - Peripheral Clock 32 Status"]
pub type PID32_R = crate::BitReader;
#[doc = "Field `PID33` reader - Peripheral Clock 33 Status"]
pub type PID33_R = crate::BitReader;
#[doc = "Field `PID34` reader - Peripheral Clock 34 Status"]
pub type PID34_R = crate::BitReader;
#[doc = "Field `PID35` reader - Peripheral Clock 35 Status"]
pub type PID35_R = crate::BitReader;
#[doc = "Field `PID37` reader - Peripheral Clock 37 Status"]
pub type PID37_R = crate::BitReader;
#[doc = "Field `PID39` reader - Peripheral Clock 39 Status"]
pub type PID39_R = crate::BitReader;
#[doc = "Field `PID40` reader - Peripheral Clock 40 Status"]
pub type PID40_R = crate::BitReader;
#[doc = "Field `PID41` reader - Peripheral Clock 41 Status"]
pub type PID41_R = crate::BitReader;
#[doc = "Field `PID42` reader - Peripheral Clock 42 Status"]
pub type PID42_R = crate::BitReader;
#[doc = "Field `PID43` reader - Peripheral Clock 43 Status"]
pub type PID43_R = crate::BitReader;
#[doc = "Field `PID44` reader - Peripheral Clock 44 Status"]
pub type PID44_R = crate::BitReader;
#[doc = "Field `PID45` reader - Peripheral Clock 45 Status"]
pub type PID45_R = crate::BitReader;
#[doc = "Field `PID46` reader - Peripheral Clock 46 Status"]
pub type PID46_R = crate::BitReader;
#[doc = "Field `PID47` reader - Peripheral Clock 47 Status"]
pub type PID47_R = crate::BitReader;
#[doc = "Field `PID48` reader - Peripheral Clock 48 Status"]
pub type PID48_R = crate::BitReader;
#[doc = "Field `PID49` reader - Peripheral Clock 49 Status"]
pub type PID49_R = crate::BitReader;
#[doc = "Field `PID50` reader - Peripheral Clock 50 Status"]
pub type PID50_R = crate::BitReader;
#[doc = "Field `PID51` reader - Peripheral Clock 51 Status"]
pub type PID51_R = crate::BitReader;
#[doc = "Field `PID52` reader - Peripheral Clock 52 Status"]
pub type PID52_R = crate::BitReader;
#[doc = "Field `PID53` reader - Peripheral Clock 53 Status"]
pub type PID53_R = crate::BitReader;
#[doc = "Field `PID56` reader - Peripheral Clock 56 Status"]
pub type PID56_R = crate::BitReader;
#[doc = "Field `PID57` reader - Peripheral Clock 57 Status"]
pub type PID57_R = crate::BitReader;
#[doc = "Field `PID58` reader - Peripheral Clock 58 Status"]
pub type PID58_R = crate::BitReader;
#[doc = "Field `PID59` reader - Peripheral Clock 59 Status"]
pub type PID59_R = crate::BitReader;
#[doc = "Field `PID60` reader - Peripheral Clock 60 Status"]
pub type PID60_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Peripheral Clock 32 Status"]
    #[inline(always)]
    pub fn pid32(&self) -> PID32_R {
        PID32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Status"]
    #[inline(always)]
    pub fn pid33(&self) -> PID33_R {
        PID33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Status"]
    #[inline(always)]
    pub fn pid34(&self) -> PID34_R {
        PID34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Status"]
    #[inline(always)]
    pub fn pid35(&self) -> PID35_R {
        PID35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Status"]
    #[inline(always)]
    pub fn pid37(&self) -> PID37_R {
        PID37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Status"]
    #[inline(always)]
    pub fn pid39(&self) -> PID39_R {
        PID39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Status"]
    #[inline(always)]
    pub fn pid40(&self) -> PID40_R {
        PID40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Status"]
    #[inline(always)]
    pub fn pid41(&self) -> PID41_R {
        PID41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Status"]
    #[inline(always)]
    pub fn pid42(&self) -> PID42_R {
        PID42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Status"]
    #[inline(always)]
    pub fn pid43(&self) -> PID43_R {
        PID43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Status"]
    #[inline(always)]
    pub fn pid44(&self) -> PID44_R {
        PID44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral Clock 45 Status"]
    #[inline(always)]
    pub fn pid45(&self) -> PID45_R {
        PID45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral Clock 46 Status"]
    #[inline(always)]
    pub fn pid46(&self) -> PID46_R {
        PID46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral Clock 47 Status"]
    #[inline(always)]
    pub fn pid47(&self) -> PID47_R {
        PID47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral Clock 48 Status"]
    #[inline(always)]
    pub fn pid48(&self) -> PID48_R {
        PID48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Peripheral Clock 49 Status"]
    #[inline(always)]
    pub fn pid49(&self) -> PID49_R {
        PID49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral Clock 50 Status"]
    #[inline(always)]
    pub fn pid50(&self) -> PID50_R {
        PID50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Peripheral Clock 51 Status"]
    #[inline(always)]
    pub fn pid51(&self) -> PID51_R {
        PID51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral Clock 52 Status"]
    #[inline(always)]
    pub fn pid52(&self) -> PID52_R {
        PID52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral Clock 53 Status"]
    #[inline(always)]
    pub fn pid53(&self) -> PID53_R {
        PID53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Peripheral Clock 56 Status"]
    #[inline(always)]
    pub fn pid56(&self) -> PID56_R {
        PID56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral Clock 57 Status"]
    #[inline(always)]
    pub fn pid57(&self) -> PID57_R {
        PID57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Peripheral Clock 58 Status"]
    #[inline(always)]
    pub fn pid58(&self) -> PID58_R {
        PID58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peripheral Clock 59 Status"]
    #[inline(always)]
    pub fn pid59(&self) -> PID59_R {
        PID59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Peripheral Clock 60 Status"]
    #[inline(always)]
    pub fn pid60(&self) -> PID60_R {
        PID60_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Peripheral Clock Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsr1](index.html) module"]
pub struct PCSR1_SPEC;
impl crate::RegisterSpec for PCSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcsr1::R](R) reader structure"]
impl crate::Readable for PCSR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCSR1 to value 0"]
impl crate::Resettable for PCSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
