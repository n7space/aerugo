#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Field `TXRDY0` reader - Transmit Ready Interrupt Mask of channel 0"]
pub type TXRDY0_R = crate::BitReader;
#[doc = "Field `TXRDY1` reader - Transmit Ready Interrupt Mask of channel 1"]
pub type TXRDY1_R = crate::BitReader;
#[doc = "Field `EOC0` reader - End of Conversion Interrupt Mask of channel 0"]
pub type EOC0_R = crate::BitReader;
#[doc = "Field `EOC1` reader - End of Conversion Interrupt Mask of channel 1"]
pub type EOC1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Ready Interrupt Mask of channel 0"]
    #[inline(always)]
    pub fn txrdy0(&self) -> TXRDY0_R {
        TXRDY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Ready Interrupt Mask of channel 1"]
    #[inline(always)]
    pub fn txrdy1(&self) -> TXRDY1_R {
        TXRDY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Mask of channel 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Mask of channel 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
