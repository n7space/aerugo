#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Field `RXRDY` reader - Mask RXRDY Interrupt"]
pub type RXRDY_R = crate::BitReader;
#[doc = "Field `TXRDY` reader - Disable TXRDY Interrupt"]
pub type TXRDY_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Mask Overrun Error Interrupt"]
pub type OVRE_R = crate::BitReader;
#[doc = "Field `FRAME` reader - Mask Framing Error Interrupt"]
pub type FRAME_R = crate::BitReader;
#[doc = "Field `PARE` reader - Mask Parity Error Interrupt"]
pub type PARE_R = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Mask TXEMPTY Interrupt"]
pub type TXEMPTY_R = crate::BitReader;
#[doc = "Field `CMP` reader - Mask Comparison Interrupt"]
pub type CMP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Mask RXRDY Interrupt"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable TXRDY Interrupt"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask Overrun Error Interrupt"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask Framing Error Interrupt"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask Parity Error Interrupt"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask TXEMPTY Interrupt"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask Comparison Interrupt"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 15) & 1) != 0)
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
