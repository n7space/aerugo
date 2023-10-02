#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `TXRDY0` writer - Transmit Ready Interrupt Disable of channel 0"]
pub type TXRDY0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXRDY1` writer - Transmit Ready Interrupt Disable of channel 1"]
pub type TXRDY1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Disable of channel 0"]
pub type EOC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Disable of channel 1"]
pub type EOC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Transmit Ready Interrupt Disable of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy0(&mut self) -> TXRDY0_W<IDR_SPEC, 0> {
        TXRDY0_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Ready Interrupt Disable of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy1(&mut self) -> TXRDY1_W<IDR_SPEC, 1> {
        TXRDY1_W::new(self)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Disable of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn eoc0(&mut self) -> EOC0_W<IDR_SPEC, 4> {
        EOC0_W::new(self)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Disable of channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn eoc1(&mut self) -> EOC1_W<IDR_SPEC, 5> {
        EOC1_W::new(self)
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
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
