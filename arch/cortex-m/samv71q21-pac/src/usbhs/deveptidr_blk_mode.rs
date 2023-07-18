#[doc = "Register `DEVEPTIDR_BLK_MODE[%s]` writer"]
pub struct W(crate::W<DEVEPTIDR_BLK_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVEPTIDR_BLK_MODE_SPEC>;
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
impl From<crate::W<DEVEPTIDR_BLK_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVEPTIDR_BLK_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINEC` writer - Transmitted IN Interrupt Clear"]
pub type TXINEC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
#[doc = "Field `RXOUTEC` writer - Received OUT Data Interrupt Clear"]
pub type RXOUTEC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
#[doc = "Field `RXSTPEC` writer - Received SETUP Interrupt Clear"]
pub type RXSTPEC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
#[doc = "Field `NAKOUTEC` writer - NAKed OUT Interrupt Clear"]
pub type NAKOUTEC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
#[doc = "Field `NAKINEC` writer - NAKed IN Interrupt Clear"]
pub type NAKINEC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
#[doc = "Field `OVERFEC` writer - Overflow Interrupt Clear"]
pub type OVERFEC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
#[doc = "Field `STALLEDEC` writer - STALLed Interrupt Clear"]
pub type STALLEDEC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
#[doc = "Field `SHORTPACKETEC` writer - Shortpacket Interrupt Clear"]
pub type SHORTPACKETEC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Interrupt Clear"]
pub type NBUSYBKEC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Clear"]
pub type FIFOCONC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
#[doc = "Field `EPDISHDMAC` writer - Endpoint Interrupts Disable HDMA Request Clear"]
pub type EPDISHDMAC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
#[doc = "Field `NYETDISC` writer - NYET Token Disable Clear"]
pub type NYETDISC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
#[doc = "Field `STALLRQC` writer - STALL Request Clear"]
pub type STALLRQC_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPTIDR_BLK_MODE_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinec(&mut self) -> TXINEC_W<0> {
        TXINEC_W::new(self)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutec(&mut self) -> RXOUTEC_W<1> {
        RXOUTEC_W::new(self)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpec(&mut self) -> RXSTPEC_W<2> {
        RXSTPEC_W::new(self)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutec(&mut self) -> NAKOUTEC_W<3> {
        NAKOUTEC_W::new(self)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakinec(&mut self) -> NAKINEC_W<4> {
        NAKINEC_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfec(&mut self) -> OVERFEC_W<5> {
        OVERFEC_W::new(self)
    }
    #[doc = "Bit 6 - STALLed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stalledec(&mut self) -> STALLEDEC_W<6> {
        STALLEDEC_W::new(self)
    }
    #[doc = "Bit 7 - Shortpacket Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketec(&mut self) -> SHORTPACKETEC_W<7> {
        SHORTPACKETEC_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W<12> {
        NBUSYBKEC_W::new(self)
    }
    #[doc = "Bit 14 - FIFO Control Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FIFOCONC_W<14> {
        FIFOCONC_W::new(self)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmac(&mut self) -> EPDISHDMAC_W<16> {
        EPDISHDMAC_W::new(self)
    }
    #[doc = "Bit 17 - NYET Token Disable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nyetdisc(&mut self) -> NYETDISC_W<17> {
        NYETDISC_W::new(self)
    }
    #[doc = "Bit 19 - STALL Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqc(&mut self) -> STALLRQC_W<19> {
        STALLRQC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptidr_blk_mode](index.html) module"]
pub struct DEVEPTIDR_BLK_MODE_SPEC;
impl crate::RegisterSpec for DEVEPTIDR_BLK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [deveptidr_blk_mode::W](W) writer structure"]
impl crate::Writable for DEVEPTIDR_BLK_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVEPTIDR_BLK_MODE[%s]
to value 0"]
impl crate::Resettable for DEVEPTIDR_BLK_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
