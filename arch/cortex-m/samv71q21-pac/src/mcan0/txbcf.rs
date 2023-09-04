#[doc = "Register `TXBCF` reader"]
pub type R = crate::R<TXBCF_SPEC>;
#[doc = "Field `CF0` reader - Cancellation Finished for Transmit Buffer 0"]
pub type CF0_R = crate::BitReader;
#[doc = "Field `CF1` reader - Cancellation Finished for Transmit Buffer 1"]
pub type CF1_R = crate::BitReader;
#[doc = "Field `CF2` reader - Cancellation Finished for Transmit Buffer 2"]
pub type CF2_R = crate::BitReader;
#[doc = "Field `CF3` reader - Cancellation Finished for Transmit Buffer 3"]
pub type CF3_R = crate::BitReader;
#[doc = "Field `CF4` reader - Cancellation Finished for Transmit Buffer 4"]
pub type CF4_R = crate::BitReader;
#[doc = "Field `CF5` reader - Cancellation Finished for Transmit Buffer 5"]
pub type CF5_R = crate::BitReader;
#[doc = "Field `CF6` reader - Cancellation Finished for Transmit Buffer 6"]
pub type CF6_R = crate::BitReader;
#[doc = "Field `CF7` reader - Cancellation Finished for Transmit Buffer 7"]
pub type CF7_R = crate::BitReader;
#[doc = "Field `CF8` reader - Cancellation Finished for Transmit Buffer 8"]
pub type CF8_R = crate::BitReader;
#[doc = "Field `CF9` reader - Cancellation Finished for Transmit Buffer 9"]
pub type CF9_R = crate::BitReader;
#[doc = "Field `CF10` reader - Cancellation Finished for Transmit Buffer 10"]
pub type CF10_R = crate::BitReader;
#[doc = "Field `CF11` reader - Cancellation Finished for Transmit Buffer 11"]
pub type CF11_R = crate::BitReader;
#[doc = "Field `CF12` reader - Cancellation Finished for Transmit Buffer 12"]
pub type CF12_R = crate::BitReader;
#[doc = "Field `CF13` reader - Cancellation Finished for Transmit Buffer 13"]
pub type CF13_R = crate::BitReader;
#[doc = "Field `CF14` reader - Cancellation Finished for Transmit Buffer 14"]
pub type CF14_R = crate::BitReader;
#[doc = "Field `CF15` reader - Cancellation Finished for Transmit Buffer 15"]
pub type CF15_R = crate::BitReader;
#[doc = "Field `CF16` reader - Cancellation Finished for Transmit Buffer 16"]
pub type CF16_R = crate::BitReader;
#[doc = "Field `CF17` reader - Cancellation Finished for Transmit Buffer 17"]
pub type CF17_R = crate::BitReader;
#[doc = "Field `CF18` reader - Cancellation Finished for Transmit Buffer 18"]
pub type CF18_R = crate::BitReader;
#[doc = "Field `CF19` reader - Cancellation Finished for Transmit Buffer 19"]
pub type CF19_R = crate::BitReader;
#[doc = "Field `CF20` reader - Cancellation Finished for Transmit Buffer 20"]
pub type CF20_R = crate::BitReader;
#[doc = "Field `CF21` reader - Cancellation Finished for Transmit Buffer 21"]
pub type CF21_R = crate::BitReader;
#[doc = "Field `CF22` reader - Cancellation Finished for Transmit Buffer 22"]
pub type CF22_R = crate::BitReader;
#[doc = "Field `CF23` reader - Cancellation Finished for Transmit Buffer 23"]
pub type CF23_R = crate::BitReader;
#[doc = "Field `CF24` reader - Cancellation Finished for Transmit Buffer 24"]
pub type CF24_R = crate::BitReader;
#[doc = "Field `CF25` reader - Cancellation Finished for Transmit Buffer 25"]
pub type CF25_R = crate::BitReader;
#[doc = "Field `CF26` reader - Cancellation Finished for Transmit Buffer 26"]
pub type CF26_R = crate::BitReader;
#[doc = "Field `CF27` reader - Cancellation Finished for Transmit Buffer 27"]
pub type CF27_R = crate::BitReader;
#[doc = "Field `CF28` reader - Cancellation Finished for Transmit Buffer 28"]
pub type CF28_R = crate::BitReader;
#[doc = "Field `CF29` reader - Cancellation Finished for Transmit Buffer 29"]
pub type CF29_R = crate::BitReader;
#[doc = "Field `CF30` reader - Cancellation Finished for Transmit Buffer 30"]
pub type CF30_R = crate::BitReader;
#[doc = "Field `CF31` reader - Cancellation Finished for Transmit Buffer 31"]
pub type CF31_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Cancellation Finished for Transmit Buffer 0"]
    #[inline(always)]
    pub fn cf0(&self) -> CF0_R {
        CF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cancellation Finished for Transmit Buffer 1"]
    #[inline(always)]
    pub fn cf1(&self) -> CF1_R {
        CF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cancellation Finished for Transmit Buffer 2"]
    #[inline(always)]
    pub fn cf2(&self) -> CF2_R {
        CF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cancellation Finished for Transmit Buffer 3"]
    #[inline(always)]
    pub fn cf3(&self) -> CF3_R {
        CF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cancellation Finished for Transmit Buffer 4"]
    #[inline(always)]
    pub fn cf4(&self) -> CF4_R {
        CF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Cancellation Finished for Transmit Buffer 5"]
    #[inline(always)]
    pub fn cf5(&self) -> CF5_R {
        CF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Cancellation Finished for Transmit Buffer 6"]
    #[inline(always)]
    pub fn cf6(&self) -> CF6_R {
        CF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Cancellation Finished for Transmit Buffer 7"]
    #[inline(always)]
    pub fn cf7(&self) -> CF7_R {
        CF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Cancellation Finished for Transmit Buffer 8"]
    #[inline(always)]
    pub fn cf8(&self) -> CF8_R {
        CF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Cancellation Finished for Transmit Buffer 9"]
    #[inline(always)]
    pub fn cf9(&self) -> CF9_R {
        CF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Cancellation Finished for Transmit Buffer 10"]
    #[inline(always)]
    pub fn cf10(&self) -> CF10_R {
        CF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Cancellation Finished for Transmit Buffer 11"]
    #[inline(always)]
    pub fn cf11(&self) -> CF11_R {
        CF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Cancellation Finished for Transmit Buffer 12"]
    #[inline(always)]
    pub fn cf12(&self) -> CF12_R {
        CF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Cancellation Finished for Transmit Buffer 13"]
    #[inline(always)]
    pub fn cf13(&self) -> CF13_R {
        CF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Cancellation Finished for Transmit Buffer 14"]
    #[inline(always)]
    pub fn cf14(&self) -> CF14_R {
        CF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Cancellation Finished for Transmit Buffer 15"]
    #[inline(always)]
    pub fn cf15(&self) -> CF15_R {
        CF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Cancellation Finished for Transmit Buffer 16"]
    #[inline(always)]
    pub fn cf16(&self) -> CF16_R {
        CF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Cancellation Finished for Transmit Buffer 17"]
    #[inline(always)]
    pub fn cf17(&self) -> CF17_R {
        CF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Cancellation Finished for Transmit Buffer 18"]
    #[inline(always)]
    pub fn cf18(&self) -> CF18_R {
        CF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Cancellation Finished for Transmit Buffer 19"]
    #[inline(always)]
    pub fn cf19(&self) -> CF19_R {
        CF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Cancellation Finished for Transmit Buffer 20"]
    #[inline(always)]
    pub fn cf20(&self) -> CF20_R {
        CF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Cancellation Finished for Transmit Buffer 21"]
    #[inline(always)]
    pub fn cf21(&self) -> CF21_R {
        CF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Cancellation Finished for Transmit Buffer 22"]
    #[inline(always)]
    pub fn cf22(&self) -> CF22_R {
        CF22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Cancellation Finished for Transmit Buffer 23"]
    #[inline(always)]
    pub fn cf23(&self) -> CF23_R {
        CF23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Cancellation Finished for Transmit Buffer 24"]
    #[inline(always)]
    pub fn cf24(&self) -> CF24_R {
        CF24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Cancellation Finished for Transmit Buffer 25"]
    #[inline(always)]
    pub fn cf25(&self) -> CF25_R {
        CF25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Cancellation Finished for Transmit Buffer 26"]
    #[inline(always)]
    pub fn cf26(&self) -> CF26_R {
        CF26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Cancellation Finished for Transmit Buffer 27"]
    #[inline(always)]
    pub fn cf27(&self) -> CF27_R {
        CF27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Cancellation Finished for Transmit Buffer 28"]
    #[inline(always)]
    pub fn cf28(&self) -> CF28_R {
        CF28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Cancellation Finished for Transmit Buffer 29"]
    #[inline(always)]
    pub fn cf29(&self) -> CF29_R {
        CF29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Cancellation Finished for Transmit Buffer 30"]
    #[inline(always)]
    pub fn cf30(&self) -> CF30_R {
        CF30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Cancellation Finished for Transmit Buffer 31"]
    #[inline(always)]
    pub fn cf31(&self) -> CF31_R {
        CF31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Transmit Buffer Cancellation Finished Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBCF_SPEC;
impl crate::RegisterSpec for TXBCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcf::R`](R) reader structure"]
impl crate::Readable for TXBCF_SPEC {}
#[doc = "`reset()` method sets TXBCF to value 0"]
impl crate::Resettable for TXBCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
