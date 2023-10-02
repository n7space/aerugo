#[doc = "Register `DEVEPTICR_ISO_MODE[%s]` writer"]
pub type W = crate::W<DEVEPTICR_ISO_MODE_SPEC>;
#[doc = "Field `TXINIC` writer - Transmitted IN Data Interrupt Clear"]
pub type TXINIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOUTIC` writer - Received OUT Data Interrupt Clear"]
pub type RXOUTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNDERFIC` writer - Underflow Interrupt Clear"]
pub type UNDERFIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HBISOINERRIC` writer - High Bandwidth Isochronous IN Underflow Error Interrupt Clear"]
pub type HBISOINERRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HBISOFLUSHIC` writer - High Bandwidth Isochronous IN Flush Interrupt Clear"]
pub type HBISOFLUSHIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OVERFIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCERRIC` writer - CRC Error Interrupt Clear"]
pub type CRCERRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHORTPACKETC` writer - Short Packet Interrupt Clear"]
pub type SHORTPACKETC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinic(&mut self) -> TXINIC_W<DEVEPTICR_ISO_MODE_SPEC, 0> {
        TXINIC_W::new(self)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutic(&mut self) -> RXOUTIC_W<DEVEPTICR_ISO_MODE_SPEC, 1> {
        RXOUTIC_W::new(self)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn underfic(&mut self) -> UNDERFIC_W<DEVEPTICR_ISO_MODE_SPEC, 2> {
        UNDERFIC_W::new(self)
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Underflow Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerric(&mut self) -> HBISOINERRIC_W<DEVEPTICR_ISO_MODE_SPEC, 3> {
        HBISOINERRIC_W::new(self)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushic(&mut self) -> HBISOFLUSHIC_W<DEVEPTICR_ISO_MODE_SPEC, 4> {
        HBISOFLUSHIC_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfic(&mut self) -> OVERFIC_W<DEVEPTICR_ISO_MODE_SPEC, 5> {
        OVERFIC_W::new(self)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn crcerric(&mut self) -> CRCERRIC_W<DEVEPTICR_ISO_MODE_SPEC, 6> {
        CRCERRIC_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketc(&mut self) -> SHORTPACKETC_W<DEVEPTICR_ISO_MODE_SPEC, 7> {
        SHORTPACKETC_W::new(self)
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
#[doc = "Device Endpoint Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr_iso_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVEPTICR_ISO_MODE_SPEC;
impl crate::RegisterSpec for DEVEPTICR_ISO_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devepticr_iso_mode::W`](W) writer structure"]
impl crate::Writable for DEVEPTICR_ISO_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVEPTICR_ISO_MODE[%s]
to value 0"]
impl crate::Resettable for DEVEPTICR_ISO_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
