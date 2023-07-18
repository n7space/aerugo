#[doc = "Register `IMR2` reader"]
pub struct R(crate::R<IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRDY` reader - Write Ready for Synchronous Channels Update Interrupt Mask"]
pub type WRDY_R = crate::BitReader;
#[doc = "Field `UNRE` reader - Synchronous Channels Update Underrun Error Interrupt Mask"]
pub type UNRE_R = crate::BitReader;
#[doc = "Field `CMPM0` reader - Comparison 0 Match Interrupt Mask"]
pub type CMPM0_R = crate::BitReader;
#[doc = "Field `CMPM1` reader - Comparison 1 Match Interrupt Mask"]
pub type CMPM1_R = crate::BitReader;
#[doc = "Field `CMPM2` reader - Comparison 2 Match Interrupt Mask"]
pub type CMPM2_R = crate::BitReader;
#[doc = "Field `CMPM3` reader - Comparison 3 Match Interrupt Mask"]
pub type CMPM3_R = crate::BitReader;
#[doc = "Field `CMPM4` reader - Comparison 4 Match Interrupt Mask"]
pub type CMPM4_R = crate::BitReader;
#[doc = "Field `CMPM5` reader - Comparison 5 Match Interrupt Mask"]
pub type CMPM5_R = crate::BitReader;
#[doc = "Field `CMPM6` reader - Comparison 6 Match Interrupt Mask"]
pub type CMPM6_R = crate::BitReader;
#[doc = "Field `CMPM7` reader - Comparison 7 Match Interrupt Mask"]
pub type CMPM7_R = crate::BitReader;
#[doc = "Field `CMPU0` reader - Comparison 0 Update Interrupt Mask"]
pub type CMPU0_R = crate::BitReader;
#[doc = "Field `CMPU1` reader - Comparison 1 Update Interrupt Mask"]
pub type CMPU1_R = crate::BitReader;
#[doc = "Field `CMPU2` reader - Comparison 2 Update Interrupt Mask"]
pub type CMPU2_R = crate::BitReader;
#[doc = "Field `CMPU3` reader - Comparison 3 Update Interrupt Mask"]
pub type CMPU3_R = crate::BitReader;
#[doc = "Field `CMPU4` reader - Comparison 4 Update Interrupt Mask"]
pub type CMPU4_R = crate::BitReader;
#[doc = "Field `CMPU5` reader - Comparison 5 Update Interrupt Mask"]
pub type CMPU5_R = crate::BitReader;
#[doc = "Field `CMPU6` reader - Comparison 6 Update Interrupt Mask"]
pub type CMPU6_R = crate::BitReader;
#[doc = "Field `CMPU7` reader - Comparison 7 Update Interrupt Mask"]
pub type CMPU7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write Ready for Synchronous Channels Update Interrupt Mask"]
    #[inline(always)]
    pub fn wrdy(&self) -> WRDY_R {
        WRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronous Channels Update Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparison 0 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm0(&self) -> CMPM0_R {
        CMPM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparison 1 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm1(&self) -> CMPM1_R {
        CMPM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Comparison 2 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm2(&self) -> CMPM2_R {
        CMPM2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Comparison 3 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm3(&self) -> CMPM3_R {
        CMPM3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparison 4 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm4(&self) -> CMPM4_R {
        CMPM4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Comparison 5 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm5(&self) -> CMPM5_R {
        CMPM5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparison 6 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm6(&self) -> CMPM6_R {
        CMPM6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparison 7 Match Interrupt Mask"]
    #[inline(always)]
    pub fn cmpm7(&self) -> CMPM7_R {
        CMPM7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Comparison 0 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu0(&self) -> CMPU0_R {
        CMPU0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Comparison 1 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu1(&self) -> CMPU1_R {
        CMPU1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Comparison 2 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu2(&self) -> CMPU2_R {
        CMPU2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Comparison 3 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu3(&self) -> CMPU3_R {
        CMPU3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Comparison 4 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu4(&self) -> CMPU4_R {
        CMPU4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Comparison 5 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu5(&self) -> CMPU5_R {
        CMPU5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Comparison 6 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu6(&self) -> CMPU6_R {
        CMPU6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Comparison 7 Update Interrupt Mask"]
    #[inline(always)]
    pub fn cmpu7(&self) -> CMPU7_R {
        CMPU7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "PWM Interrupt Mask Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr2](index.html) module"]
pub struct IMR2_SPEC;
impl crate::RegisterSpec for IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr2::R](R) reader structure"]
impl crate::Readable for IMR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR2 to value 0"]
impl crate::Resettable for IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
