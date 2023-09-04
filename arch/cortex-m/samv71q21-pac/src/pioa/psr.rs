#[doc = "Register `PSR` reader"]
pub type R = crate::R<PSR_SPEC>;
#[doc = "Field `P0` reader - PIO Status"]
pub type P0_R = crate::BitReader;
#[doc = "Field `P1` reader - PIO Status"]
pub type P1_R = crate::BitReader;
#[doc = "Field `P2` reader - PIO Status"]
pub type P2_R = crate::BitReader;
#[doc = "Field `P3` reader - PIO Status"]
pub type P3_R = crate::BitReader;
#[doc = "Field `P4` reader - PIO Status"]
pub type P4_R = crate::BitReader;
#[doc = "Field `P5` reader - PIO Status"]
pub type P5_R = crate::BitReader;
#[doc = "Field `P6` reader - PIO Status"]
pub type P6_R = crate::BitReader;
#[doc = "Field `P7` reader - PIO Status"]
pub type P7_R = crate::BitReader;
#[doc = "Field `P8` reader - PIO Status"]
pub type P8_R = crate::BitReader;
#[doc = "Field `P9` reader - PIO Status"]
pub type P9_R = crate::BitReader;
#[doc = "Field `P10` reader - PIO Status"]
pub type P10_R = crate::BitReader;
#[doc = "Field `P11` reader - PIO Status"]
pub type P11_R = crate::BitReader;
#[doc = "Field `P12` reader - PIO Status"]
pub type P12_R = crate::BitReader;
#[doc = "Field `P13` reader - PIO Status"]
pub type P13_R = crate::BitReader;
#[doc = "Field `P14` reader - PIO Status"]
pub type P14_R = crate::BitReader;
#[doc = "Field `P15` reader - PIO Status"]
pub type P15_R = crate::BitReader;
#[doc = "Field `P16` reader - PIO Status"]
pub type P16_R = crate::BitReader;
#[doc = "Field `P17` reader - PIO Status"]
pub type P17_R = crate::BitReader;
#[doc = "Field `P18` reader - PIO Status"]
pub type P18_R = crate::BitReader;
#[doc = "Field `P19` reader - PIO Status"]
pub type P19_R = crate::BitReader;
#[doc = "Field `P20` reader - PIO Status"]
pub type P20_R = crate::BitReader;
#[doc = "Field `P21` reader - PIO Status"]
pub type P21_R = crate::BitReader;
#[doc = "Field `P22` reader - PIO Status"]
pub type P22_R = crate::BitReader;
#[doc = "Field `P23` reader - PIO Status"]
pub type P23_R = crate::BitReader;
#[doc = "Field `P24` reader - PIO Status"]
pub type P24_R = crate::BitReader;
#[doc = "Field `P25` reader - PIO Status"]
pub type P25_R = crate::BitReader;
#[doc = "Field `P26` reader - PIO Status"]
pub type P26_R = crate::BitReader;
#[doc = "Field `P27` reader - PIO Status"]
pub type P27_R = crate::BitReader;
#[doc = "Field `P28` reader - PIO Status"]
pub type P28_R = crate::BitReader;
#[doc = "Field `P29` reader - PIO Status"]
pub type P29_R = crate::BitReader;
#[doc = "Field `P30` reader - PIO Status"]
pub type P30_R = crate::BitReader;
#[doc = "Field `P31` reader - PIO Status"]
pub type P31_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PIO Status"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PIO Status"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PIO Status"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PIO Status"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PIO Status"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PIO Status"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PIO Status"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PIO Status"]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PIO Status"]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PIO Status"]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PIO Status"]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PIO Status"]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PIO Status"]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PIO Status"]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PIO Status"]
    #[inline(always)]
    pub fn p14(&self) -> P14_R {
        P14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PIO Status"]
    #[inline(always)]
    pub fn p15(&self) -> P15_R {
        P15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PIO Status"]
    #[inline(always)]
    pub fn p16(&self) -> P16_R {
        P16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PIO Status"]
    #[inline(always)]
    pub fn p17(&self) -> P17_R {
        P17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PIO Status"]
    #[inline(always)]
    pub fn p18(&self) -> P18_R {
        P18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PIO Status"]
    #[inline(always)]
    pub fn p19(&self) -> P19_R {
        P19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PIO Status"]
    #[inline(always)]
    pub fn p20(&self) -> P20_R {
        P20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PIO Status"]
    #[inline(always)]
    pub fn p21(&self) -> P21_R {
        P21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PIO Status"]
    #[inline(always)]
    pub fn p22(&self) -> P22_R {
        P22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PIO Status"]
    #[inline(always)]
    pub fn p23(&self) -> P23_R {
        P23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PIO Status"]
    #[inline(always)]
    pub fn p24(&self) -> P24_R {
        P24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PIO Status"]
    #[inline(always)]
    pub fn p25(&self) -> P25_R {
        P25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PIO Status"]
    #[inline(always)]
    pub fn p26(&self) -> P26_R {
        P26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PIO Status"]
    #[inline(always)]
    pub fn p27(&self) -> P27_R {
        P27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PIO Status"]
    #[inline(always)]
    pub fn p28(&self) -> P28_R {
        P28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PIO Status"]
    #[inline(always)]
    pub fn p29(&self) -> P29_R {
        P29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PIO Status"]
    #[inline(always)]
    pub fn p30(&self) -> P30_R {
        P30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PIO Status"]
    #[inline(always)]
    pub fn p31(&self) -> P31_R {
        P31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "PIO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PSR_SPEC {}
#[doc = "`reset()` method sets PSR to value 0"]
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
