#[doc = "Register `EEFC_FSR` reader"]
pub type R = crate::R<EEFC_FSR_SPEC>;
#[doc = "Field `FRDY` reader - Flash Ready Status (cleared when Flash is busy)"]
pub type FRDY_R = crate::BitReader;
#[doc = "Field `FCMDE` reader - Flash Command Error Status (cleared on read or by writing EEFC_FCR)"]
pub type FCMDE_R = crate::BitReader;
#[doc = "Field `FLOCKE` reader - Flash Lock Error Status (cleared on read)"]
pub type FLOCKE_R = crate::BitReader;
#[doc = "Field `FLERR` reader - Flash Error Status (cleared when a programming operation starts)"]
pub type FLERR_R = crate::BitReader;
#[doc = "Field `UECCELSB` reader - Unique ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
pub type UECCELSB_R = crate::BitReader;
#[doc = "Field `MECCELSB` reader - Multiple ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
pub type MECCELSB_R = crate::BitReader;
#[doc = "Field `UECCEMSB` reader - Unique ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
pub type UECCEMSB_R = crate::BitReader;
#[doc = "Field `MECCEMSB` reader - Multiple ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
pub type MECCEMSB_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Flash Ready Status (cleared when Flash is busy)"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Command Error Status (cleared on read or by writing EEFC_FCR)"]
    #[inline(always)]
    pub fn fcmde(&self) -> FCMDE_R {
        FCMDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Lock Error Status (cleared on read)"]
    #[inline(always)]
    pub fn flocke(&self) -> FLOCKE_R {
        FLOCKE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash Error Status (cleared when a programming operation starts)"]
    #[inline(always)]
    pub fn flerr(&self) -> FLERR_R {
        FLERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Unique ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn ueccelsb(&self) -> UECCELSB_R {
        UECCELSB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Multiple ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn meccelsb(&self) -> MECCELSB_R {
        MECCELSB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Unique ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn ueccemsb(&self) -> UECCEMSB_R {
        UECCEMSB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Multiple ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn meccemsb(&self) -> MECCEMSB_R {
        MECCEMSB_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "EEFC Flash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefc_fsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEFC_FSR_SPEC;
impl crate::RegisterSpec for EEFC_FSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eefc_fsr::R`](R) reader structure"]
impl crate::Readable for EEFC_FSR_SPEC {}
#[doc = "`reset()` method sets EEFC_FSR to value 0"]
impl crate::Resettable for EEFC_FSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
