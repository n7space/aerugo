#[doc = "Register `PUDR` writer"]
pub struct W(crate::W<PUDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUDR_SPEC>;
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
impl From<crate::W<PUDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P0` writer - Pull-Up Disable"]
pub type P0_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P1` writer - Pull-Up Disable"]
pub type P1_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P2` writer - Pull-Up Disable"]
pub type P2_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P3` writer - Pull-Up Disable"]
pub type P3_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P4` writer - Pull-Up Disable"]
pub type P4_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P5` writer - Pull-Up Disable"]
pub type P5_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P6` writer - Pull-Up Disable"]
pub type P6_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P7` writer - Pull-Up Disable"]
pub type P7_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P8` writer - Pull-Up Disable"]
pub type P8_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P9` writer - Pull-Up Disable"]
pub type P9_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P10` writer - Pull-Up Disable"]
pub type P10_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P11` writer - Pull-Up Disable"]
pub type P11_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P12` writer - Pull-Up Disable"]
pub type P12_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P13` writer - Pull-Up Disable"]
pub type P13_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P14` writer - Pull-Up Disable"]
pub type P14_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P15` writer - Pull-Up Disable"]
pub type P15_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P16` writer - Pull-Up Disable"]
pub type P16_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P17` writer - Pull-Up Disable"]
pub type P17_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P18` writer - Pull-Up Disable"]
pub type P18_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P19` writer - Pull-Up Disable"]
pub type P19_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P20` writer - Pull-Up Disable"]
pub type P20_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P21` writer - Pull-Up Disable"]
pub type P21_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P22` writer - Pull-Up Disable"]
pub type P22_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P23` writer - Pull-Up Disable"]
pub type P23_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P24` writer - Pull-Up Disable"]
pub type P24_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P25` writer - Pull-Up Disable"]
pub type P25_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P26` writer - Pull-Up Disable"]
pub type P26_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P27` writer - Pull-Up Disable"]
pub type P27_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P28` writer - Pull-Up Disable"]
pub type P28_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P29` writer - Pull-Up Disable"]
pub type P29_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P30` writer - Pull-Up Disable"]
pub type P30_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
#[doc = "Field `P31` writer - Pull-Up Disable"]
pub type P31_W<'a, const O: u8> = crate::BitWriter<'a, PUDR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<0> {
        P0_W::new(self)
    }
    #[doc = "Bit 1 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<1> {
        P1_W::new(self)
    }
    #[doc = "Bit 2 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<2> {
        P2_W::new(self)
    }
    #[doc = "Bit 3 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<3> {
        P3_W::new(self)
    }
    #[doc = "Bit 4 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<4> {
        P4_W::new(self)
    }
    #[doc = "Bit 5 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<5> {
        P5_W::new(self)
    }
    #[doc = "Bit 6 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<6> {
        P6_W::new(self)
    }
    #[doc = "Bit 7 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<7> {
        P7_W::new(self)
    }
    #[doc = "Bit 8 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<8> {
        P8_W::new(self)
    }
    #[doc = "Bit 9 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<9> {
        P9_W::new(self)
    }
    #[doc = "Bit 10 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<10> {
        P10_W::new(self)
    }
    #[doc = "Bit 11 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<11> {
        P11_W::new(self)
    }
    #[doc = "Bit 12 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<12> {
        P12_W::new(self)
    }
    #[doc = "Bit 13 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<13> {
        P13_W::new(self)
    }
    #[doc = "Bit 14 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14_W<14> {
        P14_W::new(self)
    }
    #[doc = "Bit 15 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15_W<15> {
        P15_W::new(self)
    }
    #[doc = "Bit 16 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16_W<16> {
        P16_W::new(self)
    }
    #[doc = "Bit 17 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17_W<17> {
        P17_W::new(self)
    }
    #[doc = "Bit 18 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18_W<18> {
        P18_W::new(self)
    }
    #[doc = "Bit 19 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19_W<19> {
        P19_W::new(self)
    }
    #[doc = "Bit 20 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20_W<20> {
        P20_W::new(self)
    }
    #[doc = "Bit 21 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21_W<21> {
        P21_W::new(self)
    }
    #[doc = "Bit 22 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22_W<22> {
        P22_W::new(self)
    }
    #[doc = "Bit 23 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23_W<23> {
        P23_W::new(self)
    }
    #[doc = "Bit 24 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24_W<24> {
        P24_W::new(self)
    }
    #[doc = "Bit 25 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25_W<25> {
        P25_W::new(self)
    }
    #[doc = "Bit 26 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26_W<26> {
        P26_W::new(self)
    }
    #[doc = "Bit 27 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27_W<27> {
        P27_W::new(self)
    }
    #[doc = "Bit 28 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28_W<28> {
        P28_W::new(self)
    }
    #[doc = "Bit 29 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29_W<29> {
        P29_W::new(self)
    }
    #[doc = "Bit 30 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30_W<30> {
        P30_W::new(self)
    }
    #[doc = "Bit 31 - Pull-Up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31_W<31> {
        P31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pull-up Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pudr](index.html) module"]
pub struct PUDR_SPEC;
impl crate::RegisterSpec for PUDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pudr::W](W) writer structure"]
impl crate::Writable for PUDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUDR to value 0"]
impl crate::Resettable for PUDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
