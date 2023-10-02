#[doc = "Register `US_IDR_LON_MODE` writer"]
pub type W = crate::W<US_IDR_LON_MODE_SPEC>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
pub type RXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
pub type TXRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSFE` writer - LON Short Frame Error Interrupt Disable"]
pub type LSFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCRCE` writer - LON CRC Error Interrupt Disable"]
pub type LCRCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub type TXEMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNRE` writer - SPI Underrun Error Interrupt Disable"]
pub type UNRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LTXD` writer - LON Transmission Done Interrupt Disable"]
pub type LTXD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LCOL` writer - LON Collision Interrupt Disable"]
pub type LCOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LFET` writer - LON Frame Early Termination Interrupt Disable"]
pub type LFET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LRXD` writer - LON Reception Done Interrupt Disable"]
pub type LRXD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LBLOVFE` writer - LON Backlog Overflow Error Interrupt Disable"]
pub type LBLOVFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<US_IDR_LON_MODE_SPEC, 0> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<US_IDR_LON_MODE_SPEC, 1> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<US_IDR_LON_MODE_SPEC, 5> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 6 - LON Short Frame Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lsfe(&mut self) -> LSFE_W<US_IDR_LON_MODE_SPEC, 6> {
        LSFE_W::new(self)
    }
    #[doc = "Bit 7 - LON CRC Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lcrce(&mut self) -> LCRCE_W<US_IDR_LON_MODE_SPEC, 7> {
        LCRCE_W::new(self)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<US_IDR_LON_MODE_SPEC, 9> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UNRE_W<US_IDR_LON_MODE_SPEC, 10> {
        UNRE_W::new(self)
    }
    #[doc = "Bit 24 - LON Transmission Done Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ltxd(&mut self) -> LTXD_W<US_IDR_LON_MODE_SPEC, 24> {
        LTXD_W::new(self)
    }
    #[doc = "Bit 25 - LON Collision Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lcol(&mut self) -> LCOL_W<US_IDR_LON_MODE_SPEC, 25> {
        LCOL_W::new(self)
    }
    #[doc = "Bit 26 - LON Frame Early Termination Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lfet(&mut self) -> LFET_W<US_IDR_LON_MODE_SPEC, 26> {
        LFET_W::new(self)
    }
    #[doc = "Bit 27 - LON Reception Done Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lrxd(&mut self) -> LRXD_W<US_IDR_LON_MODE_SPEC, 27> {
        LRXD_W::new(self)
    }
    #[doc = "Bit 28 - LON Backlog Overflow Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lblovfe(&mut self) -> LBLOVFE_W<US_IDR_LON_MODE_SPEC, 28> {
        LBLOVFE_W::new(self)
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
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`us_idr_lon_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_IDR_LON_MODE_SPEC;
impl crate::RegisterSpec for US_IDR_LON_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`us_idr_lon_mode::W`](W) writer structure"]
impl crate::Writable for US_IDR_LON_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_IDR_LON_MODE to value 0"]
impl crate::Resettable for US_IDR_LON_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
