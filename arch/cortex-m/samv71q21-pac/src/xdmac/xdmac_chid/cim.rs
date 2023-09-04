#[doc = "Register `CIM` reader"]
pub type R = crate::R<CIM_SPEC>;
#[doc = "Field `BIM` reader - End of Block Interrupt Mask Bit"]
pub type BIM_R = crate::BitReader;
#[doc = "Field `LIM` reader - End of Linked List Interrupt Mask Bit"]
pub type LIM_R = crate::BitReader;
#[doc = "Field `DIM` reader - End of Disable Interrupt Mask Bit"]
pub type DIM_R = crate::BitReader;
#[doc = "Field `FIM` reader - End of Flush Interrupt Mask Bit"]
pub type FIM_R = crate::BitReader;
#[doc = "Field `RBEIM` reader - Read Bus Error Interrupt Mask Bit"]
pub type RBEIM_R = crate::BitReader;
#[doc = "Field `WBEIM` reader - Write Bus Error Interrupt Mask Bit"]
pub type WBEIM_R = crate::BitReader;
#[doc = "Field `ROIM` reader - Request Overflow Error Interrupt Mask Bit"]
pub type ROIM_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of Block Interrupt Mask Bit"]
    #[inline(always)]
    pub fn bim(&self) -> BIM_R {
        BIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Mask Bit"]
    #[inline(always)]
    pub fn lim(&self) -> LIM_R {
        LIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Mask Bit"]
    #[inline(always)]
    pub fn dim(&self) -> DIM_R {
        DIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Mask Bit"]
    #[inline(always)]
    pub fn fim(&self) -> FIM_R {
        FIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn rbeim(&self) -> RBEIM_R {
        RBEIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn wbeim(&self) -> WBEIM_R {
        WBEIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn roim(&self) -> ROIM_R {
        ROIM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Channel Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cim::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIM_SPEC;
impl crate::RegisterSpec for CIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cim::R`](R) reader structure"]
impl crate::Readable for CIM_SPEC {}
#[doc = "`reset()` method sets CIM to value 0"]
impl crate::Resettable for CIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
