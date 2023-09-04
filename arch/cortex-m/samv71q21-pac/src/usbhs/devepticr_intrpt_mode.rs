#[doc = "Register `DEVEPTICR_INTRPT_MODE[%s]` writer"]
pub type W = crate::W<DEVEPTICR_INTRPT_MODE_SPEC>;
#[doc = "Field `TXINIC` writer - Transmitted IN Data Interrupt Clear"]
pub type TXINIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOUTIC` writer - Received OUT Data Interrupt Clear"]
pub type RXOUTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSTPIC` writer - Received SETUP Interrupt Clear"]
pub type RXSTPIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKOUTIC` writer - NAKed OUT Interrupt Clear"]
pub type NAKOUTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKINIC` writer - NAKed IN Interrupt Clear"]
pub type NAKINIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OVERFIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALLEDIC` writer - STALLed Interrupt Clear"]
pub type STALLEDIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHORTPACKETC` writer - Short Packet Interrupt Clear"]
pub type SHORTPACKETC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinic(&mut self) -> TXINIC_W<DEVEPTICR_INTRPT_MODE_SPEC, 0> {
        TXINIC_W::new(self)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutic(&mut self) -> RXOUTIC_W<DEVEPTICR_INTRPT_MODE_SPEC, 1> {
        RXOUTIC_W::new(self)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpic(&mut self) -> RXSTPIC_W<DEVEPTICR_INTRPT_MODE_SPEC, 2> {
        RXSTPIC_W::new(self)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutic(&mut self) -> NAKOUTIC_W<DEVEPTICR_INTRPT_MODE_SPEC, 3> {
        NAKOUTIC_W::new(self)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakinic(&mut self) -> NAKINIC_W<DEVEPTICR_INTRPT_MODE_SPEC, 4> {
        NAKINIC_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfic(&mut self) -> OVERFIC_W<DEVEPTICR_INTRPT_MODE_SPEC, 5> {
        OVERFIC_W::new(self)
    }
    #[doc = "Bit 6 - STALLed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stalledic(&mut self) -> STALLEDIC_W<DEVEPTICR_INTRPT_MODE_SPEC, 6> {
        STALLEDIC_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketc(&mut self) -> SHORTPACKETC_W<DEVEPTICR_INTRPT_MODE_SPEC, 7> {
        SHORTPACKETC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Endpoint Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr_intrpt_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVEPTICR_INTRPT_MODE_SPEC;
impl crate::RegisterSpec for DEVEPTICR_INTRPT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devepticr_intrpt_mode::W`](W) writer structure"]
impl crate::Writable for DEVEPTICR_INTRPT_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVEPTICR_INTRPT_MODE[%s]
to value 0"]
impl crate::Resettable for DEVEPTICR_INTRPT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
