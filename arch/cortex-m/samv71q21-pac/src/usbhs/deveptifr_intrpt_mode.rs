#[doc = "Register `DEVEPTIFR_INTRPT_MODE[%s]` writer"]
pub type W = crate::W<DEVEPTIFR_INTRPT_MODE_SPEC>;
#[doc = "Field `TXINIS` writer - Transmitted IN Data Interrupt Set"]
pub type TXINIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOUTIS` writer - Received OUT Data Interrupt Set"]
pub type RXOUTIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSTPIS` writer - Received SETUP Interrupt Set"]
pub type RXSTPIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKOUTIS` writer - NAKed OUT Interrupt Set"]
pub type NAKOUTIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKINIS` writer - NAKed IN Interrupt Set"]
pub type NAKINIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OVERFIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALLEDIS` writer - STALLed Interrupt Set"]
pub type STALLEDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHORTPACKETS` writer - Short Packet Interrupt Set"]
pub type SHORTPACKETS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Interrupt Set"]
pub type NBUSYBKS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txinis(&mut self) -> TXINIS_W<DEVEPTIFR_INTRPT_MODE_SPEC, 0> {
        TXINIS_W::new(self)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutis(&mut self) -> RXOUTIS_W<DEVEPTIFR_INTRPT_MODE_SPEC, 1> {
        RXOUTIS_W::new(self)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpis(&mut self) -> RXSTPIS_W<DEVEPTIFR_INTRPT_MODE_SPEC, 2> {
        RXSTPIS_W::new(self)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutis(&mut self) -> NAKOUTIS_W<DEVEPTIFR_INTRPT_MODE_SPEC, 3> {
        NAKOUTIS_W::new(self)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakinis(&mut self) -> NAKINIS_W<DEVEPTIFR_INTRPT_MODE_SPEC, 4> {
        NAKINIS_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OVERFIS_W<DEVEPTIFR_INTRPT_MODE_SPEC, 5> {
        OVERFIS_W::new(self)
    }
    #[doc = "Bit 6 - STALLed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn stalledis(&mut self) -> STALLEDIS_W<DEVEPTIFR_INTRPT_MODE_SPEC, 6> {
        STALLEDIS_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpackets(&mut self) -> SHORTPACKETS_W<DEVEPTIFR_INTRPT_MODE_SPEC, 7> {
        SHORTPACKETS_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W<DEVEPTIFR_INTRPT_MODE_SPEC, 12> {
        NBUSYBKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Endpoint Interrupt Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr_intrpt_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVEPTIFR_INTRPT_MODE_SPEC;
impl crate::RegisterSpec for DEVEPTIFR_INTRPT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptifr_intrpt_mode::W`](W) writer structure"]
impl crate::Writable for DEVEPTIFR_INTRPT_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVEPTIFR_INTRPT_MODE[%s]
to value 0"]
impl crate::Resettable for DEVEPTIFR_INTRPT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
