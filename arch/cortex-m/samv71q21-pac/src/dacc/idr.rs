#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
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
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXRDY0` writer - Transmit Ready Interrupt Disable of channel 0"]
pub type TXRDY0_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TXRDY1` writer - Transmit Ready Interrupt Disable of channel 1"]
pub type TXRDY1_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Disable of channel 0"]
pub type EOC0_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Disable of channel 1"]
pub type EOC1_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Transmit Ready Interrupt Disable of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy0(&mut self) -> TXRDY0_W<0> {
        TXRDY0_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Ready Interrupt Disable of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy1(&mut self) -> TXRDY1_W<1> {
        TXRDY1_W::new(self)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Disable of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn eoc0(&mut self) -> EOC0_W<4> {
        EOC0_W::new(self)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Disable of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn eoc1(&mut self) -> EOC1_W<5> {
        EOC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
