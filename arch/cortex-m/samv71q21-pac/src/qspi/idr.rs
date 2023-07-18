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
#[doc = "Field `RDRF` writer - Receive Data Register Full Interrupt Disable"]
pub type RDRF_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TDRE` writer - Transmit Data Register Empty Interrupt Disable"]
pub type TDRE_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TXEMPTY` writer - Transmission Registers Empty Disable"]
pub type TXEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `OVRES` writer - Overrun Error Interrupt Disable"]
pub type OVRES_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `CSR` writer - Chip Select Rise Interrupt Disable"]
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `CSS` writer - Chip Select Status Interrupt Disable"]
pub type CSS_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `INSTRE` writer - Instruction End Interrupt Disable"]
pub type INSTRE_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rdrf(&mut self) -> RDRF_W<0> {
        RDRF_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tdre(&mut self) -> TDRE_W<1> {
        TDRE_W::new(self)
    }
    #[doc = "Bit 2 - Transmission Registers Empty Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<2> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovres(&mut self) -> OVRES_W<3> {
        OVRES_W::new(self)
    }
    #[doc = "Bit 8 - Chip Select Rise Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<8> {
        CSR_W::new(self)
    }
    #[doc = "Bit 9 - Chip Select Status Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CSS_W<9> {
        CSS_W::new(self)
    }
    #[doc = "Bit 10 - Instruction End Interrupt Disable"]
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
