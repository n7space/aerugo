#[doc = "Register `HSTPIPIDR_CTRL_MODE[%s]` writer"]
pub type W = crate::W<HSTPIPIDR_CTRL_MODE_SPEC>;
#[doc = "Field `RXINEC` writer - Received IN Data Interrupt Disable"]
pub type RXINEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXOUTEC` writer - Transmitted OUT Data Interrupt Disable"]
pub type TXOUTEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXSTPEC` writer - Transmitted SETUP Interrupt Disable"]
pub type TXSTPEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERREC` writer - Pipe Error Interrupt Disable"]
pub type PERREC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKEDEC` writer - NAKed Interrupt Disable"]
pub type NAKEDEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFIEC` writer - Overflow Interrupt Disable"]
pub type OVERFIEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSTALLDEC` writer - Received STALLed Interrupt Disable"]
pub type RXSTALLDEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHORTPACKETIEC` writer - Short Packet Interrupt Disable"]
pub type SHORTPACKETIEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NBUSYBKEC` writer - Number of Busy Banks Disable"]
pub type NBUSYBKEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOCONC` writer - FIFO Control Disable"]
pub type FIFOCONC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDISHDMAC` writer - Pipe Interrupts Disable HDMA Request Disable"]
pub type PDISHDMAC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PFREEZEC` writer - Pipe Freeze Disable"]
pub type PFREEZEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxinec(&mut self) -> RXINEC_W<HSTPIPIDR_CTRL_MODE_SPEC, 0> {
        RXINEC_W::new(self)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txoutec(&mut self) -> TXOUTEC_W<HSTPIPIDR_CTRL_MODE_SPEC, 1> {
        TXOUTEC_W::new(self)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txstpec(&mut self) -> TXSTPEC_W<HSTPIPIDR_CTRL_MODE_SPEC, 2> {
        TXSTPEC_W::new(self)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn perrec(&mut self) -> PERREC_W<HSTPIPIDR_CTRL_MODE_SPEC, 3> {
        PERREC_W::new(self)
    }
    #[doc = "Bit 4 - NAKed Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nakedec(&mut self) -> NAKEDEC_W<HSTPIPIDR_CTRL_MODE_SPEC, 4> {
        NAKEDEC_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn overfiec(&mut self) -> OVERFIEC_W<HSTPIPIDR_CTRL_MODE_SPEC, 5> {
        OVERFIEC_W::new(self)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldec(&mut self) -> RXSTALLDEC_W<HSTPIPIDR_CTRL_MODE_SPEC, 6> {
        RXSTALLDEC_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketiec(&mut self) -> SHORTPACKETIEC_W<HSTPIPIDR_CTRL_MODE_SPEC, 7> {
        SHORTPACKETIEC_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W<HSTPIPIDR_CTRL_MODE_SPEC, 12> {
        NBUSYBKEC_W::new(self)
    }
    #[doc = "Bit 14 - FIFO Control Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoconc(&mut self) -> FIFOCONC_W<HSTPIPIDR_CTRL_MODE_SPEC, 14> {
        FIFOCONC_W::new(self)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pdishdmac(&mut self) -> PDISHDMAC_W<HSTPIPIDR_CTRL_MODE_SPEC, 16> {
        PDISHDMAC_W::new(self)
    }
    #[doc = "Bit 17 - Pipe Freeze Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pfreezec(&mut self) -> PFREEZEC_W<HSTPIPIDR_CTRL_MODE_SPEC, 17> {
        PFREEZEC_W::new(self)
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
#[doc = "Host Pipe Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr_ctrl_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTPIPIDR_CTRL_MODE_SPEC;
impl crate::RegisterSpec for HSTPIPIDR_CTRL_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipidr_ctrl_mode::W`](W) writer structure"]
impl crate::Writable for HSTPIPIDR_CTRL_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTPIPIDR_CTRL_MODE[%s]
to value 0"]
impl crate::Resettable for HSTPIPIDR_CTRL_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
