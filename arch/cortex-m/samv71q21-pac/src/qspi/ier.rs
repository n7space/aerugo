#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDRF` writer - Receive Data Register Full Interrupt Enable"]
pub type RDRF_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `TDRE` writer - Transmit Data Register Empty Interrupt Enable"]
pub type TDRE_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `TXEMPTY` writer - Transmission Registers Empty Enable"]
pub type TXEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `OVRES` writer - Overrun Error Interrupt Enable"]
pub type OVRES_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `CSR` writer - Chip Select Rise Interrupt Enable"]
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `CSS` writer - Chip Select Status Interrupt Enable"]
pub type CSS_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `INSTRE` writer - Instruction End Interrupt Enable"]
pub type INSTRE_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdrf(&mut self) -> RDRF_W<0> {
        RDRF_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdre(&mut self) -> TDRE_W<1> {
        TDRE_W::new(self)
    }
    #[doc = "Bit 2 - Transmission Registers Empty Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<2> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovres(&mut self) -> OVRES_W<3> {
        OVRES_W::new(self)
    }
    #[doc = "Bit 8 - Chip Select Rise Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<8> {
        CSR_W::new(self)
    }
    #[doc = "Bit 9 - Chip Select Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CSS_W<9> {
        CSS_W::new(self)
    }
    #[doc = "Bit 10 - Instruction End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn instre(&mut self) -> INSTRE_W<10> {
        INSTRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
