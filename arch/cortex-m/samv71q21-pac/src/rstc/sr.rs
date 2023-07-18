#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `URSTS` reader - User Reset Status"]
pub type URSTS_R = crate::BitReader;
#[doc = "Field `RSTTYP` reader - Reset Type"]
pub type RSTTYP_R = crate::FieldReader<RSTTYPSELECT_A>;
#[doc = "Reset Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSTTYPSELECT_A {
    #[doc = "0: First power-up reset"]
    GENERAL_RST = 0,
    #[doc = "1: Return from Backup Mode"]
    BACKUP_RST = 1,
    #[doc = "2: Watchdog fault occurred"]
    WDT_RST = 2,
    #[doc = "3: Processor reset required by the software"]
    SOFT_RST = 3,
    #[doc = "4: NRST pin detected low"]
    USER_RST = 4,
}
impl From<RSTTYPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTTYPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSTTYPSELECT_A {
    type Ux = u8;
}
impl RSTTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSTTYPSELECT_A> {
        match self.bits {
            0 => Some(RSTTYPSELECT_A::GENERAL_RST),
            1 => Some(RSTTYPSELECT_A::BACKUP_RST),
            2 => Some(RSTTYPSELECT_A::WDT_RST),
            3 => Some(RSTTYPSELECT_A::SOFT_RST),
            4 => Some(RSTTYPSELECT_A::USER_RST),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GENERAL_RST`"]
    #[inline(always)]
    pub fn is_general_rst(&self) -> bool {
        *self == RSTTYPSELECT_A::GENERAL_RST
    }
    #[doc = "Checks if the value of the field is `BACKUP_RST`"]
    #[inline(always)]
    pub fn is_backup_rst(&self) -> bool {
        *self == RSTTYPSELECT_A::BACKUP_RST
    }
    #[doc = "Checks if the value of the field is `WDT_RST`"]
    #[inline(always)]
    pub fn is_wdt_rst(&self) -> bool {
        *self == RSTTYPSELECT_A::WDT_RST
    }
    #[doc = "Checks if the value of the field is `SOFT_RST`"]
    #[inline(always)]
    pub fn is_soft_rst(&self) -> bool {
        *self == RSTTYPSELECT_A::SOFT_RST
    }
    #[doc = "Checks if the value of the field is `USER_RST`"]
    #[inline(always)]
    pub fn is_user_rst(&self) -> bool {
        *self == RSTTYPSELECT_A::USER_RST
    }
}
#[doc = "Field `NRSTL` reader - NRST Pin Level"]
pub type NRSTL_R = crate::BitReader;
#[doc = "Field `SRCMP` reader - Software Reset Command in Progress"]
pub type SRCMP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - User Reset Status"]
    #[inline(always)]
    pub fn ursts(&self) -> URSTS_R {
        URSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Reset Type"]
    #[inline(always)]
    pub fn rsttyp(&self) -> RSTTYP_R {
        RSTTYP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - NRST Pin Level"]
    #[inline(always)]
    pub fn nrstl(&self) -> NRSTL_R {
        NRSTL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software Reset Command in Progress"]
    #[inline(always)]
    pub fn srcmp(&self) -> SRCMP_R {
        SRCMP_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
