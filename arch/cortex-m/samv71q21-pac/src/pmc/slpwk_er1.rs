#[doc = "Register `SLPWK_ER1` writer"]
pub struct W(crate::W<SLPWK_ER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLPWK_ER1_SPEC>;
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
impl From<crate::W<SLPWK_ER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLPWK_ER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID32` writer - Peripheral 32 SleepWalking Enable"]
pub type PID32_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID33` writer - Peripheral 33 SleepWalking Enable"]
pub type PID33_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID34` writer - Peripheral 34 SleepWalking Enable"]
pub type PID34_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID35` writer - Peripheral 35 SleepWalking Enable"]
pub type PID35_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID37` writer - Peripheral 37 SleepWalking Enable"]
pub type PID37_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID39` writer - Peripheral 39 SleepWalking Enable"]
pub type PID39_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID40` writer - Peripheral 40 SleepWalking Enable"]
pub type PID40_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID41` writer - Peripheral 41 SleepWalking Enable"]
pub type PID41_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID42` writer - Peripheral 42 SleepWalking Enable"]
pub type PID42_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID43` writer - Peripheral 43 SleepWalking Enable"]
pub type PID43_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID44` writer - Peripheral 44 SleepWalking Enable"]
pub type PID44_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID45` writer - Peripheral 45 SleepWalking Enable"]
pub type PID45_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID46` writer - Peripheral 46 SleepWalking Enable"]
pub type PID46_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID47` writer - Peripheral 47 SleepWalking Enable"]
pub type PID47_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID48` writer - Peripheral 48 SleepWalking Enable"]
pub type PID48_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID49` writer - Peripheral 49 SleepWalking Enable"]
pub type PID49_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID50` writer - Peripheral 50 SleepWalking Enable"]
pub type PID50_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID51` writer - Peripheral 51 SleepWalking Enable"]
pub type PID51_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID52` writer - Peripheral 52 SleepWalking Enable"]
pub type PID52_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID53` writer - Peripheral 53 SleepWalking Enable"]
pub type PID53_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID56` writer - Peripheral 56 SleepWalking Enable"]
pub type PID56_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID57` writer - Peripheral 57 SleepWalking Enable"]
pub type PID57_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID58` writer - Peripheral 58 SleepWalking Enable"]
pub type PID58_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID59` writer - Peripheral 59 SleepWalking Enable"]
pub type PID59_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
#[doc = "Field `PID60` writer - Peripheral 60 SleepWalking Enable"]
pub type PID60_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER1_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Peripheral 32 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid32(&mut self) -> PID32_W<0> {
        PID32_W::new(self)
    }
    #[doc = "Bit 1 - Peripheral 33 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid33(&mut self) -> PID33_W<1> {
        PID33_W::new(self)
    }
    #[doc = "Bit 2 - Peripheral 34 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid34(&mut self) -> PID34_W<2> {
        PID34_W::new(self)
    }
    #[doc = "Bit 3 - Peripheral 35 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid35(&mut self) -> PID35_W<3> {
        PID35_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral 37 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid37(&mut self) -> PID37_W<5> {
        PID37_W::new(self)
    }
    #[doc = "Bit 7 - Peripheral 39 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid39(&mut self) -> PID39_W<7> {
        PID39_W::new(self)
    }
    #[doc = "Bit 8 - Peripheral 40 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid40(&mut self) -> PID40_W<8> {
        PID40_W::new(self)
    }
    #[doc = "Bit 9 - Peripheral 41 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid41(&mut self) -> PID41_W<9> {
        PID41_W::new(self)
    }
    #[doc = "Bit 10 - Peripheral 42 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid42(&mut self) -> PID42_W<10> {
        PID42_W::new(self)
    }
    #[doc = "Bit 11 - Peripheral 43 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid43(&mut self) -> PID43_W<11> {
        PID43_W::new(self)
    }
    #[doc = "Bit 12 - Peripheral 44 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid44(&mut self) -> PID44_W<12> {
        PID44_W::new(self)
    }
    #[doc = "Bit 13 - Peripheral 45 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid45(&mut self) -> PID45_W<13> {
        PID45_W::new(self)
    }
    #[doc = "Bit 14 - Peripheral 46 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid46(&mut self) -> PID46_W<14> {
        PID46_W::new(self)
    }
    #[doc = "Bit 15 - Peripheral 47 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid47(&mut self) -> PID47_W<15> {
        PID47_W::new(self)
    }
    #[doc = "Bit 16 - Peripheral 48 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid48(&mut self) -> PID48_W<16> {
        PID48_W::new(self)
    }
    #[doc = "Bit 17 - Peripheral 49 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid49(&mut self) -> PID49_W<17> {
        PID49_W::new(self)
    }
    #[doc = "Bit 18 - Peripheral 50 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid50(&mut self) -> PID50_W<18> {
        PID50_W::new(self)
    }
    #[doc = "Bit 19 - Peripheral 51 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid51(&mut self) -> PID51_W<19> {
        PID51_W::new(self)
    }
    #[doc = "Bit 20 - Peripheral 52 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid52(&mut self) -> PID52_W<20> {
        PID52_W::new(self)
    }
    #[doc = "Bit 21 - Peripheral 53 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid53(&mut self) -> PID53_W<21> {
        PID53_W::new(self)
    }
    #[doc = "Bit 24 - Peripheral 56 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid56(&mut self) -> PID56_W<24> {
        PID56_W::new(self)
    }
    #[doc = "Bit 25 - Peripheral 57 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid57(&mut self) -> PID57_W<25> {
        PID57_W::new(self)
    }
    #[doc = "Bit 26 - Peripheral 58 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid58(&mut self) -> PID58_W<26> {
        PID58_W::new(self)
    }
    #[doc = "Bit 27 - Peripheral 59 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid59(&mut self) -> PID59_W<27> {
        PID59_W::new(self)
    }
    #[doc = "Bit 28 - Peripheral 60 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid60(&mut self) -> PID60_W<28> {
        PID60_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SleepWalking Enable Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_er1](index.html) module"]
pub struct SLPWK_ER1_SPEC;
impl crate::RegisterSpec for SLPWK_ER1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [slpwk_er1::W](W) writer structure"]
impl crate::Writable for SLPWK_ER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLPWK_ER1 to value 0"]
impl crate::Resettable for SLPWK_ER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
