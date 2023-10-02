#[doc = "Register `SLPWK_DR0` writer"]
pub type W = crate::W<SLPWK_DR0_SPEC>;
#[doc = "Field `PID7` writer - Peripheral 7 SleepWalking Disable"]
pub type PID7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID8` writer - Peripheral 8 SleepWalking Disable"]
pub type PID8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID9` writer - Peripheral 9 SleepWalking Disable"]
pub type PID9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID10` writer - Peripheral 10 SleepWalking Disable"]
pub type PID10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID11` writer - Peripheral 11 SleepWalking Disable"]
pub type PID11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID12` writer - Peripheral 12 SleepWalking Disable"]
pub type PID12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID13` writer - Peripheral 13 SleepWalking Disable"]
pub type PID13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID14` writer - Peripheral 14 SleepWalking Disable"]
pub type PID14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID15` writer - Peripheral 15 SleepWalking Disable"]
pub type PID15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID16` writer - Peripheral 16 SleepWalking Disable"]
pub type PID16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID17` writer - Peripheral 17 SleepWalking Disable"]
pub type PID17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID18` writer - Peripheral 18 SleepWalking Disable"]
pub type PID18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID19` writer - Peripheral 19 SleepWalking Disable"]
pub type PID19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID20` writer - Peripheral 20 SleepWalking Disable"]
pub type PID20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID21` writer - Peripheral 21 SleepWalking Disable"]
pub type PID21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID22` writer - Peripheral 22 SleepWalking Disable"]
pub type PID22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID23` writer - Peripheral 23 SleepWalking Disable"]
pub type PID23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID24` writer - Peripheral 24 SleepWalking Disable"]
pub type PID24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID25` writer - Peripheral 25 SleepWalking Disable"]
pub type PID25_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID26` writer - Peripheral 26 SleepWalking Disable"]
pub type PID26_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID27` writer - Peripheral 27 SleepWalking Disable"]
pub type PID27_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID28` writer - Peripheral 28 SleepWalking Disable"]
pub type PID28_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID29` writer - Peripheral 29 SleepWalking Disable"]
pub type PID29_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID30` writer - Peripheral 30 SleepWalking Disable"]
pub type PID30_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID31` writer - Peripheral 31 SleepWalking Disable"]
pub type PID31_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 7 - Peripheral 7 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid7(&mut self) -> PID7_W<SLPWK_DR0_SPEC, 7> {
        PID7_W::new(self)
    }
    #[doc = "Bit 8 - Peripheral 8 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid8(&mut self) -> PID8_W<SLPWK_DR0_SPEC, 8> {
        PID8_W::new(self)
    }
    #[doc = "Bit 9 - Peripheral 9 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid9(&mut self) -> PID9_W<SLPWK_DR0_SPEC, 9> {
        PID9_W::new(self)
    }
    #[doc = "Bit 10 - Peripheral 10 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid10(&mut self) -> PID10_W<SLPWK_DR0_SPEC, 10> {
        PID10_W::new(self)
    }
    #[doc = "Bit 11 - Peripheral 11 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid11(&mut self) -> PID11_W<SLPWK_DR0_SPEC, 11> {
        PID11_W::new(self)
    }
    #[doc = "Bit 12 - Peripheral 12 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid12(&mut self) -> PID12_W<SLPWK_DR0_SPEC, 12> {
        PID12_W::new(self)
    }
    #[doc = "Bit 13 - Peripheral 13 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid13(&mut self) -> PID13_W<SLPWK_DR0_SPEC, 13> {
        PID13_W::new(self)
    }
    #[doc = "Bit 14 - Peripheral 14 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid14(&mut self) -> PID14_W<SLPWK_DR0_SPEC, 14> {
        PID14_W::new(self)
    }
    #[doc = "Bit 15 - Peripheral 15 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid15(&mut self) -> PID15_W<SLPWK_DR0_SPEC, 15> {
        PID15_W::new(self)
    }
    #[doc = "Bit 16 - Peripheral 16 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid16(&mut self) -> PID16_W<SLPWK_DR0_SPEC, 16> {
        PID16_W::new(self)
    }
    #[doc = "Bit 17 - Peripheral 17 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid17(&mut self) -> PID17_W<SLPWK_DR0_SPEC, 17> {
        PID17_W::new(self)
    }
    #[doc = "Bit 18 - Peripheral 18 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid18(&mut self) -> PID18_W<SLPWK_DR0_SPEC, 18> {
        PID18_W::new(self)
    }
    #[doc = "Bit 19 - Peripheral 19 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid19(&mut self) -> PID19_W<SLPWK_DR0_SPEC, 19> {
        PID19_W::new(self)
    }
    #[doc = "Bit 20 - Peripheral 20 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid20(&mut self) -> PID20_W<SLPWK_DR0_SPEC, 20> {
        PID20_W::new(self)
    }
    #[doc = "Bit 21 - Peripheral 21 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid21(&mut self) -> PID21_W<SLPWK_DR0_SPEC, 21> {
        PID21_W::new(self)
    }
    #[doc = "Bit 22 - Peripheral 22 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid22(&mut self) -> PID22_W<SLPWK_DR0_SPEC, 22> {
        PID22_W::new(self)
    }
    #[doc = "Bit 23 - Peripheral 23 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid23(&mut self) -> PID23_W<SLPWK_DR0_SPEC, 23> {
        PID23_W::new(self)
    }
    #[doc = "Bit 24 - Peripheral 24 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid24(&mut self) -> PID24_W<SLPWK_DR0_SPEC, 24> {
        PID24_W::new(self)
    }
    #[doc = "Bit 25 - Peripheral 25 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid25(&mut self) -> PID25_W<SLPWK_DR0_SPEC, 25> {
        PID25_W::new(self)
    }
    #[doc = "Bit 26 - Peripheral 26 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid26(&mut self) -> PID26_W<SLPWK_DR0_SPEC, 26> {
        PID26_W::new(self)
    }
    #[doc = "Bit 27 - Peripheral 27 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid27(&mut self) -> PID27_W<SLPWK_DR0_SPEC, 27> {
        PID27_W::new(self)
    }
    #[doc = "Bit 28 - Peripheral 28 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid28(&mut self) -> PID28_W<SLPWK_DR0_SPEC, 28> {
        PID28_W::new(self)
    }
    #[doc = "Bit 29 - Peripheral 29 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid29(&mut self) -> PID29_W<SLPWK_DR0_SPEC, 29> {
        PID29_W::new(self)
    }
    #[doc = "Bit 30 - Peripheral 30 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid30(&mut self) -> PID30_W<SLPWK_DR0_SPEC, 30> {
        PID30_W::new(self)
    }
    #[doc = "Bit 31 - Peripheral 31 SleepWalking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pid31(&mut self) -> PID31_W<SLPWK_DR0_SPEC, 31> {
        PID31_W::new(self)
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
#[doc = "SleepWalking Disable Register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slpwk_dr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLPWK_DR0_SPEC;
impl crate::RegisterSpec for SLPWK_DR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slpwk_dr0::W`](W) writer structure"]
impl crate::Writable for SLPWK_DR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLPWK_DR0 to value 0"]
impl crate::Resettable for SLPWK_DR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
