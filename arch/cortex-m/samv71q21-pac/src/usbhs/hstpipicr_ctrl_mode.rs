#[doc = "Register `HSTPIPICR_CTRL_MODE[%s]` writer"]
pub type W = crate::W<HSTPIPICR_CTRL_MODE_SPEC>;
#[doc = "Field `RXINIC` writer - Received IN Data Interrupt Clear"]
pub type RXINIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXOUTIC` writer - Transmitted OUT Data Interrupt Clear"]
pub type TXOUTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXSTPIC` writer - Transmitted SETUP Interrupt Clear"]
pub type TXSTPIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKEDIC` writer - NAKed Interrupt Clear"]
pub type NAKEDIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OVERFIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSTALLDIC` writer - Received STALLed Interrupt Clear"]
pub type RXSTALLDIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHORTPACKETIC` writer - Short Packet Interrupt Clear"]
pub type SHORTPACKETIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxinic(&mut self) -> RXINIC_W<HSTPIPICR_CTRL_MODE_SPEC, 0> {
        RXINIC_W::new(self)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txoutic(&mut self) -> TXOUTIC_W<HSTPIPICR_CTRL_MODE_SPEC, 1> {
        TXOUTIC_W::new(self)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txstpic(&mut self) -> TXSTPIC_W<HSTPIPICR_CTRL_MODE_SPEC, 2> {
        TXSTPIC_W::new(self)
    }
    #[doc = "Bit 4 - NAKed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakedic(&mut self) -> NAKEDIC_W<HSTPIPICR_CTRL_MODE_SPEC, 4> {
        NAKEDIC_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfic(&mut self) -> OVERFIC_W<HSTPIPICR_CTRL_MODE_SPEC, 5> {
        OVERFIC_W::new(self)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldic(&mut self) -> RXSTALLDIC_W<HSTPIPICR_CTRL_MODE_SPEC, 6> {
        RXSTALLDIC_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketic(&mut self) -> SHORTPACKETIC_W<HSTPIPICR_CTRL_MODE_SPEC, 7> {
        SHORTPACKETIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Pipe Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr_ctrl_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTPIPICR_CTRL_MODE_SPEC;
impl crate::RegisterSpec for HSTPIPICR_CTRL_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hstpipicr_ctrl_mode::W`](W) writer structure"]
impl crate::Writable for HSTPIPICR_CTRL_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTPIPICR_CTRL_MODE[%s]
to value 0"]
impl crate::Resettable for HSTPIPICR_CTRL_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
