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
#[doc = "Field `RDERRI` reader - Remote Device Connection Error Interrupt (Host mode only)"]
pub type RDERRI_R = crate::BitReader;
#[doc = "Field `SPEED` reader - Speed Status (Device mode only)"]
pub type SPEED_R = crate::FieldReader<SPEEDSELECT_A>;
#[doc = "Speed Status (Device mode only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPEEDSELECT_A {
    #[doc = "0: Full-Speed mode"]
    FULL_SPEED = 0,
    #[doc = "1: High-Speed mode"]
    HIGH_SPEED = 1,
    #[doc = "2: Low-Speed mode"]
    LOW_SPEED = 2,
}
impl From<SPEEDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEEDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPEEDSELECT_A {
    type Ux = u8;
}
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPEEDSELECT_A> {
        match self.bits {
            0 => Some(SPEEDSELECT_A::FULL_SPEED),
            1 => Some(SPEEDSELECT_A::HIGH_SPEED),
            2 => Some(SPEEDSELECT_A::LOW_SPEED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FULL_SPEED`"]
    #[inline(always)]
    pub fn is_full_speed(&self) -> bool {
        *self == SPEEDSELECT_A::FULL_SPEED
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == SPEEDSELECT_A::HIGH_SPEED
    }
    #[doc = "Checks if the value of the field is `LOW_SPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == SPEEDSELECT_A::LOW_SPEED
    }
}
#[doc = "Field `CLKUSABLE` reader - UTMI Clock Usable"]
pub type CLKUSABLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt (Host mode only)"]
    #[inline(always)]
    pub fn rderri(&self) -> RDERRI_R {
        RDERRI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Speed Status (Device mode only)"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - UTMI Clock Usable"]
    #[inline(always)]
    pub fn clkusable(&self) -> CLKUSABLE_R {
        CLKUSABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "General Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
