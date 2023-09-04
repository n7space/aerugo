#[doc = "Register `SODR` writer"]
pub type W = crate::W<SODR_SPEC>;
#[doc = "Field `P0` writer - Set Output Data"]
pub type P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P1` writer - Set Output Data"]
pub type P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2` writer - Set Output Data"]
pub type P2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P3` writer - Set Output Data"]
pub type P3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P4` writer - Set Output Data"]
pub type P4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P5` writer - Set Output Data"]
pub type P5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P6` writer - Set Output Data"]
pub type P6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P7` writer - Set Output Data"]
pub type P7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P8` writer - Set Output Data"]
pub type P8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P9` writer - Set Output Data"]
pub type P9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P10` writer - Set Output Data"]
pub type P10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P11` writer - Set Output Data"]
pub type P11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P12` writer - Set Output Data"]
pub type P12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P13` writer - Set Output Data"]
pub type P13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P14` writer - Set Output Data"]
pub type P14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P15` writer - Set Output Data"]
pub type P15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P16` writer - Set Output Data"]
pub type P16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P17` writer - Set Output Data"]
pub type P17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P18` writer - Set Output Data"]
pub type P18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P19` writer - Set Output Data"]
pub type P19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P20` writer - Set Output Data"]
pub type P20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P21` writer - Set Output Data"]
pub type P21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P22` writer - Set Output Data"]
pub type P22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P23` writer - Set Output Data"]
pub type P23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P24` writer - Set Output Data"]
pub type P24_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P25` writer - Set Output Data"]
pub type P25_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P26` writer - Set Output Data"]
pub type P26_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P27` writer - Set Output Data"]
pub type P27_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P28` writer - Set Output Data"]
pub type P28_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P29` writer - Set Output Data"]
pub type P29_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P30` writer - Set Output Data"]
pub type P30_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P31` writer - Set Output Data"]
pub type P31_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<SODR_SPEC, 0> {
        P0_W::new(self)
    }
    #[doc = "Bit 1 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<SODR_SPEC, 1> {
        P1_W::new(self)
    }
    #[doc = "Bit 2 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<SODR_SPEC, 2> {
        P2_W::new(self)
    }
    #[doc = "Bit 3 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<SODR_SPEC, 3> {
        P3_W::new(self)
    }
    #[doc = "Bit 4 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<SODR_SPEC, 4> {
        P4_W::new(self)
    }
    #[doc = "Bit 5 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<SODR_SPEC, 5> {
        P5_W::new(self)
    }
    #[doc = "Bit 6 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<SODR_SPEC, 6> {
        P6_W::new(self)
    }
    #[doc = "Bit 7 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<SODR_SPEC, 7> {
        P7_W::new(self)
    }
    #[doc = "Bit 8 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<SODR_SPEC, 8> {
        P8_W::new(self)
    }
    #[doc = "Bit 9 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<SODR_SPEC, 9> {
        P9_W::new(self)
    }
    #[doc = "Bit 10 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<SODR_SPEC, 10> {
        P10_W::new(self)
    }
    #[doc = "Bit 11 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<SODR_SPEC, 11> {
        P11_W::new(self)
    }
    #[doc = "Bit 12 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<SODR_SPEC, 12> {
        P12_W::new(self)
    }
    #[doc = "Bit 13 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<SODR_SPEC, 13> {
        P13_W::new(self)
    }
    #[doc = "Bit 14 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14_W<SODR_SPEC, 14> {
        P14_W::new(self)
    }
    #[doc = "Bit 15 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15_W<SODR_SPEC, 15> {
        P15_W::new(self)
    }
    #[doc = "Bit 16 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16_W<SODR_SPEC, 16> {
        P16_W::new(self)
    }
    #[doc = "Bit 17 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17_W<SODR_SPEC, 17> {
        P17_W::new(self)
    }
    #[doc = "Bit 18 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18_W<SODR_SPEC, 18> {
        P18_W::new(self)
    }
    #[doc = "Bit 19 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19_W<SODR_SPEC, 19> {
        P19_W::new(self)
    }
    #[doc = "Bit 20 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20_W<SODR_SPEC, 20> {
        P20_W::new(self)
    }
    #[doc = "Bit 21 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21_W<SODR_SPEC, 21> {
        P21_W::new(self)
    }
    #[doc = "Bit 22 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22_W<SODR_SPEC, 22> {
        P22_W::new(self)
    }
    #[doc = "Bit 23 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23_W<SODR_SPEC, 23> {
        P23_W::new(self)
    }
    #[doc = "Bit 24 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24_W<SODR_SPEC, 24> {
        P24_W::new(self)
    }
    #[doc = "Bit 25 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25_W<SODR_SPEC, 25> {
        P25_W::new(self)
    }
    #[doc = "Bit 26 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26_W<SODR_SPEC, 26> {
        P26_W::new(self)
    }
    #[doc = "Bit 27 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27_W<SODR_SPEC, 27> {
        P27_W::new(self)
    }
    #[doc = "Bit 28 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28_W<SODR_SPEC, 28> {
        P28_W::new(self)
    }
    #[doc = "Bit 29 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29_W<SODR_SPEC, 29> {
        P29_W::new(self)
    }
    #[doc = "Bit 30 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30_W<SODR_SPEC, 30> {
        P30_W::new(self)
    }
    #[doc = "Bit 31 - Set Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31_W<SODR_SPEC, 31> {
        P31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Set Output Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sodr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SODR_SPEC;
impl crate::RegisterSpec for SODR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sodr::W`](W) writer structure"]
impl crate::Writable for SODR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SODR to value 0"]
impl crate::Resettable for SODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
