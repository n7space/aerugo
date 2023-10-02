#[doc = "Register `US_CR_SPI_MODE` writer"]
pub type W = crate::W<US_CR_SPI_MODE_SPEC>;
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub type RSTRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub type RSTTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RXDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TXDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTSTA` writer - Reset Status Bits"]
pub type RSTSTA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCS` writer - Force SPI Chip Select"]
pub type FCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCS` writer - Release SPI Chip Select"]
pub type RCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rstrx(&mut self) -> RSTRX_W<US_CR_SPI_MODE_SPEC, 2> {
        RSTRX_W::new(self)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn rsttx(&mut self) -> RSTTX_W<US_CR_SPI_MODE_SPEC, 3> {
        RSTTX_W::new(self)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<US_CR_SPI_MODE_SPEC, 4> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RXDIS_W<US_CR_SPI_MODE_SPEC, 5> {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<US_CR_SPI_MODE_SPEC, 6> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TXDIS_W<US_CR_SPI_MODE_SPEC, 7> {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rststa(&mut self) -> RSTSTA_W<US_CR_SPI_MODE_SPEC, 8> {
        RSTSTA_W::new(self)
    }
    #[doc = "Bit 18 - Force SPI Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FCS_W<US_CR_SPI_MODE_SPEC, 18> {
        FCS_W::new(self)
    }
    #[doc = "Bit 19 - Release SPI Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn rcs(&mut self) -> RCS_W<US_CR_SPI_MODE_SPEC, 19> {
        RCS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_cr_spi_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_CR_SPI_MODE_SPEC;
impl crate::RegisterSpec for US_CR_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_cr_spi_mode::W`](W) writer structure"]
impl crate::Writable for US_CR_SPI_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_CR_SPI_MODE to value 0"]
impl crate::Resettable for US_CR_SPI_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
