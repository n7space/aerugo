#[doc = "Register `HSTPIPIDR_INTRPT_MODE[%s]` writer"]
pub struct W(crate::W<HSTPIPIDR_INTRPT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTPIPIDR_INTRPT_MODE_SPEC>;
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
impl From<crate::W<HSTPIPIDR_INTRPT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTPIPIDR_INTRPT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINEC` writer - Received IN Data Interrupt Disable"]
pub type RXINEC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIDR_INTRPT_MODE_SPEC, O>;
#[doc = "Field `TXOUTEC` writer - Transmitted OUT Data Interrupt Disable"]
pub type TXOUTEC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIDR_INTRPT_MODE_SPEC, O>;
#[doc = "Field `UNDERFIEC` writer - Underflow Interrupt Disable"]
pub type UNDERFIEC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIDR_INTRPT_MODE_SPEC, O>;
#[doc = "Field `PERREC` writer - Pipe Error Interrupt Disable"]
pub type PERREC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIDR_INTRPT_MODE_SPEC, O>;
#[doc = "Field `NAKEDEC` writer - NAKed Interrupt Disable"]
pub type NAKEDEC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIDR_INTRPT_MODE_SPEC, O>;
#[doc = "Field `OVERFIEC` writer - Overflow Interrupt Disable"]
pub type OVERFIEC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIDR_INTRPT_MODE_SPEC, O>;
#[doc = "Field `RXSTALLDEC` writer - Received STALLed Interrupt Disable"]
pub type RXSTALLDEC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIDR_INTRPT_MODE_SPEC, O>;
#[doc = "Field `SHORTPACKETIEC` writer - Short Packet Interrupt Disable"]
pub type SHORTPACKETIEC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIDR_INTRPT_MODE_SPEC, O>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Disable"]
pub type NBUSYBKEC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIDR_INTRPT_MODE_SPEC, O>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Disable"]
pub type FIFOCONC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIDR_INTRPT_MODE_SPEC, O>;
#[doc = "Field `PDISHDMAC` writer - Pipe Interrupts Disable HDMA Request Disable"]
pub type PDISHDMAC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIDR_INTRPT_MODE_SPEC, O>;
#[doc = "Field `PFREEZEC` writer - Pipe Freeze Disable"]
pub type PFREEZEC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIDR_INTRPT_MODE_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxinec(&mut self) -> RXINEC_W<0> {
        RXINEC_W::new(self)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txoutec(&mut self) -> TXOUTEC_W<1> {
        TXOUTEC_W::new(self)
    }
    #[doc = "Bit 2 - Underflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn underfiec(&mut self) -> UNDERFIEC_W<2> {
        UNDERFIEC_W::new(self)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn perrec(&mut self) -> PERREC_W<3> {
        PERREC_W::new(self)
    }
    #[doc = "Bit 4 - NAKed Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nakedec(&mut self) -> NAKEDEC_W<4> {
        NAKEDEC_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn overfiec(&mut self) -> OVERFIEC_W<5> {
        OVERFIEC_W::new(self)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldec(&mut self) -> RXSTALLDEC_W<6> {
        RXSTALLDEC_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketiec(&mut self) -> SHORTPACKETIEC_W<7> {
        SHORTPACKETIEC_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W<12> {
        NBUSYBKEC_W::new(self)
    }
    #[doc = "Bit 14 - FIFO Control Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FIFOCONC_W<14> {
        FIFOCONC_W::new(self)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pdishdmac(&mut self) -> PDISHDMAC_W<16> {
        PDISHDMAC_W::new(self)
    }
    #[doc = "Bit 17 - Pipe Freeze Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pfreezec(&mut self) -> PFREEZEC_W<17> {
        PFREEZEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipidr_intrpt_mode](index.html) module"]
pub struct HSTPIPIDR_INTRPT_MODE_SPEC;
impl crate::RegisterSpec for HSTPIPIDR_INTRPT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hstpipidr_intrpt_mode::W](W) writer structure"]
impl crate::Writable for HSTPIPIDR_INTRPT_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTPIPIDR_INTRPT_MODE[%s]
to value 0"]
impl crate::Resettable for HSTPIPIDR_INTRPT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
