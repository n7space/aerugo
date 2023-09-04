#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `START` writer - Send a START Condition"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOP` writer - Send a STOP Condition"]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSEN` writer - TWIHS Master Mode Enabled"]
pub type MSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MSDIS` writer - TWIHS Master Mode Disabled"]
pub type MSDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SVEN` writer - TWIHS Slave Mode Enabled"]
pub type SVEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SVDIS` writer - TWIHS Slave Mode Disabled"]
pub type SVDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QUICK` writer - SMBus Quick Command"]
pub type QUICK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSEN` writer - TWIHS High-Speed Mode Enabled"]
pub type HSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSDIS` writer - TWIHS High-Speed Mode Disabled"]
pub type HSDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMBEN` writer - SMBus Mode Enabled"]
pub type SMBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMBDIS` writer - SMBus Mode Disabled"]
pub type SMBDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PECEN` writer - Packet Error Checking Enable"]
pub type PECEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PECDIS` writer - Packet Error Checking Disable"]
pub type PECDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PECRQ` writer - PEC Request"]
pub type PECRQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLEAR` writer - Bus CLEAR Command"]
pub type CLEAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACMEN` writer - Alternative Command Mode Enable"]
pub type ACMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACMDIS` writer - Alternative Command Mode Disable"]
pub type ACMDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THRCLR` writer - Transmit Holding Register Clear"]
pub type THRCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCKCLR` writer - Lock Clear"]
pub type LOCKCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOEN` writer - FIFO Enable"]
pub type FIFOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFODIS` writer - FIFO Disable"]
pub type FIFODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Send a START Condition"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CR_SPEC, 0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Send a STOP Condition"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR_SPEC, 1> {
        STOP_W::new(self)
    }
    #[doc = "Bit 2 - TWIHS Master Mode Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn msen(&mut self) -> MSEN_W<CR_SPEC, 2> {
        MSEN_W::new(self)
    }
    #[doc = "Bit 3 - TWIHS Master Mode Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn msdis(&mut self) -> MSDIS_W<CR_SPEC, 3> {
        MSDIS_W::new(self)
    }
    #[doc = "Bit 4 - TWIHS Slave Mode Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sven(&mut self) -> SVEN_W<CR_SPEC, 4> {
        SVEN_W::new(self)
    }
    #[doc = "Bit 5 - TWIHS Slave Mode Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn svdis(&mut self) -> SVDIS_W<CR_SPEC, 5> {
        SVDIS_W::new(self)
    }
    #[doc = "Bit 6 - SMBus Quick Command"]
    #[inline(always)]
    #[must_use]
    pub fn quick(&mut self) -> QUICK_W<CR_SPEC, 6> {
        QUICK_W::new(self)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CR_SPEC, 7> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 8 - TWIHS High-Speed Mode Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn hsen(&mut self) -> HSEN_W<CR_SPEC, 8> {
        HSEN_W::new(self)
    }
    #[doc = "Bit 9 - TWIHS High-Speed Mode Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn hsdis(&mut self) -> HSDIS_W<CR_SPEC, 9> {
        HSDIS_W::new(self)
    }
    #[doc = "Bit 10 - SMBus Mode Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn smben(&mut self) -> SMBEN_W<CR_SPEC, 10> {
        SMBEN_W::new(self)
    }
    #[doc = "Bit 11 - SMBus Mode Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn smbdis(&mut self) -> SMBDIS_W<CR_SPEC, 11> {
        SMBDIS_W::new(self)
    }
    #[doc = "Bit 12 - Packet Error Checking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<CR_SPEC, 12> {
        PECEN_W::new(self)
    }
    #[doc = "Bit 13 - Packet Error Checking Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pecdis(&mut self) -> PECDIS_W<CR_SPEC, 13> {
        PECDIS_W::new(self)
    }
    #[doc = "Bit 14 - PEC Request"]
    #[inline(always)]
    #[must_use]
    pub fn pecrq(&mut self) -> PECRQ_W<CR_SPEC, 14> {
        PECRQ_W::new(self)
    }
    #[doc = "Bit 15 - Bus CLEAR Command"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<CR_SPEC, 15> {
        CLEAR_W::new(self)
    }
    #[doc = "Bit 16 - Alternative Command Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmen(&mut self) -> ACMEN_W<CR_SPEC, 16> {
        ACMEN_W::new(self)
    }
    #[doc = "Bit 17 - Alternative Command Mode Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acmdis(&mut self) -> ACMDIS_W<CR_SPEC, 17> {
        ACMDIS_W::new(self)
    }
    #[doc = "Bit 24 - Transmit Holding Register Clear"]
    #[inline(always)]
    #[must_use]
    pub fn thrclr(&mut self) -> THRCLR_W<CR_SPEC, 24> {
        THRCLR_W::new(self)
    }
    #[doc = "Bit 26 - Lock Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lockclr(&mut self) -> LOCKCLR_W<CR_SPEC, 26> {
        LOCKCLR_W::new(self)
    }
    #[doc = "Bit 28 - FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FIFOEN_W<CR_SPEC, 28> {
        FIFOEN_W::new(self)
    }
    #[doc = "Bit 29 - FIFO Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fifodis(&mut self) -> FIFODIS_W<CR_SPEC, 29> {
        FIFODIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
