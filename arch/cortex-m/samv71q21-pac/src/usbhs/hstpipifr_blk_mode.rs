#[doc = "Register `HSTPIPIFR_BLK_MODE[%s]` writer"]
pub struct W(crate::W<HSTPIPIFR_BLK_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTPIPIFR_BLK_MODE_SPEC>;
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
impl From<crate::W<HSTPIPIFR_BLK_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTPIPIFR_BLK_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINIS` writer - Received IN Data Interrupt Set"]
pub type RXINIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIFR_BLK_MODE_SPEC, O>;
#[doc = "Field `TXOUTIS` writer - Transmitted OUT Data Interrupt Set"]
pub type TXOUTIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIFR_BLK_MODE_SPEC, O>;
#[doc = "Field `TXSTPIS` writer - Transmitted SETUP Interrupt Set"]
pub type TXSTPIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIFR_BLK_MODE_SPEC, O>;
#[doc = "Field `PERRIS` writer - Pipe Error Interrupt Set"]
pub type PERRIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIFR_BLK_MODE_SPEC, O>;
#[doc = "Field `NAKEDIS` writer - NAKed Interrupt Set"]
pub type NAKEDIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIFR_BLK_MODE_SPEC, O>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OVERFIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIFR_BLK_MODE_SPEC, O>;
#[doc = "Field `RXSTALLDIS` writer - Received STALLed Interrupt Set"]
pub type RXSTALLDIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIFR_BLK_MODE_SPEC, O>;
#[doc = "Field `SHORTPACKETIS` writer - Short Packet Interrupt Set"]
pub type SHORTPACKETIS_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIFR_BLK_MODE_SPEC, O>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Set"]
pub type NBUSYBKS_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPIFR_BLK_MODE_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxinis(&mut self) -> RXINIS_W<0> {
        RXINIS_W::new(self)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txoutis(&mut self) -> TXOUTIS_W<1> {
        TXOUTIS_W::new(self)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn txstpis(&mut self) -> TXSTPIS_W<2> {
        TXSTPIS_W::new(self)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn perris(&mut self) -> PERRIS_W<3> {
        PERRIS_W::new(self)
    }
    #[doc = "Bit 4 - NAKed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn nakedis(&mut self) -> NAKEDIS_W<4> {
        NAKEDIS_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn overfis(&mut self) -> OVERFIS_W<5> {
        OVERFIS_W::new(self)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldis(&mut self) -> RXSTALLDIS_W<6> {
        RXSTALLDIS_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketis(&mut self) -> SHORTPACKETIS_W<7> {
        SHORTPACKETIS_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Set"]
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
#[doc = "Host Pipe Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipifr_blk_mode](index.html) module"]
pub struct HSTPIPIFR_BLK_MODE_SPEC;
impl crate::RegisterSpec for HSTPIPIFR_BLK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hstpipifr_blk_mode::W](W) writer structure"]
impl crate::Writable for HSTPIPIFR_BLK_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTPIPIFR_BLK_MODE[%s]
to value 0"]
impl crate::Resettable for HSTPIPIFR_BLK_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
