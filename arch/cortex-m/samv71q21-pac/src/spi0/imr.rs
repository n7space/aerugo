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
#[doc = "Field `RDRF` reader - Receive Data Register Full Interrupt Mask"]
pub type RDRF_R = crate::BitReader;
#[doc = "Field `TDRE` reader - SPI Transmit Data Register Empty Interrupt Mask"]
pub type TDRE_R = crate::BitReader;
#[doc = "Field `MODF` reader - Mode Fault Error Interrupt Mask"]
pub type MODF_R = crate::BitReader;
#[doc = "Field `OVRES` reader - Overrun Error Interrupt Mask"]
pub type OVRES_R = crate::BitReader;
#[doc = "Field `NSSR` reader - NSS Rising Interrupt Mask"]
pub type NSSR_R = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty Mask"]
pub type TXEMPTY_R = crate::BitReader;
#[doc = "Field `UNDES` reader - Underrun Error Interrupt Mask"]
pub type UNDES_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Mask"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Transmit Data Register Empty Interrupt Mask"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Mask"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Mask"]
    #[inline(always)]
    pub fn nssr(&self) -> NSSR_R {
        NSSR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Registers Empty Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn undes(&self) -> UNDES_R {
        UNDES_R::new(((self.bits >> 10) & 1) != 0)
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
