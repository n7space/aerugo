#[doc = "Register `US_IMR_SPI_MODE` reader"]
pub struct R(crate::R<US_IMR_SPI_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_IMR_SPI_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_IMR_SPI_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_IMR_SPI_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - RXRDY Interrupt Mask"]
pub type RXRDY_R = crate::BitReader;
#[doc = "Field `TXRDY` reader - TXRDY Interrupt Mask"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub type OVRE_R = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - TXEMPTY Interrupt Mask"]
pub type TXEMPTY_R = crate::BitReader;
#[doc = "Field `UNRE` reader - SPI Underrun Error Interrupt Mask"]
pub type UNRE_R = crate::BitReader;
#[doc = "Field `NSSE` reader - NSS Line (Driving CTS Pin) Rising or Falling Edge Event"]
pub type NSSE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 19 - NSS Line (Driving CTS Pin) Rising or Falling Edge Event"]
    #[inline(always)]
    pub fn nsse(&self) -> NSSE_R {
        NSSE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_imr_spi_mode](index.html) module"]
pub struct US_IMR_SPI_MODE_SPEC;
impl crate::RegisterSpec for US_IMR_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_imr_spi_mode::R](R) reader structure"]
impl crate::Readable for US_IMR_SPI_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_IMR_SPI_MODE to value 0"]
impl crate::Resettable for US_IMR_SPI_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
