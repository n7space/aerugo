#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `CE` reader - Comparison Edge (cleared on read)"]
pub type CE_R = crate::BitReader;
#[doc = "Field `SCO` reader - Synchronized Comparator Output"]
pub type SCO_R = crate::BitReader;
#[doc = "Field `MASK` reader - Flag Mask"]
pub type MASK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Comparison Edge (cleared on read)"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronized Comparator Output"]
    #[inline(always)]
    pub fn sco(&self) -> SCO_R {
        SCO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - Flag Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
