#[doc = "Register `DEVEPTIDR_ISO_MODE[%s]` writer"]
pub type W = crate::W<DEVEPTIDR_ISO_MODE_SPEC>;
#[doc = "Field `TXINEC` writer - Transmitted IN Interrupt Clear"]
pub type TXINEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOUTEC` writer - Received OUT Data Interrupt Clear"]
pub type RXOUTEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNDERFEC` writer - Underflow Interrupt Clear"]
pub type UNDERFEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HBISOINERREC` writer - High Bandwidth Isochronous IN Underflow Error Interrupt Clear"]
pub type HBISOINERREC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HBISOFLUSHEC` writer - High Bandwidth Isochronous IN Flush Interrupt Clear"]
pub type HBISOFLUSHEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFEC` writer - Overflow Interrupt Clear"]
pub type OVERFEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHORTPACKETEC` writer - Shortpacket Interrupt Clear"]
pub type SHORTPACKETEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MDATAEC` writer - MData Interrupt Clear"]
pub type MDATAEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATAXEC` writer - DataX Interrupt Clear"]
pub type DATAXEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRORTRANSEC` writer - Transaction Error Interrupt Clear"]
pub type ERRORTRANSEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Interrupt Clear"]
pub type NBUSYBKEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Clear"]
pub type FIFOCONC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDISHDMAC` writer - Endpoint Interrupts Disable HDMA Request Clear"]
pub type EPDISHDMAC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinec(&mut self) -> TXINEC_W<DEVEPTIDR_ISO_MODE_SPEC, 0> {
        TXINEC_W::new(self)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutec(&mut self) -> RXOUTEC_W<DEVEPTIDR_ISO_MODE_SPEC, 1> {
        RXOUTEC_W::new(self)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn underfec(&mut self) -> UNDERFEC_W<DEVEPTIDR_ISO_MODE_SPEC, 2> {
        UNDERFEC_W::new(self)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerrec(&mut self) -> HBISOINERREC_W<DEVEPTIDR_ISO_MODE_SPEC, 3> {
        HBISOINERREC_W::new(self)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushec(&mut self) -> HBISOFLUSHEC_W<DEVEPTIDR_ISO_MODE_SPEC, 4> {
        HBISOFLUSHEC_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfec(&mut self) -> OVERFEC_W<DEVEPTIDR_ISO_MODE_SPEC, 5> {
        OVERFEC_W::new(self)
    }
    #[doc = "Bit 7 - Shortpacket Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketec(&mut self) -> SHORTPACKETEC_W<DEVEPTIDR_ISO_MODE_SPEC, 7> {
        SHORTPACKETEC_W::new(self)
    }
    #[doc = "Bit 8 - MData Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn mdataec(&mut self) -> MDATAEC_W<DEVEPTIDR_ISO_MODE_SPEC, 8> {
        MDATAEC_W::new(self)
    }
    #[doc = "Bit 9 - DataX Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dataxec(&mut self) -> DATAXEC_W<DEVEPTIDR_ISO_MODE_SPEC, 9> {
        DATAXEC_W::new(self)
    }
    #[doc = "Bit 10 - Transaction Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn errortransec(&mut self) -> ERRORTRANSEC_W<DEVEPTIDR_ISO_MODE_SPEC, 10> {
        ERRORTRANSEC_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W<DEVEPTIDR_ISO_MODE_SPEC, 12> {
        NBUSYBKEC_W::new(self)
    }
    #[doc = "Bit 14 - FIFO Control Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FIFOCONC_W<DEVEPTIDR_ISO_MODE_SPEC, 14> {
        FIFOCONC_W::new(self)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmac(&mut self) -> EPDISHDMAC_W<DEVEPTIDR_ISO_MODE_SPEC, 16> {
        EPDISHDMAC_W::new(self)
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
#[doc = "Device Endpoint Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr_iso_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVEPTIDR_ISO_MODE_SPEC;
impl crate::RegisterSpec for DEVEPTIDR_ISO_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptidr_iso_mode::W`](W) writer structure"]
impl crate::Writable for DEVEPTIDR_ISO_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVEPTIDR_ISO_MODE[%s]
to value 0"]
impl crate::Resettable for DEVEPTIDR_ISO_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
