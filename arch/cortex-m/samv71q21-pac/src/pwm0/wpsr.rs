#[doc = "Register `WPSR` reader"]
pub type R = crate::R<WPSR_SPEC>;
#[doc = "Field `WPSWS0` reader - Write Protect SW Status"]
pub type WPSWS0_R = crate::BitReader;
#[doc = "Field `WPSWS1` reader - Write Protect SW Status"]
pub type WPSWS1_R = crate::BitReader;
#[doc = "Field `WPSWS2` reader - Write Protect SW Status"]
pub type WPSWS2_R = crate::BitReader;
#[doc = "Field `WPSWS3` reader - Write Protect SW Status"]
pub type WPSWS3_R = crate::BitReader;
#[doc = "Field `WPSWS4` reader - Write Protect SW Status"]
pub type WPSWS4_R = crate::BitReader;
#[doc = "Field `WPSWS5` reader - Write Protect SW Status"]
pub type WPSWS5_R = crate::BitReader;
#[doc = "Field `WPVS` reader - Write Protect Violation Status"]
pub type WPVS_R = crate::BitReader;
#[doc = "Field `WPHWS0` reader - Write Protect HW Status"]
pub type WPHWS0_R = crate::BitReader;
#[doc = "Field `WPHWS1` reader - Write Protect HW Status"]
pub type WPHWS1_R = crate::BitReader;
#[doc = "Field `WPHWS2` reader - Write Protect HW Status"]
pub type WPHWS2_R = crate::BitReader;
#[doc = "Field `WPHWS3` reader - Write Protect HW Status"]
pub type WPHWS3_R = crate::BitReader;
#[doc = "Field `WPHWS4` reader - Write Protect HW Status"]
pub type WPHWS4_R = crate::BitReader;
#[doc = "Field `WPHWS5` reader - Write Protect HW Status"]
pub type WPHWS5_R = crate::BitReader;
#[doc = "Field `WPVSRC` reader - Write Protect Violation Source"]
pub type WPVSRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws0(&self) -> WPSWS0_R {
        WPSWS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws1(&self) -> WPSWS1_R {
        WPSWS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws2(&self) -> WPSWS2_R {
        WPSWS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws3(&self) -> WPSWS3_R {
        WPSWS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws4(&self) -> WPSWS4_R {
        WPSWS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws5(&self) -> WPSWS5_R {
        WPSWS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Protect Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws0(&self) -> WPHWS0_R {
        WPHWS0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws1(&self) -> WPHWS1_R {
        WPHWS1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws2(&self) -> WPHWS2_R {
        WPHWS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws3(&self) -> WPHWS3_R {
        WPHWS3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws4(&self) -> WPHWS4_R {
        WPHWS4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws5(&self) -> WPHWS5_R {
        WPHWS5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Write Protect Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "PWM Write Protection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPSR_SPEC;
impl crate::RegisterSpec for WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsr::R`](R) reader structure"]
impl crate::Readable for WPSR_SPEC {}
#[doc = "`reset()` method sets WPSR to value 0"]
impl crate::Resettable for WPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
