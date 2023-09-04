#[doc = "Register `QIMR` reader"]
pub type R = crate::R<QIMR_SPEC>;
#[doc = "Field `IDX` reader - Index"]
pub type IDX_R = crate::BitReader;
#[doc = "Field `DIRCHG` reader - Direction Change"]
pub type DIRCHG_R = crate::BitReader;
#[doc = "Field `QERR` reader - Quadrature Error"]
pub type QERR_R = crate::BitReader;
#[doc = "Field `MPE` reader - Consecutive Missing Pulse Error"]
pub type MPE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Index"]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction Change"]
    #[inline(always)]
    pub fn dirchg(&self) -> DIRCHG_R {
        DIRCHG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Quadrature Error"]
    #[inline(always)]
    pub fn qerr(&self) -> QERR_R {
        QERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Consecutive Missing Pulse Error"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "QDEC Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qimr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QIMR_SPEC;
impl crate::RegisterSpec for QIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qimr::R`](R) reader structure"]
impl crate::Readable for QIMR_SPEC {}
#[doc = "`reset()` method sets QIMR to value 0"]
impl crate::Resettable for QIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
