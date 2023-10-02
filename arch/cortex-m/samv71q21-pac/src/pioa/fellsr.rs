#[doc = "Register `FELLSR` writer"]
pub type W = crate::W<FELLSR_SPEC>;
#[doc = "Field `P0` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P1` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P3` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P4` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P5` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P6` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P7` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P8` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P9` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P10` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P11` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P12` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P13` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P14` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P15` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P16` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P17` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P18` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P19` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P20` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P21` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P22` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P23` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P24` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P25` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P25_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P26` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P26_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P27` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P27_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P28` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P28_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P29` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P29_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P30` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P30_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P31` writer - Falling Edge/Low-Level Interrupt Selection"]
pub type P31_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<FELLSR_SPEC, 0> {
        P0_W::new(self)
    }
    #[doc = "Bit 1 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<FELLSR_SPEC, 1> {
        P1_W::new(self)
    }
    #[doc = "Bit 2 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<FELLSR_SPEC, 2> {
        P2_W::new(self)
    }
    #[doc = "Bit 3 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<FELLSR_SPEC, 3> {
        P3_W::new(self)
    }
    #[doc = "Bit 4 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<FELLSR_SPEC, 4> {
        P4_W::new(self)
    }
    #[doc = "Bit 5 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<FELLSR_SPEC, 5> {
        P5_W::new(self)
    }
    #[doc = "Bit 6 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<FELLSR_SPEC, 6> {
        P6_W::new(self)
    }
    #[doc = "Bit 7 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<FELLSR_SPEC, 7> {
        P7_W::new(self)
    }
    #[doc = "Bit 8 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<FELLSR_SPEC, 8> {
        P8_W::new(self)
    }
    #[doc = "Bit 9 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<FELLSR_SPEC, 9> {
        P9_W::new(self)
    }
    #[doc = "Bit 10 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<FELLSR_SPEC, 10> {
        P10_W::new(self)
    }
    #[doc = "Bit 11 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<FELLSR_SPEC, 11> {
        P11_W::new(self)
    }
    #[doc = "Bit 12 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<FELLSR_SPEC, 12> {
        P12_W::new(self)
    }
    #[doc = "Bit 13 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<FELLSR_SPEC, 13> {
        P13_W::new(self)
    }
    #[doc = "Bit 14 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14_W<FELLSR_SPEC, 14> {
        P14_W::new(self)
    }
    #[doc = "Bit 15 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15_W<FELLSR_SPEC, 15> {
        P15_W::new(self)
    }
    #[doc = "Bit 16 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16_W<FELLSR_SPEC, 16> {
        P16_W::new(self)
    }
    #[doc = "Bit 17 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17_W<FELLSR_SPEC, 17> {
        P17_W::new(self)
    }
    #[doc = "Bit 18 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18_W<FELLSR_SPEC, 18> {
        P18_W::new(self)
    }
    #[doc = "Bit 19 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19_W<FELLSR_SPEC, 19> {
        P19_W::new(self)
    }
    #[doc = "Bit 20 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20_W<FELLSR_SPEC, 20> {
        P20_W::new(self)
    }
    #[doc = "Bit 21 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21_W<FELLSR_SPEC, 21> {
        P21_W::new(self)
    }
    #[doc = "Bit 22 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22_W<FELLSR_SPEC, 22> {
        P22_W::new(self)
    }
    #[doc = "Bit 23 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23_W<FELLSR_SPEC, 23> {
        P23_W::new(self)
    }
    #[doc = "Bit 24 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24_W<FELLSR_SPEC, 24> {
        P24_W::new(self)
    }
    #[doc = "Bit 25 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25_W<FELLSR_SPEC, 25> {
        P25_W::new(self)
    }
    #[doc = "Bit 26 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26_W<FELLSR_SPEC, 26> {
        P26_W::new(self)
    }
    #[doc = "Bit 27 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27_W<FELLSR_SPEC, 27> {
        P27_W::new(self)
    }
    #[doc = "Bit 28 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28_W<FELLSR_SPEC, 28> {
        P28_W::new(self)
    }
    #[doc = "Bit 29 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29_W<FELLSR_SPEC, 29> {
        P29_W::new(self)
    }
    #[doc = "Bit 30 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30_W<FELLSR_SPEC, 30> {
        P30_W::new(self)
    }
    #[doc = "Bit 31 - Falling Edge/Low-Level Interrupt Selection"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31_W<FELLSR_SPEC, 31> {
        P31_W::new(self)
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
#[doc = "Falling Edge/Low-Level Select Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fellsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FELLSR_SPEC;
impl crate::RegisterSpec for FELLSR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fellsr::W`](W) writer structure"]
impl crate::Writable for FELLSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FELLSR to value 0"]
impl crate::Resettable for FELLSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
