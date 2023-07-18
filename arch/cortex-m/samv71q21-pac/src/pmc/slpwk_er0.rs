#[doc = "Register `SLPWK_ER0` writer"]
pub struct W(crate::W<SLPWK_ER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLPWK_ER0_SPEC>;
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
impl From<crate::W<SLPWK_ER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLPWK_ER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID7` writer - Peripheral 7 SleepWalking Enable"]
pub type PID7_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID8` writer - Peripheral 8 SleepWalking Enable"]
pub type PID8_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID9` writer - Peripheral 9 SleepWalking Enable"]
pub type PID9_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID10` writer - Peripheral 10 SleepWalking Enable"]
pub type PID10_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID11` writer - Peripheral 11 SleepWalking Enable"]
pub type PID11_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID12` writer - Peripheral 12 SleepWalking Enable"]
pub type PID12_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID13` writer - Peripheral 13 SleepWalking Enable"]
pub type PID13_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID14` writer - Peripheral 14 SleepWalking Enable"]
pub type PID14_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID15` writer - Peripheral 15 SleepWalking Enable"]
pub type PID15_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID16` writer - Peripheral 16 SleepWalking Enable"]
pub type PID16_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID17` writer - Peripheral 17 SleepWalking Enable"]
pub type PID17_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID18` writer - Peripheral 18 SleepWalking Enable"]
pub type PID18_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID19` writer - Peripheral 19 SleepWalking Enable"]
pub type PID19_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID20` writer - Peripheral 20 SleepWalking Enable"]
pub type PID20_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID21` writer - Peripheral 21 SleepWalking Enable"]
pub type PID21_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID22` writer - Peripheral 22 SleepWalking Enable"]
pub type PID22_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID23` writer - Peripheral 23 SleepWalking Enable"]
pub type PID23_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID24` writer - Peripheral 24 SleepWalking Enable"]
pub type PID24_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID25` writer - Peripheral 25 SleepWalking Enable"]
pub type PID25_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID26` writer - Peripheral 26 SleepWalking Enable"]
pub type PID26_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID27` writer - Peripheral 27 SleepWalking Enable"]
pub type PID27_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID28` writer - Peripheral 28 SleepWalking Enable"]
pub type PID28_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID29` writer - Peripheral 29 SleepWalking Enable"]
pub type PID29_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID30` writer - Peripheral 30 SleepWalking Enable"]
pub type PID30_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
#[doc = "Field `PID31` writer - Peripheral 31 SleepWalking Enable"]
pub type PID31_W<'a, const O: u8> = crate::BitWriter<'a, SLPWK_ER0_SPEC, O>;
impl W {
    #[doc = "Bit 7 - Peripheral 7 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid7(&mut self) -> PID7_W<7> {
        PID7_W::new(self)
    }
    #[doc = "Bit 8 - Peripheral 8 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid8(&mut self) -> PID8_W<8> {
        PID8_W::new(self)
    }
    #[doc = "Bit 9 - Peripheral 9 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid9(&mut self) -> PID9_W<9> {
        PID9_W::new(self)
    }
    #[doc = "Bit 10 - Peripheral 10 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid10(&mut self) -> PID10_W<10> {
        PID10_W::new(self)
    }
    #[doc = "Bit 11 - Peripheral 11 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid11(&mut self) -> PID11_W<11> {
        PID11_W::new(self)
    }
    #[doc = "Bit 12 - Peripheral 12 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid12(&mut self) -> PID12_W<12> {
        PID12_W::new(self)
    }
    #[doc = "Bit 13 - Peripheral 13 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid13(&mut self) -> PID13_W<13> {
        PID13_W::new(self)
    }
    #[doc = "Bit 14 - Peripheral 14 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid14(&mut self) -> PID14_W<14> {
        PID14_W::new(self)
    }
    #[doc = "Bit 15 - Peripheral 15 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid15(&mut self) -> PID15_W<15> {
        PID15_W::new(self)
    }
    #[doc = "Bit 16 - Peripheral 16 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid16(&mut self) -> PID16_W<16> {
        PID16_W::new(self)
    }
    #[doc = "Bit 17 - Peripheral 17 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid17(&mut self) -> PID17_W<17> {
        PID17_W::new(self)
    }
    #[doc = "Bit 18 - Peripheral 18 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid18(&mut self) -> PID18_W<18> {
        PID18_W::new(self)
    }
    #[doc = "Bit 19 - Peripheral 19 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid19(&mut self) -> PID19_W<19> {
        PID19_W::new(self)
    }
    #[doc = "Bit 20 - Peripheral 20 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid20(&mut self) -> PID20_W<20> {
        PID20_W::new(self)
    }
    #[doc = "Bit 21 - Peripheral 21 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid21(&mut self) -> PID21_W<21> {
        PID21_W::new(self)
    }
    #[doc = "Bit 22 - Peripheral 22 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid22(&mut self) -> PID22_W<22> {
        PID22_W::new(self)
    }
    #[doc = "Bit 23 - Peripheral 23 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid23(&mut self) -> PID23_W<23> {
        PID23_W::new(self)
    }
    #[doc = "Bit 24 - Peripheral 24 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid24(&mut self) -> PID24_W<24> {
        PID24_W::new(self)
    }
    #[doc = "Bit 25 - Peripheral 25 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid25(&mut self) -> PID25_W<25> {
        PID25_W::new(self)
    }
    #[doc = "Bit 26 - Peripheral 26 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid26(&mut self) -> PID26_W<26> {
        PID26_W::new(self)
    }
    #[doc = "Bit 27 - Peripheral 27 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid27(&mut self) -> PID27_W<27> {
        PID27_W::new(self)
    }
    #[doc = "Bit 28 - Peripheral 28 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid28(&mut self) -> PID28_W<28> {
        PID28_W::new(self)
    }
    #[doc = "Bit 29 - Peripheral 29 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid29(&mut self) -> PID29_W<29> {
        PID29_W::new(self)
    }
    #[doc = "Bit 30 - Peripheral 30 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid30(&mut self) -> PID30_W<30> {
        PID30_W::new(self)
    }
    #[doc = "Bit 31 - Peripheral 31 SleepWalking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pid31(&mut self) -> PID31_W<31> {
        PID31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SleepWalking Enable Register 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_er0](index.html) module"]
pub struct SLPWK_ER0_SPEC;
impl crate::RegisterSpec for SLPWK_ER0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [slpwk_er0::W](W) writer structure"]
impl crate::Writable for SLPWK_ER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLPWK_ER0 to value 0"]
impl crate::Resettable for SLPWK_ER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
