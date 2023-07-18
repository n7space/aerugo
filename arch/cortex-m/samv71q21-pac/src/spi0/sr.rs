#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDRF` reader - Receive Data Register Full (cleared by reading SPI_RDR)"]
pub type RDRF_R = crate::BitReader;
#[doc = "Field `TDRE` reader - Transmit Data Register Empty (cleared by writing SPI_TDR)"]
pub type TDRE_R = crate::BitReader;
#[doc = "Field `MODF` reader - Mode Fault Error (cleared on read)"]
pub type MODF_R = crate::BitReader;
#[doc = "Field `OVRES` reader - Overrun Error Status (cleared on read)"]
pub type OVRES_R = crate::BitReader;
#[doc = "Field `NSSR` reader - NSS Rising (cleared on read)"]
pub type NSSR_R = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty (cleared by writing SPI_TDR)"]
pub type TXEMPTY_R = crate::BitReader;
#[doc = "Field `UNDES` reader - Underrun Error Status (Slave mode only) (cleared on read)"]
pub type UNDES_R = crate::BitReader;
#[doc = "Field `SPIENS` reader - SPI Enable Status"]
pub type SPIENS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full (cleared by reading SPI_RDR)"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty (cleared by writing SPI_TDR)"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error (cleared on read)"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Status (cleared on read)"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Rising (cleared on read)"]
    #[inline(always)]
    pub fn nssr(&self) -> NSSR_R {
        NSSR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Registers Empty (cleared by writing SPI_TDR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Underrun Error Status (Slave mode only) (cleared on read)"]
    #[inline(always)]
    pub fn undes(&self) -> UNDES_R {
        UNDES_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI Enable Status"]
    #[inline(always)]
    pub fn spiens(&self) -> SPIENS_R {
        SPIENS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
