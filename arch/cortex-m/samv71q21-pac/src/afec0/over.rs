#[doc = "Register `OVER` reader"]
pub type R = crate::R<OVER_SPEC>;
#[doc = "Field `OVRE0` reader - Overrun Error 0"]
pub type OVRE0_R = crate::BitReader;
#[doc = "Field `OVRE1` reader - Overrun Error 1"]
pub type OVRE1_R = crate::BitReader;
#[doc = "Field `OVRE2` reader - Overrun Error 2"]
pub type OVRE2_R = crate::BitReader;
#[doc = "Field `OVRE3` reader - Overrun Error 3"]
pub type OVRE3_R = crate::BitReader;
#[doc = "Field `OVRE4` reader - Overrun Error 4"]
pub type OVRE4_R = crate::BitReader;
#[doc = "Field `OVRE5` reader - Overrun Error 5"]
pub type OVRE5_R = crate::BitReader;
#[doc = "Field `OVRE6` reader - Overrun Error 6"]
pub type OVRE6_R = crate::BitReader;
#[doc = "Field `OVRE7` reader - Overrun Error 7"]
pub type OVRE7_R = crate::BitReader;
#[doc = "Field `OVRE8` reader - Overrun Error 8"]
pub type OVRE8_R = crate::BitReader;
#[doc = "Field `OVRE9` reader - Overrun Error 9"]
pub type OVRE9_R = crate::BitReader;
#[doc = "Field `OVRE10` reader - Overrun Error 10"]
pub type OVRE10_R = crate::BitReader;
#[doc = "Field `OVRE11` reader - Overrun Error 11"]
pub type OVRE11_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Overrun Error 0"]
    #[inline(always)]
    pub fn ovre0(&self) -> OVRE0_R {
        OVRE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error 1"]
    #[inline(always)]
    pub fn ovre1(&self) -> OVRE1_R {
        OVRE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overrun Error 2"]
    #[inline(always)]
    pub fn ovre2(&self) -> OVRE2_R {
        OVRE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error 3"]
    #[inline(always)]
    pub fn ovre3(&self) -> OVRE3_R {
        OVRE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overrun Error 4"]
    #[inline(always)]
    pub fn ovre4(&self) -> OVRE4_R {
        OVRE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error 5"]
    #[inline(always)]
    pub fn ovre5(&self) -> OVRE5_R {
        OVRE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun Error 6"]
    #[inline(always)]
    pub fn ovre6(&self) -> OVRE6_R {
        OVRE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Overrun Error 7"]
    #[inline(always)]
    pub fn ovre7(&self) -> OVRE7_R {
        OVRE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Overrun Error 8"]
    #[inline(always)]
    pub fn ovre8(&self) -> OVRE8_R {
        OVRE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Overrun Error 9"]
    #[inline(always)]
    pub fn ovre9(&self) -> OVRE9_R {
        OVRE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun Error 10"]
    #[inline(always)]
    pub fn ovre10(&self) -> OVRE10_R {
        OVRE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun Error 11"]
    #[inline(always)]
    pub fn ovre11(&self) -> OVRE11_R {
        OVRE11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "AFEC Overrun Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`over::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OVER_SPEC;
impl crate::RegisterSpec for OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`over::R`](R) reader structure"]
impl crate::Readable for OVER_SPEC {}
#[doc = "`reset()` method sets OVER to value 0"]
impl crate::Resettable for OVER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
