#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MOSCXTS` reader - Main Crystal Oscillator Status Interrupt Mask"]
pub type MOSCXTS_R = crate::BitReader;
#[doc = "Field `LOCKA` reader - PLLA Lock Interrupt Mask"]
pub type LOCKA_R = crate::BitReader;
#[doc = "Field `MCKRDY` reader - Master Clock Ready Interrupt Mask"]
pub type MCKRDY_R = crate::BitReader;
#[doc = "Field `LOCKU` reader - UTMI PLL Lock Interrupt Mask"]
pub type LOCKU_R = crate::BitReader;
#[doc = "Field `PCKRDY0` reader - Programmable Clock Ready 0 Interrupt Mask"]
pub type PCKRDY0_R = crate::BitReader;
#[doc = "Field `PCKRDY1` reader - Programmable Clock Ready 1 Interrupt Mask"]
pub type PCKRDY1_R = crate::BitReader;
#[doc = "Field `PCKRDY2` reader - Programmable Clock Ready 2 Interrupt Mask"]
pub type PCKRDY2_R = crate::BitReader;
#[doc = "Field `PCKRDY3` reader - Programmable Clock Ready 3 Interrupt Mask"]
pub type PCKRDY3_R = crate::BitReader;
#[doc = "Field `PCKRDY4` reader - Programmable Clock Ready 4 Interrupt Mask"]
pub type PCKRDY4_R = crate::BitReader;
#[doc = "Field `PCKRDY5` reader - Programmable Clock Ready 5 Interrupt Mask"]
pub type PCKRDY5_R = crate::BitReader;
#[doc = "Field `PCKRDY6` reader - Programmable Clock Ready 6 Interrupt Mask"]
pub type PCKRDY6_R = crate::BitReader;
#[doc = "Field `PCKRDY7` reader - Programmable Clock Ready 7 Interrupt Mask"]
pub type PCKRDY7_R = crate::BitReader;
#[doc = "Field `MOSCSELS` reader - Main Clock Source Oscillator Selection Status Interrupt Mask"]
pub type MOSCSELS_R = crate::BitReader;
#[doc = "Field `MOSCRCS` reader - Main RC Status Interrupt Mask"]
pub type MOSCRCS_R = crate::BitReader;
#[doc = "Field `CFDEV` reader - Clock Failure Detector Event Interrupt Mask"]
pub type CFDEV_R = crate::BitReader;
#[doc = "Field `XT32KERR` reader - 32.768 kHz Crystal Oscillator Error Interrupt Mask"]
pub type XT32KERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscxts(&self) -> MOSCXTS_R {
        MOSCXTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Mask"]
    #[inline(always)]
    pub fn locka(&self) -> LOCKA_R {
        LOCKA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Mask"]
    #[inline(always)]
    pub fn mckrdy(&self) -> MCKRDY_R {
        MCKRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - UTMI PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn locku(&self) -> LOCKU_R {
        LOCKU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy0(&self) -> PCKRDY0_R {
        PCKRDY0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy1(&self) -> PCKRDY1_R {
        PCKRDY1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy2(&self) -> PCKRDY2_R {
        PCKRDY2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Programmable Clock Ready 3 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy3(&self) -> PCKRDY3_R {
        PCKRDY3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Programmable Clock Ready 4 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy4(&self) -> PCKRDY4_R {
        PCKRDY4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Programmable Clock Ready 5 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy5(&self) -> PCKRDY5_R {
        PCKRDY5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Programmable Clock Ready 6 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy6(&self) -> PCKRDY6_R {
        PCKRDY6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Programmable Clock Ready 7 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy7(&self) -> PCKRDY7_R {
        PCKRDY7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Main Clock Source Oscillator Selection Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscsels(&self) -> MOSCSELS_R {
        MOSCSELS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Main RC Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscrcs(&self) -> MOSCRCS_R {
        MOSCRCS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Mask"]
    #[inline(always)]
    pub fn cfdev(&self) -> CFDEV_R {
        CFDEV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - 32.768 kHz Crystal Oscillator Error Interrupt Mask"]
    #[inline(always)]
    pub fn xt32kerr(&self) -> XT32KERR_R {
        XT32KERR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
