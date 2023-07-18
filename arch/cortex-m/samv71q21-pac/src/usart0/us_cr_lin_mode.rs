#[doc = "Register `US_CR_LIN_MODE` writer"]
pub struct W(crate::W<US_CR_LIN_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_CR_LIN_MODE_SPEC>;
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
impl From<crate::W<US_CR_LIN_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_CR_LIN_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub type RSTRX_W<'a, const O: u8> = crate::BitWriter<'a, US_CR_LIN_MODE_SPEC, O>;
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub type RSTTX_W<'a, const O: u8> = crate::BitWriter<'a, US_CR_LIN_MODE_SPEC, O>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, US_CR_LIN_MODE_SPEC, O>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RXDIS_W<'a, const O: u8> = crate::BitWriter<'a, US_CR_LIN_MODE_SPEC, O>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, US_CR_LIN_MODE_SPEC, O>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TXDIS_W<'a, const O: u8> = crate::BitWriter<'a, US_CR_LIN_MODE_SPEC, O>;
#[doc = "Field `RSTSTA` writer - Reset Status Bits"]
pub type RSTSTA_W<'a, const O: u8> = crate::BitWriter<'a, US_CR_LIN_MODE_SPEC, O>;
#[doc = "Field `LINABT` writer - Abort LIN Transmission"]
pub type LINABT_W<'a, const O: u8> = crate::BitWriter<'a, US_CR_LIN_MODE_SPEC, O>;
#[doc = "Field `LINWKUP` writer - Send LIN Wakeup Signal"]
pub type LINWKUP_W<'a, const O: u8> = crate::BitWriter<'a, US_CR_LIN_MODE_SPEC, O>;
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rstrx(&mut self) -> RSTRX_W<2> {
        RSTRX_W::new(self)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn rsttx(&mut self) -> RSTTX_W<3> {
        RSTTX_W::new(self)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<4> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RXDIS_W<5> {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<6> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TXDIS_W<7> {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rststa(&mut self) -> RSTSTA_W<8> {
        RSTSTA_W::new(self)
    }
    #[doc = "Bit 20 - Abort LIN Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn linabt(&mut self) -> LINABT_W<20> {
        LINABT_W::new(self)
    }
    #[doc = "Bit 21 - Send LIN Wakeup Signal"]
    #[inline(always)]
    #[must_use]
    pub fn linwkup(&mut self) -> LINWKUP_W<21> {
        LINWKUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_cr_lin_mode](index.html) module"]
pub struct US_CR_LIN_MODE_SPEC;
impl crate::RegisterSpec for US_CR_LIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [us_cr_lin_mode::W](W) writer structure"]
impl crate::Writable for US_CR_LIN_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_CR_LIN_MODE to value 0"]
impl crate::Resettable for US_CR_LIN_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
