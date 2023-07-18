#[doc = "Register `HSTPIPICR_BLK_MODE[%s]` writer"]
pub struct W(crate::W<HSTPIPICR_BLK_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTPIPICR_BLK_MODE_SPEC>;
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
impl From<crate::W<HSTPIPICR_BLK_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTPIPICR_BLK_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINIC` writer - Received IN Data Interrupt Clear"]
pub type RXINIC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPICR_BLK_MODE_SPEC, O>;
#[doc = "Field `TXOUTIC` writer - Transmitted OUT Data Interrupt Clear"]
pub type TXOUTIC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPICR_BLK_MODE_SPEC, O>;
#[doc = "Field `TXSTPIC` writer - Transmitted SETUP Interrupt Clear"]
pub type TXSTPIC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPICR_BLK_MODE_SPEC, O>;
#[doc = "Field `NAKEDIC` writer - NAKed Interrupt Clear"]
pub type NAKEDIC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPICR_BLK_MODE_SPEC, O>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OVERFIC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPICR_BLK_MODE_SPEC, O>;
#[doc = "Field `RXSTALLDIC` writer - Received STALLed Interrupt Clear"]
pub type RXSTALLDIC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPICR_BLK_MODE_SPEC, O>;
#[doc = "Field `SHORTPACKETIC` writer - Short Packet Interrupt Clear"]
pub type SHORTPACKETIC_W<'a, const O: u8> = crate::BitWriter<'a, HSTPIPICR_BLK_MODE_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxinic(&mut self) -> RXINIC_W<0> {
        RXINIC_W::new(self)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txoutic(&mut self) -> TXOUTIC_W<1> {
        TXOUTIC_W::new(self)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txstpic(&mut self) -> TXSTPIC_W<2> {
        TXSTPIC_W::new(self)
    }
    #[doc = "Bit 4 - NAKed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakedic(&mut self) -> NAKEDIC_W<4> {
        NAKEDIC_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfic(&mut self) -> OVERFIC_W<5> {
        OVERFIC_W::new(self)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstalldic(&mut self) -> RXSTALLDIC_W<6> {
        RXSTALLDIC_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketic(&mut self) -> SHORTPACKETIC_W<7> {
        SHORTPACKETIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipicr_blk_mode](index.html) module"]
pub struct HSTPIPICR_BLK_MODE_SPEC;
impl crate::RegisterSpec for HSTPIPICR_BLK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hstpipicr_blk_mode::W](W) writer structure"]
impl crate::Writable for HSTPIPICR_BLK_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTPIPICR_BLK_MODE[%s]
to value 0"]
impl crate::Resettable for HSTPIPICR_BLK_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
