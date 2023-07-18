#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RXDIS_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `CKEN` writer - Clocks Enable"]
pub type CKEN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `CKDIS` writer - Clocks Disable"]
pub type CKDIS_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TXDIS_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<0> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RXDIS_W<1> {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 2 - Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cken(&mut self) -> CKEN_W<2> {
        CKEN_W::new(self)
    }
    #[doc = "Bit 3 - Clocks Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ckdis(&mut self) -> CKDIS_W<3> {
        CKDIS_W::new(self)
    }
    #[doc = "Bit 4 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<4> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 5 - Transmitter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TXDIS_W<5> {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<7> {
        SWRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
