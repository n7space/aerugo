#[doc = "Register `DEVEPTIFR_ISO_MODE[%s]` writer"]
pub struct W(crate::W<DEVEPTIFR_ISO_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVEPTIFR_ISO_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DEVEPTIFR_ISO_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVEPTIFR_ISO_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINIS` writer - Transmitted IN Data Interrupt Set"]
pub type TXINIS_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIFR_ISO_MODE_SPEC, O>;
#[doc = "Field `RXOUTIS` writer - Received OUT Data Interrupt Set"]
pub type RXOUTIS_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIFR_ISO_MODE_SPEC, O>;
#[doc = "Field `UNDERFIS` writer - Underflow Interrupt Set"]
pub type UNDERFIS_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIFR_ISO_MODE_SPEC, O>;
#[doc = "Field `HBISOINERRIS` writer - High Bandwidth Isochronous IN Underflow Error Interrupt Set"]
pub type HBISOINERRIS_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIFR_ISO_MODE_SPEC, O>;
#[doc = "Field `HBISOFLUSHIS` writer - High Bandwidth Isochronous IN Flush Interrupt Set"]
pub type HBISOFLUSHIS_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIFR_ISO_MODE_SPEC, O>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OVERFIS_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIFR_ISO_MODE_SPEC, O>;
#[doc = "Field `CRCERRIS` writer - CRC Error Interrupt Set"]
pub type CRCERRIS_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIFR_ISO_MODE_SPEC, O>;
#[doc = "Field `SHORTPACKETS` writer - Short Packet Interrupt Set"]
pub type SHORTPACKETS_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIFR_ISO_MODE_SPEC, O>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Interrupt Set"]
pub type NBUSYBKS_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIFR_ISO_MODE_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txinis(&mut self) -> TXINIS_W<0> {
        TXINIS_W::new(self)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutis(&mut self) -> RXOUTIS_W<1> {
        RXOUTIS_W::new(self)
    }
    #[doc = "Bit 2 - Underflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn underfis(&mut self) -> UNDERFIS_W<2> {
        UNDERFIS_W::new(self)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerris(&mut self) -> HBISOINERRIS_W<3> {
        HBISOINERRIS_W::new(self)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushis(&mut self) -> HBISOFLUSHIS_W<4> {
        HBISOFLUSHIS_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OVERFIS_W<5> {
        OVERFIS_W::new(self)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn crcerris(&mut self) -> CRCERRIS_W<6> {
        CRCERRIS_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpackets(&mut self) -> SHORTPACKETS_W<7> {
        SHORTPACKETS_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W<12> {
        NBUSYBKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Interrupt Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptifr_iso_mode](index.html) module"]
pub struct DEVEPTIFR_ISO_MODE_SPEC;
impl crate::RegisterSpec for DEVEPTIFR_ISO_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [deveptifr_iso_mode::W](W) writer structure"]
impl crate::Writable for DEVEPTIFR_ISO_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVEPTIFR_ISO_MODE[%s]
to value 0"]
impl crate::Resettable for DEVEPTIFR_ISO_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
