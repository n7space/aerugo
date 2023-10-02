#[doc = "Register `DEVEPTIER_BLK_MODE[%s]` writer"]
pub type W = crate::W<DEVEPTIER_BLK_MODE_SPEC>;
#[doc = "Field `TXINES` writer - Transmitted IN Data Interrupt Enable"]
pub type TXINES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOUTES` writer - Received OUT Data Interrupt Enable"]
pub type RXOUTES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSTPES` writer - Received SETUP Interrupt Enable"]
pub type RXSTPES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKOUTES` writer - NAKed OUT Interrupt Enable"]
pub type NAKOUTES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKINES` writer - NAKed IN Interrupt Enable"]
pub type NAKINES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVERFES` writer - Overflow Interrupt Enable"]
pub type OVERFES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALLEDES` writer - STALLed Interrupt Enable"]
pub type STALLEDES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHORTPACKETES` writer - Short Packet Interrupt Enable"]
pub type SHORTPACKETES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Interrupt Enable"]
pub type NBUSYBKES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `KILLBKS` writer - Kill IN Bank"]
pub type KILLBKS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFOCONS` writer - FIFO Control"]
pub type FIFOCONS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDISHDMAS` writer - Endpoint Interrupts Disable HDMA Request Enable"]
pub type EPDISHDMAS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NYETDISS` writer - NYET Token Disable Enable"]
pub type NYETDISS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RSTDTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALLRQS` writer - STALL Request Enable"]
pub type STALLRQS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txines(&mut self) -> TXINES_W<DEVEPTIER_BLK_MODE_SPEC, 0> {
        TXINES_W::new(self)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutes(&mut self) -> RXOUTES_W<DEVEPTIER_BLK_MODE_SPEC, 1> {
        RXOUTES_W::new(self)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpes(&mut self) -> RXSTPES_W<DEVEPTIER_BLK_MODE_SPEC, 2> {
        RXSTPES_W::new(self)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutes(&mut self) -> NAKOUTES_W<DEVEPTIER_BLK_MODE_SPEC, 3> {
        NAKOUTES_W::new(self)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakines(&mut self) -> NAKINES_W<DEVEPTIER_BLK_MODE_SPEC, 4> {
        NAKINES_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn overfes(&mut self) -> OVERFES_W<DEVEPTIER_BLK_MODE_SPEC, 5> {
        OVERFES_W::new(self)
    }
    #[doc = "Bit 6 - STALLed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stalledes(&mut self) -> STALLEDES_W<DEVEPTIER_BLK_MODE_SPEC, 6> {
        STALLEDES_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketes(&mut self) -> SHORTPACKETES_W<DEVEPTIER_BLK_MODE_SPEC, 7> {
        SHORTPACKETES_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W<DEVEPTIER_BLK_MODE_SPEC, 12> {
        NBUSYBKES_W::new(self)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    #[must_use]
    pub fn killbks(&mut self) -> KILLBKS_W<DEVEPTIER_BLK_MODE_SPEC, 13> {
        KILLBKS_W::new(self)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    #[must_use]
    pub fn fifocons(&mut self) -> FIFOCONS_W<DEVEPTIER_BLK_MODE_SPEC, 14> {
        FIFOCONS_W::new(self)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epdishdmas(&mut self) -> EPDISHDMAS_W<DEVEPTIER_BLK_MODE_SPEC, 16> {
        EPDISHDMAS_W::new(self)
    }
    #[doc = "Bit 17 - NYET Token Disable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nyetdiss(&mut self) -> NYETDISS_W<DEVEPTIER_BLK_MODE_SPEC, 17> {
        NYETDISS_W::new(self)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstdts(&mut self) -> RSTDTS_W<DEVEPTIER_BLK_MODE_SPEC, 18> {
        RSTDTS_W::new(self)
    }
    #[doc = "Bit 19 - STALL Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqs(&mut self) -> STALLRQS_W<DEVEPTIER_BLK_MODE_SPEC, 19> {
        STALLRQS_W::new(self)
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
#[doc = "Device Endpoint Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier_blk_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVEPTIER_BLK_MODE_SPEC;
impl crate::RegisterSpec for DEVEPTIER_BLK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`deveptier_blk_mode::W`](W) writer structure"]
impl crate::Writable for DEVEPTIER_BLK_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVEPTIER_BLK_MODE[%s]
to value 0"]
impl crate::Resettable for DEVEPTIER_BLK_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
