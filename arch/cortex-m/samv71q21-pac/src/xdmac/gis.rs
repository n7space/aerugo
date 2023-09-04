#[doc = "Register `GIS` reader"]
pub type R = crate::R<GIS_SPEC>;
#[doc = "Field `IS0` reader - XDMAC Channel 0 Interrupt Status Bit"]
pub type IS0_R = crate::BitReader;
#[doc = "Field `IS1` reader - XDMAC Channel 1 Interrupt Status Bit"]
pub type IS1_R = crate::BitReader;
#[doc = "Field `IS2` reader - XDMAC Channel 2 Interrupt Status Bit"]
pub type IS2_R = crate::BitReader;
#[doc = "Field `IS3` reader - XDMAC Channel 3 Interrupt Status Bit"]
pub type IS3_R = crate::BitReader;
#[doc = "Field `IS4` reader - XDMAC Channel 4 Interrupt Status Bit"]
pub type IS4_R = crate::BitReader;
#[doc = "Field `IS5` reader - XDMAC Channel 5 Interrupt Status Bit"]
pub type IS5_R = crate::BitReader;
#[doc = "Field `IS6` reader - XDMAC Channel 6 Interrupt Status Bit"]
pub type IS6_R = crate::BitReader;
#[doc = "Field `IS7` reader - XDMAC Channel 7 Interrupt Status Bit"]
pub type IS7_R = crate::BitReader;
#[doc = "Field `IS8` reader - XDMAC Channel 8 Interrupt Status Bit"]
pub type IS8_R = crate::BitReader;
#[doc = "Field `IS9` reader - XDMAC Channel 9 Interrupt Status Bit"]
pub type IS9_R = crate::BitReader;
#[doc = "Field `IS10` reader - XDMAC Channel 10 Interrupt Status Bit"]
pub type IS10_R = crate::BitReader;
#[doc = "Field `IS11` reader - XDMAC Channel 11 Interrupt Status Bit"]
pub type IS11_R = crate::BitReader;
#[doc = "Field `IS12` reader - XDMAC Channel 12 Interrupt Status Bit"]
pub type IS12_R = crate::BitReader;
#[doc = "Field `IS13` reader - XDMAC Channel 13 Interrupt Status Bit"]
pub type IS13_R = crate::BitReader;
#[doc = "Field `IS14` reader - XDMAC Channel 14 Interrupt Status Bit"]
pub type IS14_R = crate::BitReader;
#[doc = "Field `IS15` reader - XDMAC Channel 15 Interrupt Status Bit"]
pub type IS15_R = crate::BitReader;
#[doc = "Field `IS16` reader - XDMAC Channel 16 Interrupt Status Bit"]
pub type IS16_R = crate::BitReader;
#[doc = "Field `IS17` reader - XDMAC Channel 17 Interrupt Status Bit"]
pub type IS17_R = crate::BitReader;
#[doc = "Field `IS18` reader - XDMAC Channel 18 Interrupt Status Bit"]
pub type IS18_R = crate::BitReader;
#[doc = "Field `IS19` reader - XDMAC Channel 19 Interrupt Status Bit"]
pub type IS19_R = crate::BitReader;
#[doc = "Field `IS20` reader - XDMAC Channel 20 Interrupt Status Bit"]
pub type IS20_R = crate::BitReader;
#[doc = "Field `IS21` reader - XDMAC Channel 21 Interrupt Status Bit"]
pub type IS21_R = crate::BitReader;
#[doc = "Field `IS22` reader - XDMAC Channel 22 Interrupt Status Bit"]
pub type IS22_R = crate::BitReader;
#[doc = "Field `IS23` reader - XDMAC Channel 23 Interrupt Status Bit"]
pub type IS23_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is0(&self) -> IS0_R {
        IS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is1(&self) -> IS1_R {
        IS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is2(&self) -> IS2_R {
        IS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is3(&self) -> IS3_R {
        IS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is4(&self) -> IS4_R {
        IS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is5(&self) -> IS5_R {
        IS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is6(&self) -> IS6_R {
        IS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is7(&self) -> IS7_R {
        IS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is8(&self) -> IS8_R {
        IS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is9(&self) -> IS9_R {
        IS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is10(&self) -> IS10_R {
        IS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is11(&self) -> IS11_R {
        IS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is12(&self) -> IS12_R {
        IS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is13(&self) -> IS13_R {
        IS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is14(&self) -> IS14_R {
        IS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is15(&self) -> IS15_R {
        IS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is16(&self) -> IS16_R {
        IS16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is17(&self) -> IS17_R {
        IS17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is18(&self) -> IS18_R {
        IS18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is19(&self) -> IS19_R {
        IS19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is20(&self) -> IS20_R {
        IS20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is21(&self) -> IS21_R {
        IS21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is22(&self) -> IS22_R {
        IS22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Status Bit"]
    #[inline(always)]
    pub fn is23(&self) -> IS23_R {
        IS23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Global Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GIS_SPEC;
impl crate::RegisterSpec for GIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gis::R`](R) reader structure"]
impl crate::Readable for GIS_SPEC {}
#[doc = "`reset()` method sets GIS to value 0"]
impl crate::Resettable for GIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
