#[doc = "Register `PCIDR` writer"]
pub type W = crate::W<PCIDR_SPEC>;
#[doc = "Field `DRDY` writer - Parallel Capture Mode Data Ready Interrupt Disable"]
pub type DRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVRE` writer - Parallel Capture Mode Overrun Error Interrupt Disable"]
pub type OVRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENDRX` writer - End of Reception Transfer Interrupt Disable"]
pub type ENDRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBUFF` writer - Reception Buffer Full Interrupt Disable"]
pub type RXBUFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DRDY_W<PCIDR_SPEC, 0> {
        DRDY_W::new(self)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<PCIDR_SPEC, 1> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 2 - End of Reception Transfer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<PCIDR_SPEC, 2> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 3 - Reception Buffer Full Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<PCIDR_SPEC, 3> {
        RXBUFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Parallel Capture Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCIDR_SPEC;
impl crate::RegisterSpec for PCIDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pcidr::W`](W) writer structure"]
impl crate::Writable for PCIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCIDR to value 0"]
impl crate::Resettable for PCIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
