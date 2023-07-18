#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXEN` reader - Receiver Enabled"]
pub type RXEN_R = crate::BitReader;
#[doc = "Field `RXRDY` reader - Receive Ready"]
pub type RXRDY_R = crate::BitReader;
#[doc = "Field `RXOR` reader - Receive Overrun"]
pub type RXOR_R = crate::BitReader;
#[doc = "Field `TXEN` reader - Transmitter Enabled"]
pub type TXEN_R = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmit Ready"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `TXUR` reader - Transmit Underrun"]
pub type TXUR_R = crate::BitReader;
#[doc = "Field `RXORCH` reader - Receive Overrun Channel"]
pub type RXORCH_R = crate::FieldReader;
#[doc = "Field `TXURCH` reader - Transmit Underrun Channel"]
pub type TXURCH_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receiver Enabled"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    pub fn rxor(&self) -> RXOR_R {
        RXOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmitter Enabled"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn txur(&self) -> TXUR_R {
        TXUR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Receive Overrun Channel"]
    #[inline(always)]
    pub fn rxorch(&self) -> RXORCH_R {
        RXORCH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Transmit Underrun Channel"]
    #[inline(always)]
    pub fn txurch(&self) -> TXURCH_R {
        TXURCH_R::new(((self.bits >> 20) & 3) as u8)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
