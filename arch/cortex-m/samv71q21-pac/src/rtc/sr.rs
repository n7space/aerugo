#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `ACKUPD` reader - Acknowledge for Update"]
pub type ACKUPD_R = crate::BitReader<ACKUPDSELECT_A>;
#[doc = "Acknowledge for Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKUPDSELECT_A {
    #[doc = "0: Time and calendar registers cannot be updated."]
    FREERUN = 0,
    #[doc = "1: Time and calendar registers can be updated."]
    UPDATE = 1,
}
impl From<ACKUPDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ACKUPDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKUPDSELECT_A {
        match self.bits {
            false => ACKUPDSELECT_A::FREERUN,
            true => ACKUPDSELECT_A::UPDATE,
        }
    }
    #[doc = "Time and calendar registers cannot be updated."]
    #[inline(always)]
    pub fn is_freerun(&self) -> bool {
        *self == ACKUPDSELECT_A::FREERUN
    }
    #[doc = "Time and calendar registers can be updated."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == ACKUPDSELECT_A::UPDATE
    }
}
#[doc = "Field `ALARM` reader - Alarm Flag"]
pub type ALARM_R = crate::BitReader<ALARMSELECT_A>;
#[doc = "Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARMSELECT_A {
    #[doc = "0: No alarm matching condition occurred."]
    NO_ALARMEVENT = 0,
    #[doc = "1: An alarm matching condition has occurred."]
    ALARMEVENT = 1,
}
impl From<ALARMSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ALARMSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARMSELECT_A {
        match self.bits {
            false => ALARMSELECT_A::NO_ALARMEVENT,
            true => ALARMSELECT_A::ALARMEVENT,
        }
    }
    #[doc = "No alarm matching condition occurred."]
    #[inline(always)]
    pub fn is_no_alarmevent(&self) -> bool {
        *self == ALARMSELECT_A::NO_ALARMEVENT
    }
    #[doc = "An alarm matching condition has occurred."]
    #[inline(always)]
    pub fn is_alarmevent(&self) -> bool {
        *self == ALARMSELECT_A::ALARMEVENT
    }
}
#[doc = "Field `SEC` reader - Second Event"]
pub type SEC_R = crate::BitReader<SECSELECT_A>;
#[doc = "Second Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSELECT_A {
    #[doc = "0: No second event has occurred since the last clear."]
    NO_SECEVENT = 0,
    #[doc = "1: At least one second event has occurred since the last clear."]
    SECEVENT = 1,
}
impl From<SECSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SECSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECSELECT_A {
        match self.bits {
            false => SECSELECT_A::NO_SECEVENT,
            true => SECSELECT_A::SECEVENT,
        }
    }
    #[doc = "No second event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_secevent(&self) -> bool {
        *self == SECSELECT_A::NO_SECEVENT
    }
    #[doc = "At least one second event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_secevent(&self) -> bool {
        *self == SECSELECT_A::SECEVENT
    }
}
#[doc = "Field `TIMEV` reader - Time Event"]
pub type TIMEV_R = crate::BitReader<TIMEVSELECT_A>;
#[doc = "Time Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEVSELECT_A {
    #[doc = "0: No time event has occurred since the last clear."]
    NO_TIMEVENT = 0,
    #[doc = "1: At least one time event has occurred since the last clear."]
    TIMEVENT = 1,
}
impl From<TIMEVSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEVSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEVSELECT_A {
        match self.bits {
            false => TIMEVSELECT_A::NO_TIMEVENT,
            true => TIMEVSELECT_A::TIMEVENT,
        }
    }
    #[doc = "No time event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_timevent(&self) -> bool {
        *self == TIMEVSELECT_A::NO_TIMEVENT
    }
    #[doc = "At least one time event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_timevent(&self) -> bool {
        *self == TIMEVSELECT_A::TIMEVENT
    }
}
#[doc = "Field `CALEV` reader - Calendar Event"]
pub type CALEV_R = crate::BitReader<CALEVSELECT_A>;
#[doc = "Calendar Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALEVSELECT_A {
    #[doc = "0: No calendar event has occurred since the last clear."]
    NO_CALEVENT = 0,
    #[doc = "1: At least one calendar event has occurred since the last clear."]
    CALEVENT = 1,
}
impl From<CALEVSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CALEVSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CALEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALEVSELECT_A {
        match self.bits {
            false => CALEVSELECT_A::NO_CALEVENT,
            true => CALEVSELECT_A::CALEVENT,
        }
    }
    #[doc = "No calendar event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_calevent(&self) -> bool {
        *self == CALEVSELECT_A::NO_CALEVENT
    }
    #[doc = "At least one calendar event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_calevent(&self) -> bool {
        *self == CALEVSELECT_A::CALEVENT
    }
}
#[doc = "Field `TDERR` reader - Time and/or Date Free Running Error"]
pub type TDERR_R = crate::BitReader<TDERRSELECT_A>;
#[doc = "Time and/or Date Free Running Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDERRSELECT_A {
    #[doc = "0: The internal free running counters are carrying valid values since the last read of the Status Register (RTC_SR)."]
    CORRECT = 0,
    #[doc = "1: The internal free running counters have been corrupted (invalid date or time, non-BCD values) since the last read and/or they are still invalid."]
    ERR_TIMEDATE = 1,
}
impl From<TDERRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TDERRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TDERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDERRSELECT_A {
        match self.bits {
            false => TDERRSELECT_A::CORRECT,
            true => TDERRSELECT_A::ERR_TIMEDATE,
        }
    }
    #[doc = "The internal free running counters are carrying valid values since the last read of the Status Register (RTC_SR)."]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == TDERRSELECT_A::CORRECT
    }
    #[doc = "The internal free running counters have been corrupted (invalid date or time, non-BCD values) since the last read and/or they are still invalid."]
    #[inline(always)]
    pub fn is_err_timedate(&self) -> bool {
        *self == TDERRSELECT_A::ERR_TIMEDATE
    }
}
impl R {
    #[doc = "Bit 0 - Acknowledge for Update"]
    #[inline(always)]
    pub fn ackupd(&self) -> ACKUPD_R {
        ACKUPD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alarm(&self) -> ALARM_R {
        ALARM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Second Event"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time Event"]
    #[inline(always)]
    pub fn timev(&self) -> TIMEV_R {
        TIMEV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Calendar Event"]
    #[inline(always)]
    pub fn calev(&self) -> CALEV_R {
        CALEV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time and/or Date Free Running Error"]
    #[inline(always)]
    pub fn tderr(&self) -> TDERR_R {
        TDERR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
