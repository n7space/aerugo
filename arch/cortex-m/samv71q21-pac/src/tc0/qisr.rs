#[doc = "Register `QISR` reader"]
pub struct R(crate::R<QISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDX` reader - Index"]
pub type IDX_R = crate::BitReader;
#[doc = "Field `DIRCHG` reader - Direction Change"]
pub type DIRCHG_R = crate::BitReader;
#[doc = "Field `QERR` reader - Quadrature Error"]
pub type QERR_R = crate::BitReader;
#[doc = "Field `MPE` reader - Consecutive Missing Pulse Error"]
pub type MPE_R = crate::BitReader;
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader;
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
    #[doc = "Bit 8 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "QDEC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qisr](index.html) module"]
pub struct QISR_SPEC;
impl crate::RegisterSpec for QISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qisr::R](R) reader structure"]
impl crate::Readable for QISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QISR to value 0"]
impl crate::Resettable for QISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
