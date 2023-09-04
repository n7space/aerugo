#[doc = "Register `GS` reader"]
pub type R = crate::R<GS_SPEC>;
#[doc = "Field `ST0` reader - XDMAC Channel 0 Status Bit"]
pub type ST0_R = crate::BitReader;
#[doc = "Field `ST1` reader - XDMAC Channel 1 Status Bit"]
pub type ST1_R = crate::BitReader;
#[doc = "Field `ST2` reader - XDMAC Channel 2 Status Bit"]
pub type ST2_R = crate::BitReader;
#[doc = "Field `ST3` reader - XDMAC Channel 3 Status Bit"]
pub type ST3_R = crate::BitReader;
#[doc = "Field `ST4` reader - XDMAC Channel 4 Status Bit"]
pub type ST4_R = crate::BitReader;
#[doc = "Field `ST5` reader - XDMAC Channel 5 Status Bit"]
pub type ST5_R = crate::BitReader;
#[doc = "Field `ST6` reader - XDMAC Channel 6 Status Bit"]
pub type ST6_R = crate::BitReader;
#[doc = "Field `ST7` reader - XDMAC Channel 7 Status Bit"]
pub type ST7_R = crate::BitReader;
#[doc = "Field `ST8` reader - XDMAC Channel 8 Status Bit"]
pub type ST8_R = crate::BitReader;
#[doc = "Field `ST9` reader - XDMAC Channel 9 Status Bit"]
pub type ST9_R = crate::BitReader;
#[doc = "Field `ST10` reader - XDMAC Channel 10 Status Bit"]
pub type ST10_R = crate::BitReader;
#[doc = "Field `ST11` reader - XDMAC Channel 11 Status Bit"]
pub type ST11_R = crate::BitReader;
#[doc = "Field `ST12` reader - XDMAC Channel 12 Status Bit"]
pub type ST12_R = crate::BitReader;
#[doc = "Field `ST13` reader - XDMAC Channel 13 Status Bit"]
pub type ST13_R = crate::BitReader;
#[doc = "Field `ST14` reader - XDMAC Channel 14 Status Bit"]
pub type ST14_R = crate::BitReader;
#[doc = "Field `ST15` reader - XDMAC Channel 15 Status Bit"]
pub type ST15_R = crate::BitReader;
#[doc = "Field `ST16` reader - XDMAC Channel 16 Status Bit"]
pub type ST16_R = crate::BitReader;
#[doc = "Field `ST17` reader - XDMAC Channel 17 Status Bit"]
pub type ST17_R = crate::BitReader;
#[doc = "Field `ST18` reader - XDMAC Channel 18 Status Bit"]
pub type ST18_R = crate::BitReader;
#[doc = "Field `ST19` reader - XDMAC Channel 19 Status Bit"]
pub type ST19_R = crate::BitReader;
#[doc = "Field `ST20` reader - XDMAC Channel 20 Status Bit"]
pub type ST20_R = crate::BitReader;
#[doc = "Field `ST21` reader - XDMAC Channel 21 Status Bit"]
pub type ST21_R = crate::BitReader;
#[doc = "Field `ST22` reader - XDMAC Channel 22 Status Bit"]
pub type ST22_R = crate::BitReader;
#[doc = "Field `ST23` reader - XDMAC Channel 23 Status Bit"]
pub type ST23_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Status Bit"]
    #[inline(always)]
    pub fn st0(&self) -> ST0_R {
        ST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Status Bit"]
    #[inline(always)]
    pub fn st1(&self) -> ST1_R {
        ST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Status Bit"]
    #[inline(always)]
    pub fn st2(&self) -> ST2_R {
        ST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Status Bit"]
    #[inline(always)]
    pub fn st3(&self) -> ST3_R {
        ST3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Status Bit"]
    #[inline(always)]
    pub fn st4(&self) -> ST4_R {
        ST4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Status Bit"]
    #[inline(always)]
    pub fn st5(&self) -> ST5_R {
        ST5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Status Bit"]
    #[inline(always)]
    pub fn st6(&self) -> ST6_R {
        ST6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Status Bit"]
    #[inline(always)]
    pub fn st7(&self) -> ST7_R {
        ST7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Status Bit"]
    #[inline(always)]
    pub fn st8(&self) -> ST8_R {
        ST8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Status Bit"]
    #[inline(always)]
    pub fn st9(&self) -> ST9_R {
        ST9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Status Bit"]
    #[inline(always)]
    pub fn st10(&self) -> ST10_R {
        ST10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Status Bit"]
    #[inline(always)]
    pub fn st11(&self) -> ST11_R {
        ST11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Status Bit"]
    #[inline(always)]
    pub fn st12(&self) -> ST12_R {
        ST12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Status Bit"]
    #[inline(always)]
    pub fn st13(&self) -> ST13_R {
        ST13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Status Bit"]
    #[inline(always)]
    pub fn st14(&self) -> ST14_R {
        ST14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Status Bit"]
    #[inline(always)]
    pub fn st15(&self) -> ST15_R {
        ST15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Status Bit"]
    #[inline(always)]
    pub fn st16(&self) -> ST16_R {
        ST16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Status Bit"]
    #[inline(always)]
    pub fn st17(&self) -> ST17_R {
        ST17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Status Bit"]
    #[inline(always)]
    pub fn st18(&self) -> ST18_R {
        ST18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Status Bit"]
    #[inline(always)]
    pub fn st19(&self) -> ST19_R {
        ST19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Status Bit"]
    #[inline(always)]
    pub fn st20(&self) -> ST20_R {
        ST20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Status Bit"]
    #[inline(always)]
    pub fn st21(&self) -> ST21_R {
        ST21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Status Bit"]
    #[inline(always)]
    pub fn st22(&self) -> ST22_R {
        ST22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Status Bit"]
    #[inline(always)]
    pub fn st23(&self) -> ST23_R {
        ST23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Global Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GS_SPEC;
impl crate::RegisterSpec for GS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gs::R`](R) reader structure"]
impl crate::Readable for GS_SPEC {}
#[doc = "`reset()` method sets GS to value 0"]
impl crate::Resettable for GS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
