#[doc = "Register `GSWS` reader"]
pub type R = crate::R<GSWS_SPEC>;
#[doc = "Field `SWRS0` reader - XDMAC Channel 0 Software Request Status Bit"]
pub type SWRS0_R = crate::BitReader;
#[doc = "Field `SWRS1` reader - XDMAC Channel 1 Software Request Status Bit"]
pub type SWRS1_R = crate::BitReader;
#[doc = "Field `SWRS2` reader - XDMAC Channel 2 Software Request Status Bit"]
pub type SWRS2_R = crate::BitReader;
#[doc = "Field `SWRS3` reader - XDMAC Channel 3 Software Request Status Bit"]
pub type SWRS3_R = crate::BitReader;
#[doc = "Field `SWRS4` reader - XDMAC Channel 4 Software Request Status Bit"]
pub type SWRS4_R = crate::BitReader;
#[doc = "Field `SWRS5` reader - XDMAC Channel 5 Software Request Status Bit"]
pub type SWRS5_R = crate::BitReader;
#[doc = "Field `SWRS6` reader - XDMAC Channel 6 Software Request Status Bit"]
pub type SWRS6_R = crate::BitReader;
#[doc = "Field `SWRS7` reader - XDMAC Channel 7 Software Request Status Bit"]
pub type SWRS7_R = crate::BitReader;
#[doc = "Field `SWRS8` reader - XDMAC Channel 8 Software Request Status Bit"]
pub type SWRS8_R = crate::BitReader;
#[doc = "Field `SWRS9` reader - XDMAC Channel 9 Software Request Status Bit"]
pub type SWRS9_R = crate::BitReader;
#[doc = "Field `SWRS10` reader - XDMAC Channel 10 Software Request Status Bit"]
pub type SWRS10_R = crate::BitReader;
#[doc = "Field `SWRS11` reader - XDMAC Channel 11 Software Request Status Bit"]
pub type SWRS11_R = crate::BitReader;
#[doc = "Field `SWRS12` reader - XDMAC Channel 12 Software Request Status Bit"]
pub type SWRS12_R = crate::BitReader;
#[doc = "Field `SWRS13` reader - XDMAC Channel 13 Software Request Status Bit"]
pub type SWRS13_R = crate::BitReader;
#[doc = "Field `SWRS14` reader - XDMAC Channel 14 Software Request Status Bit"]
pub type SWRS14_R = crate::BitReader;
#[doc = "Field `SWRS15` reader - XDMAC Channel 15 Software Request Status Bit"]
pub type SWRS15_R = crate::BitReader;
#[doc = "Field `SWRS16` reader - XDMAC Channel 16 Software Request Status Bit"]
pub type SWRS16_R = crate::BitReader;
#[doc = "Field `SWRS17` reader - XDMAC Channel 17 Software Request Status Bit"]
pub type SWRS17_R = crate::BitReader;
#[doc = "Field `SWRS18` reader - XDMAC Channel 18 Software Request Status Bit"]
pub type SWRS18_R = crate::BitReader;
#[doc = "Field `SWRS19` reader - XDMAC Channel 19 Software Request Status Bit"]
pub type SWRS19_R = crate::BitReader;
#[doc = "Field `SWRS20` reader - XDMAC Channel 20 Software Request Status Bit"]
pub type SWRS20_R = crate::BitReader;
#[doc = "Field `SWRS21` reader - XDMAC Channel 21 Software Request Status Bit"]
pub type SWRS21_R = crate::BitReader;
#[doc = "Field `SWRS22` reader - XDMAC Channel 22 Software Request Status Bit"]
pub type SWRS22_R = crate::BitReader;
#[doc = "Field `SWRS23` reader - XDMAC Channel 23 Software Request Status Bit"]
pub type SWRS23_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs0(&self) -> SWRS0_R {
        SWRS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs1(&self) -> SWRS1_R {
        SWRS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs2(&self) -> SWRS2_R {
        SWRS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs3(&self) -> SWRS3_R {
        SWRS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs4(&self) -> SWRS4_R {
        SWRS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs5(&self) -> SWRS5_R {
        SWRS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs6(&self) -> SWRS6_R {
        SWRS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs7(&self) -> SWRS7_R {
        SWRS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs8(&self) -> SWRS8_R {
        SWRS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs9(&self) -> SWRS9_R {
        SWRS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs10(&self) -> SWRS10_R {
        SWRS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs11(&self) -> SWRS11_R {
        SWRS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs12(&self) -> SWRS12_R {
        SWRS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs13(&self) -> SWRS13_R {
        SWRS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs14(&self) -> SWRS14_R {
        SWRS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs15(&self) -> SWRS15_R {
        SWRS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs16(&self) -> SWRS16_R {
        SWRS16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs17(&self) -> SWRS17_R {
        SWRS17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs18(&self) -> SWRS18_R {
        SWRS18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs19(&self) -> SWRS19_R {
        SWRS19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs20(&self) -> SWRS20_R {
        SWRS20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs21(&self) -> SWRS21_R {
        SWRS21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs22(&self) -> SWRS22_R {
        SWRS22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs23(&self) -> SWRS23_R {
        SWRS23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Global Channel Software Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsws::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GSWS_SPEC;
impl crate::RegisterSpec for GSWS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsws::R`](R) reader structure"]
impl crate::Readable for GSWS_SPEC {}
#[doc = "`reset()` method sets GSWS to value 0"]
impl crate::Resettable for GSWS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
