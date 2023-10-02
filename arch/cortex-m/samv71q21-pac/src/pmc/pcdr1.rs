#[doc = "Register `PCDR1` writer"]
pub type W = crate::W<PCDR1_SPEC>;
#[doc = "Field `PID32` writer - Peripheral Clock 32 Disable"]
pub type PID32_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID33` writer - Peripheral Clock 33 Disable"]
pub type PID33_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID34` writer - Peripheral Clock 34 Disable"]
pub type PID34_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID35` writer - Peripheral Clock 35 Disable"]
pub type PID35_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID37` writer - Peripheral Clock 37 Disable"]
pub type PID37_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID39` writer - Peripheral Clock 39 Disable"]
pub type PID39_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID40` writer - Peripheral Clock 40 Disable"]
pub type PID40_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID41` writer - Peripheral Clock 41 Disable"]
pub type PID41_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID42` writer - Peripheral Clock 42 Disable"]
pub type PID42_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID43` writer - Peripheral Clock 43 Disable"]
pub type PID43_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID44` writer - Peripheral Clock 44 Disable"]
pub type PID44_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID45` writer - Peripheral Clock 45 Disable"]
pub type PID45_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID46` writer - Peripheral Clock 46 Disable"]
pub type PID46_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID47` writer - Peripheral Clock 47 Disable"]
pub type PID47_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID48` writer - Peripheral Clock 48 Disable"]
pub type PID48_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID49` writer - Peripheral Clock 49 Disable"]
pub type PID49_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID50` writer - Peripheral Clock 50 Disable"]
pub type PID50_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID51` writer - Peripheral Clock 51 Disable"]
pub type PID51_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID52` writer - Peripheral Clock 52 Disable"]
pub type PID52_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID53` writer - Peripheral Clock 53 Disable"]
pub type PID53_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID56` writer - Peripheral Clock 56 Disable"]
pub type PID56_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID57` writer - Peripheral Clock 57 Disable"]
pub type PID57_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID58` writer - Peripheral Clock 58 Disable"]
pub type PID58_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID59` writer - Peripheral Clock 59 Disable"]
pub type PID59_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID60` writer - Peripheral Clock 60 Disable"]
pub type PID60_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Peripheral Clock 32 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid32(&mut self) -> PID32_W<PCDR1_SPEC, 0> {
        PID32_W::new(self)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid33(&mut self) -> PID33_W<PCDR1_SPEC, 1> {
        PID33_W::new(self)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid34(&mut self) -> PID34_W<PCDR1_SPEC, 2> {
        PID34_W::new(self)
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid35(&mut self) -> PID35_W<PCDR1_SPEC, 3> {
        PID35_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid37(&mut self) -> PID37_W<PCDR1_SPEC, 5> {
        PID37_W::new(self)
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid39(&mut self) -> PID39_W<PCDR1_SPEC, 7> {
        PID39_W::new(self)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid40(&mut self) -> PID40_W<PCDR1_SPEC, 8> {
        PID40_W::new(self)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid41(&mut self) -> PID41_W<PCDR1_SPEC, 9> {
        PID41_W::new(self)
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid42(&mut self) -> PID42_W<PCDR1_SPEC, 10> {
        PID42_W::new(self)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid43(&mut self) -> PID43_W<PCDR1_SPEC, 11> {
        PID43_W::new(self)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid44(&mut self) -> PID44_W<PCDR1_SPEC, 12> {
        PID44_W::new(self)
    }
    #[doc = "Bit 13 - Peripheral Clock 45 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid45(&mut self) -> PID45_W<PCDR1_SPEC, 13> {
        PID45_W::new(self)
    }
    #[doc = "Bit 14 - Peripheral Clock 46 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid46(&mut self) -> PID46_W<PCDR1_SPEC, 14> {
        PID46_W::new(self)
    }
    #[doc = "Bit 15 - Peripheral Clock 47 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid47(&mut self) -> PID47_W<PCDR1_SPEC, 15> {
        PID47_W::new(self)
    }
    #[doc = "Bit 16 - Peripheral Clock 48 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid48(&mut self) -> PID48_W<PCDR1_SPEC, 16> {
        PID48_W::new(self)
    }
    #[doc = "Bit 17 - Peripheral Clock 49 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid49(&mut self) -> PID49_W<PCDR1_SPEC, 17> {
        PID49_W::new(self)
    }
    #[doc = "Bit 18 - Peripheral Clock 50 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid50(&mut self) -> PID50_W<PCDR1_SPEC, 18> {
        PID50_W::new(self)
    }
    #[doc = "Bit 19 - Peripheral Clock 51 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid51(&mut self) -> PID51_W<PCDR1_SPEC, 19> {
        PID51_W::new(self)
    }
    #[doc = "Bit 20 - Peripheral Clock 52 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid52(&mut self) -> PID52_W<PCDR1_SPEC, 20> {
        PID52_W::new(self)
    }
    #[doc = "Bit 21 - Peripheral Clock 53 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid53(&mut self) -> PID53_W<PCDR1_SPEC, 21> {
        PID53_W::new(self)
    }
    #[doc = "Bit 24 - Peripheral Clock 56 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid56(&mut self) -> PID56_W<PCDR1_SPEC, 24> {
        PID56_W::new(self)
    }
    #[doc = "Bit 25 - Peripheral Clock 57 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid57(&mut self) -> PID57_W<PCDR1_SPEC, 25> {
        PID57_W::new(self)
    }
    #[doc = "Bit 26 - Peripheral Clock 58 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid58(&mut self) -> PID58_W<PCDR1_SPEC, 26> {
        PID58_W::new(self)
    }
    #[doc = "Bit 27 - Peripheral Clock 59 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid59(&mut self) -> PID59_W<PCDR1_SPEC, 27> {
        PID59_W::new(self)
    }
    #[doc = "Bit 28 - Peripheral Clock 60 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid60(&mut self) -> PID60_W<PCDR1_SPEC, 28> {
        PID60_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral Clock Disable Register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCDR1_SPEC;
impl crate::RegisterSpec for PCDR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pcdr1::W`](W) writer structure"]
impl crate::Writable for PCDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDR1 to value 0"]
impl crate::Resettable for PCDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
