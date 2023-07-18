#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDRF` reader - Receive Data Register Full Interrupt Mask"]
pub type RDRF_R = crate::BitReader;
#[doc = "Field `TDRE` reader - Transmit Data Register Empty Interrupt Mask"]
pub type TDRE_R = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty Mask"]
pub type TXEMPTY_R = crate::BitReader;
#[doc = "Field `OVRES` reader - Overrun Error Interrupt Mask"]
pub type OVRES_R = crate::BitReader;
#[doc = "Field `CSR` reader - Chip Select Rise Interrupt Mask"]
pub type CSR_R = crate::BitReader;
#[doc = "Field `CSS` reader - Chip Select Status Interrupt Mask"]
pub type CSS_R = crate::BitReader;
#[doc = "Field `INSTRE` reader - Instruction End Interrupt Mask"]
pub type INSTRE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Mask"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Mask"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Registers Empty Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Chip Select Rise Interrupt Mask"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Chip Select Status Interrupt Mask"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Instruction End Interrupt Mask"]
    #[inline(always)]
    pub fn instre(&self) -> INSTRE_R {
        INSTRE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
