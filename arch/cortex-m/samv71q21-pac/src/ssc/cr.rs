#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `RXEN` writer - Receive Enable"]
pub type RXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXDIS` writer - Receive Disable"]
pub type RXDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXEN` writer - Transmit Enable"]
pub type TXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDIS` writer - Transmit Disable"]
pub type TXDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<CR_SPEC, 0> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 1 - Receive Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RXDIS_W<CR_SPEC, 1> {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<CR_SPEC, 8> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 9 - Transmit Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TXDIS_W<CR_SPEC, 9> {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 15 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CR_SPEC, 15> {
        SWRST_W::new(self)
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
