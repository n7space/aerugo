#[doc = "Register `SCSR` reader"]
pub type R = crate::R<SCSR_SPEC>;
#[doc = "Field `HCLKS` reader - HCLK Status"]
pub type HCLKS_R = crate::BitReader;
#[doc = "Field `USBCLK` reader - USB FS Clock Status"]
pub type USBCLK_R = crate::BitReader;
#[doc = "Field `PCK0` reader - Programmable Clock 0 Output Status"]
pub type PCK0_R = crate::BitReader;
#[doc = "Field `PCK1` reader - Programmable Clock 1 Output Status"]
pub type PCK1_R = crate::BitReader;
#[doc = "Field `PCK2` reader - Programmable Clock 2 Output Status"]
pub type PCK2_R = crate::BitReader;
#[doc = "Field `PCK3` reader - Programmable Clock 3 Output Status"]
pub type PCK3_R = crate::BitReader;
#[doc = "Field `PCK4` reader - Programmable Clock 4 Output Status"]
pub type PCK4_R = crate::BitReader;
#[doc = "Field `PCK5` reader - Programmable Clock 5 Output Status"]
pub type PCK5_R = crate::BitReader;
#[doc = "Field `PCK6` reader - Programmable Clock 6 Output Status"]
pub type PCK6_R = crate::BitReader;
#[doc = "Field `PCK7` reader - Programmable Clock 7 Output Status"]
pub type PCK7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HCLK Status"]
    #[inline(always)]
    pub fn hclks(&self) -> HCLKS_R {
        HCLKS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - USB FS Clock Status"]
    #[inline(always)]
    pub fn usbclk(&self) -> USBCLK_R {
        USBCLK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Status"]
    #[inline(always)]
    pub fn pck0(&self) -> PCK0_R {
        PCK0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Status"]
    #[inline(always)]
    pub fn pck1(&self) -> PCK1_R {
        PCK1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Status"]
    #[inline(always)]
    pub fn pck2(&self) -> PCK2_R {
        PCK2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Programmable Clock 3 Output Status"]
    #[inline(always)]
    pub fn pck3(&self) -> PCK3_R {
        PCK3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Programmable Clock 4 Output Status"]
    #[inline(always)]
    pub fn pck4(&self) -> PCK4_R {
        PCK4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Programmable Clock 5 Output Status"]
    #[inline(always)]
    pub fn pck5(&self) -> PCK5_R {
        PCK5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Programmable Clock 6 Output Status"]
    #[inline(always)]
    pub fn pck6(&self) -> PCK6_R {
        PCK6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Programmable Clock 7 Output Status"]
    #[inline(always)]
    pub fn pck7(&self) -> PCK7_R {
        PCK7_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "System Clock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCSR_SPEC;
impl crate::RegisterSpec for SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scsr::R`](R) reader structure"]
impl crate::Readable for SCSR_SPEC {}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for SCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
