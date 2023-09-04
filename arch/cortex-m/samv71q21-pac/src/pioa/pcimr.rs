#[doc = "Register `PCIMR` reader"]
pub type R = crate::R<PCIMR_SPEC>;
#[doc = "Field `DRDY` reader - Parallel Capture Mode Data Ready Interrupt Mask"]
pub type DRDY_R = crate::BitReader;
#[doc = "Field `OVRE` reader - Parallel Capture Mode Overrun Error Interrupt Mask"]
pub type OVRE_R = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of Reception Transfer Interrupt Mask"]
pub type ENDRX_R = crate::BitReader;
#[doc = "Field `RXBUFF` reader - Reception Buffer Full Interrupt Mask"]
pub type RXBUFF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Reception Transfer Interrupt Mask"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reception Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Parallel Capture Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcimr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCIMR_SPEC;
impl crate::RegisterSpec for PCIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcimr::R`](R) reader structure"]
impl crate::Readable for PCIMR_SPEC {}
#[doc = "`reset()` method sets PCIMR to value 0"]
impl crate::Resettable for PCIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
