#[doc = "Register `TXBTO` reader"]
pub struct R(crate::R<TXBTO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBTO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBTO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBTO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TO0` reader - Transmission Occurred for Buffer 0"]
pub type TO0_R = crate::BitReader;
#[doc = "Field `TO1` reader - Transmission Occurred for Buffer 1"]
pub type TO1_R = crate::BitReader;
#[doc = "Field `TO2` reader - Transmission Occurred for Buffer 2"]
pub type TO2_R = crate::BitReader;
#[doc = "Field `TO3` reader - Transmission Occurred for Buffer 3"]
pub type TO3_R = crate::BitReader;
#[doc = "Field `TO4` reader - Transmission Occurred for Buffer 4"]
pub type TO4_R = crate::BitReader;
#[doc = "Field `TO5` reader - Transmission Occurred for Buffer 5"]
pub type TO5_R = crate::BitReader;
#[doc = "Field `TO6` reader - Transmission Occurred for Buffer 6"]
pub type TO6_R = crate::BitReader;
#[doc = "Field `TO7` reader - Transmission Occurred for Buffer 7"]
pub type TO7_R = crate::BitReader;
#[doc = "Field `TO8` reader - Transmission Occurred for Buffer 8"]
pub type TO8_R = crate::BitReader;
#[doc = "Field `TO9` reader - Transmission Occurred for Buffer 9"]
pub type TO9_R = crate::BitReader;
#[doc = "Field `TO10` reader - Transmission Occurred for Buffer 10"]
pub type TO10_R = crate::BitReader;
#[doc = "Field `TO11` reader - Transmission Occurred for Buffer 11"]
pub type TO11_R = crate::BitReader;
#[doc = "Field `TO12` reader - Transmission Occurred for Buffer 12"]
pub type TO12_R = crate::BitReader;
#[doc = "Field `TO13` reader - Transmission Occurred for Buffer 13"]
pub type TO13_R = crate::BitReader;
#[doc = "Field `TO14` reader - Transmission Occurred for Buffer 14"]
pub type TO14_R = crate::BitReader;
#[doc = "Field `TO15` reader - Transmission Occurred for Buffer 15"]
pub type TO15_R = crate::BitReader;
#[doc = "Field `TO16` reader - Transmission Occurred for Buffer 16"]
pub type TO16_R = crate::BitReader;
#[doc = "Field `TO17` reader - Transmission Occurred for Buffer 17"]
pub type TO17_R = crate::BitReader;
#[doc = "Field `TO18` reader - Transmission Occurred for Buffer 18"]
pub type TO18_R = crate::BitReader;
#[doc = "Field `TO19` reader - Transmission Occurred for Buffer 19"]
pub type TO19_R = crate::BitReader;
#[doc = "Field `TO20` reader - Transmission Occurred for Buffer 20"]
pub type TO20_R = crate::BitReader;
#[doc = "Field `TO21` reader - Transmission Occurred for Buffer 21"]
pub type TO21_R = crate::BitReader;
#[doc = "Field `TO22` reader - Transmission Occurred for Buffer 22"]
pub type TO22_R = crate::BitReader;
#[doc = "Field `TO23` reader - Transmission Occurred for Buffer 23"]
pub type TO23_R = crate::BitReader;
#[doc = "Field `TO24` reader - Transmission Occurred for Buffer 24"]
pub type TO24_R = crate::BitReader;
#[doc = "Field `TO25` reader - Transmission Occurred for Buffer 25"]
pub type TO25_R = crate::BitReader;
#[doc = "Field `TO26` reader - Transmission Occurred for Buffer 26"]
pub type TO26_R = crate::BitReader;
#[doc = "Field `TO27` reader - Transmission Occurred for Buffer 27"]
pub type TO27_R = crate::BitReader;
#[doc = "Field `TO28` reader - Transmission Occurred for Buffer 28"]
pub type TO28_R = crate::BitReader;
#[doc = "Field `TO29` reader - Transmission Occurred for Buffer 29"]
pub type TO29_R = crate::BitReader;
#[doc = "Field `TO30` reader - Transmission Occurred for Buffer 30"]
pub type TO30_R = crate::BitReader;
#[doc = "Field `TO31` reader - Transmission Occurred for Buffer 31"]
pub type TO31_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission Occurred for Buffer 0"]
    #[inline(always)]
    pub fn to0(&self) -> TO0_R {
        TO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission Occurred for Buffer 1"]
    #[inline(always)]
    pub fn to1(&self) -> TO1_R {
        TO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Occurred for Buffer 2"]
    #[inline(always)]
    pub fn to2(&self) -> TO2_R {
        TO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Occurred for Buffer 3"]
    #[inline(always)]
    pub fn to3(&self) -> TO3_R {
        TO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmission Occurred for Buffer 4"]
    #[inline(always)]
    pub fn to4(&self) -> TO4_R {
        TO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmission Occurred for Buffer 5"]
    #[inline(always)]
    pub fn to5(&self) -> TO5_R {
        TO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Occurred for Buffer 6"]
    #[inline(always)]
    pub fn to6(&self) -> TO6_R {
        TO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission Occurred for Buffer 7"]
    #[inline(always)]
    pub fn to7(&self) -> TO7_R {
        TO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission Occurred for Buffer 8"]
    #[inline(always)]
    pub fn to8(&self) -> TO8_R {
        TO8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Occurred for Buffer 9"]
    #[inline(always)]
    pub fn to9(&self) -> TO9_R {
        TO9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Occurred for Buffer 10"]
    #[inline(always)]
    pub fn to10(&self) -> TO10_R {
        TO10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmission Occurred for Buffer 11"]
    #[inline(always)]
    pub fn to11(&self) -> TO11_R {
        TO11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmission Occurred for Buffer 12"]
    #[inline(always)]
    pub fn to12(&self) -> TO12_R {
        TO12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmission Occurred for Buffer 13"]
    #[inline(always)]
    pub fn to13(&self) -> TO13_R {
        TO13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmission Occurred for Buffer 14"]
    #[inline(always)]
    pub fn to14(&self) -> TO14_R {
        TO14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmission Occurred for Buffer 15"]
    #[inline(always)]
    pub fn to15(&self) -> TO15_R {
        TO15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmission Occurred for Buffer 16"]
    #[inline(always)]
    pub fn to16(&self) -> TO16_R {
        TO16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmission Occurred for Buffer 17"]
    #[inline(always)]
    pub fn to17(&self) -> TO17_R {
        TO17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmission Occurred for Buffer 18"]
    #[inline(always)]
    pub fn to18(&self) -> TO18_R {
        TO18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmission Occurred for Buffer 19"]
    #[inline(always)]
    pub fn to19(&self) -> TO19_R {
        TO19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmission Occurred for Buffer 20"]
    #[inline(always)]
    pub fn to20(&self) -> TO20_R {
        TO20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmission Occurred for Buffer 21"]
    #[inline(always)]
    pub fn to21(&self) -> TO21_R {
        TO21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmission Occurred for Buffer 22"]
    #[inline(always)]
    pub fn to22(&self) -> TO22_R {
        TO22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmission Occurred for Buffer 23"]
    #[inline(always)]
    pub fn to23(&self) -> TO23_R {
        TO23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmission Occurred for Buffer 24"]
    #[inline(always)]
    pub fn to24(&self) -> TO24_R {
        TO24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmission Occurred for Buffer 25"]
    #[inline(always)]
    pub fn to25(&self) -> TO25_R {
        TO25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmission Occurred for Buffer 26"]
    #[inline(always)]
    pub fn to26(&self) -> TO26_R {
        TO26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmission Occurred for Buffer 27"]
    #[inline(always)]
    pub fn to27(&self) -> TO27_R {
        TO27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmission Occurred for Buffer 28"]
    #[inline(always)]
    pub fn to28(&self) -> TO28_R {
        TO28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmission Occurred for Buffer 29"]
    #[inline(always)]
    pub fn to29(&self) -> TO29_R {
        TO29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmission Occurred for Buffer 30"]
    #[inline(always)]
    pub fn to30(&self) -> TO30_R {
        TO30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmission Occurred for Buffer 31"]
    #[inline(always)]
    pub fn to31(&self) -> TO31_R {
        TO31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Transmit Buffer Transmission Occurred Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbto](index.html) module"]
pub struct TXBTO_SPEC;
impl crate::RegisterSpec for TXBTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbto::R](R) reader structure"]
impl crate::Readable for TXBTO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXBTO to value 0"]
impl crate::Resettable for TXBTO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
