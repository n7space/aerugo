#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXRDY0` reader - Transmit Ready Interrupt Flag of channel 0"]
pub type TXRDY0_R = crate::BitReader;
#[doc = "Field `TXRDY1` reader - Transmit Ready Interrupt Flag of channel 1"]
pub type TXRDY1_R = crate::BitReader;
#[doc = "Field `EOC0` reader - End of Conversion Interrupt Flag of channel 0"]
pub type EOC0_R = crate::BitReader;
#[doc = "Field `EOC1` reader - End of Conversion Interrupt Flag of channel 1"]
pub type EOC1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Ready Interrupt Flag of channel 0"]
    #[inline(always)]
    pub fn txrdy0(&self) -> TXRDY0_R {
        TXRDY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Ready Interrupt Flag of channel 1"]
    #[inline(always)]
    pub fn txrdy1(&self) -> TXRDY1_R {
        TXRDY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Flag of channel 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Flag of channel 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
