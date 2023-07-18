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
#[doc = "Field `WKUPS` reader - WKUP Wake-up Status (cleared on read)"]
pub type WKUPS_R = crate::BitReader<WKUPSSELECT_A>;
#[doc = "WKUP Wake-up Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPSSELECT_A {
    #[doc = "0: No wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<WKUPSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPSSELECT_A {
        match self.bits {
            false => WKUPSSELECT_A::NO,
            true => WKUPSSELECT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == WKUPSSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == WKUPSSELECT_A::PRESENT
    }
}
#[doc = "Field `SMWS` reader - Supply Monitor Detection Wake-up Status (cleared on read)"]
pub type SMWS_R = crate::BitReader<SMWSSELECT_A>;
#[doc = "Supply Monitor Detection Wake-up Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMWSSELECT_A {
    #[doc = "0: No wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<SMWSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SMWSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SMWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMWSSELECT_A {
        match self.bits {
            false => SMWSSELECT_A::NO,
            true => SMWSSELECT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SMWSSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == SMWSSELECT_A::PRESENT
    }
}
#[doc = "Field `BODRSTS` reader - Brownout Detector Reset Status (cleared on read)"]
pub type BODRSTS_R = crate::BitReader<BODRSTSSELECT_A>;
#[doc = "Brownout Detector Reset Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BODRSTSSELECT_A {
    #[doc = "0: No core brownout rising edge event has been detected since the last read of the SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one brownout output rising edge event has been detected since the last read of the SUPC_SR."]
    PRESENT = 1,
}
impl From<BODRSTSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BODRSTSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BODRSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTSSELECT_A {
        match self.bits {
            false => BODRSTSSELECT_A::NO,
            true => BODRSTSSELECT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == BODRSTSSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == BODRSTSSELECT_A::PRESENT
    }
}
#[doc = "Field `SMRSTS` reader - Supply Monitor Reset Status (cleared on read)"]
pub type SMRSTS_R = crate::BitReader<SMRSTSSELECT_A>;
#[doc = "Supply Monitor Reset Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMRSTSSELECT_A {
    #[doc = "0: No supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    PRESENT = 1,
}
impl From<SMRSTSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SMRSTSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SMRSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMRSTSSELECT_A {
        match self.bits {
            false => SMRSTSSELECT_A::NO,
            true => SMRSTSSELECT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SMRSTSSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == SMRSTSSELECT_A::PRESENT
    }
}
#[doc = "Field `SMS` reader - Supply Monitor Status (cleared on read)"]
pub type SMS_R = crate::BitReader<SMSSELECT_A>;
#[doc = "Supply Monitor Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMSSELECT_A {
    #[doc = "0: No supply monitor detection since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one supply monitor detection since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<SMSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SMSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMSSELECT_A {
        match self.bits {
            false => SMSSELECT_A::NO,
            true => SMSSELECT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SMSSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == SMSSELECT_A::PRESENT
    }
}
#[doc = "Field `SMOS` reader - Supply Monitor Output Status"]
pub type SMOS_R = crate::BitReader<SMOSSELECT_A>;
#[doc = "Supply Monitor Output Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMOSSELECT_A {
    #[doc = "0: The supply monitor detected VDDIO higher than its threshold at its last measurement."]
    HIGH = 0,
    #[doc = "1: The supply monitor detected VDDIO lower than its threshold at its last measurement."]
    LOW = 1,
}
impl From<SMOSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SMOSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SMOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMOSSELECT_A {
        match self.bits {
            false => SMOSSELECT_A::HIGH,
            true => SMOSSELECT_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SMOSSELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SMOSSELECT_A::LOW
    }
}
#[doc = "Field `OSCSEL` reader - 32-kHz Oscillator Selection Status"]
pub type OSCSEL_R = crate::BitReader<OSCSELSELECT_A>;
#[doc = "32-kHz Oscillator Selection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCSELSELECT_A {
    #[doc = "0: The slow clock, SLCK, is generated by the embedded 32 kHz RC oscillator."]
    RC = 0,
    #[doc = "1: The slow clock, SLCK, is generated by the 32 kHz crystal oscillator."]
    CRYST = 1,
}
impl From<OSCSELSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSELSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSELSELECT_A {
        match self.bits {
            false => OSCSELSELECT_A::RC,
            true => OSCSELSELECT_A::CRYST,
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == OSCSELSELECT_A::RC
    }
    #[doc = "Checks if the value of the field is `CRYST`"]
    #[inline(always)]
    pub fn is_cryst(&self) -> bool {
        *self == OSCSELSELECT_A::CRYST
    }
}
#[doc = "Field `LPDBCS0` reader - Low-power Debouncer Wake-up Status on WKUP0 (cleared on read)"]
pub type LPDBCS0_R = crate::BitReader<LPDBCS0SELECT_A>;
#[doc = "Low-power Debouncer Wake-up Status on WKUP0 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPDBCS0SELECT_A {
    #[doc = "0: No wake-up due to the assertion of the WKUP0 pin has occurred since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one wake-up due to the assertion of the WKUP0 pin has occurred since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<LPDBCS0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCS0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LPDBCS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCS0SELECT_A {
        match self.bits {
            false => LPDBCS0SELECT_A::NO,
            true => LPDBCS0SELECT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == LPDBCS0SELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == LPDBCS0SELECT_A::PRESENT
    }
}
#[doc = "Field `LPDBCS1` reader - Low-power Debouncer Wake-up Status on WKUP1 (cleared on read)"]
pub type LPDBCS1_R = crate::BitReader<LPDBCS1SELECT_A>;
#[doc = "Low-power Debouncer Wake-up Status on WKUP1 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPDBCS1SELECT_A {
    #[doc = "0: No wake-up due to the assertion of the WKUP1 pin has occurred since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one wake-up due to the assertion of the WKUP1 pin has occurred since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<LPDBCS1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCS1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LPDBCS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCS1SELECT_A {
        match self.bits {
            false => LPDBCS1SELECT_A::NO,
            true => LPDBCS1SELECT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == LPDBCS1SELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == LPDBCS1SELECT_A::PRESENT
    }
}
#[doc = "Field `WKUPIS0` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS0_R = crate::BitReader<WKUPIS0SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS0SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS0SELECT_A {
        match self.bits {
            false => WKUPIS0SELECT_A::DIS,
            true => WKUPIS0SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS0SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS0SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS1` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS1_R = crate::BitReader<WKUPIS1SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS1SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS1SELECT_A {
        match self.bits {
            false => WKUPIS1SELECT_A::DIS,
            true => WKUPIS1SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS1SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS1SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS2` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS2_R = crate::BitReader<WKUPIS2SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS2SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS2SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS2SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS2SELECT_A {
        match self.bits {
            false => WKUPIS2SELECT_A::DIS,
            true => WKUPIS2SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS2SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS2SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS3` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS3_R = crate::BitReader<WKUPIS3SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS3SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS3SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS3SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS3SELECT_A {
        match self.bits {
            false => WKUPIS3SELECT_A::DIS,
            true => WKUPIS3SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS3SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS3SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS4` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS4_R = crate::BitReader<WKUPIS4SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS4SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS4SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS4SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS4SELECT_A {
        match self.bits {
            false => WKUPIS4SELECT_A::DIS,
            true => WKUPIS4SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS4SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS4SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS5` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS5_R = crate::BitReader<WKUPIS5SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS5SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS5SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS5SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS5SELECT_A {
        match self.bits {
            false => WKUPIS5SELECT_A::DIS,
            true => WKUPIS5SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS5SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS5SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS6` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS6_R = crate::BitReader<WKUPIS6SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS6SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS6SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS6SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS6SELECT_A {
        match self.bits {
            false => WKUPIS6SELECT_A::DIS,
            true => WKUPIS6SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS6SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS6SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS7` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS7_R = crate::BitReader<WKUPIS7SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS7SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS7SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS7SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS7SELECT_A {
        match self.bits {
            false => WKUPIS7SELECT_A::DIS,
            true => WKUPIS7SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS7SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS7SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS8` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS8_R = crate::BitReader<WKUPIS8SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS8SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS8SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS8SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS8SELECT_A {
        match self.bits {
            false => WKUPIS8SELECT_A::DIS,
            true => WKUPIS8SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS8SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS8SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS9` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS9_R = crate::BitReader<WKUPIS9SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS9SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS9SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS9SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS9SELECT_A {
        match self.bits {
            false => WKUPIS9SELECT_A::DIS,
            true => WKUPIS9SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS9SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS9SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS10` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS10_R = crate::BitReader<WKUPIS10SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS10SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS10SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS10SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS10SELECT_A {
        match self.bits {
            false => WKUPIS10SELECT_A::DIS,
            true => WKUPIS10SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS10SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS10SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS11` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS11_R = crate::BitReader<WKUPIS11SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS11SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS11SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS11SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS11SELECT_A {
        match self.bits {
            false => WKUPIS11SELECT_A::DIS,
            true => WKUPIS11SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS11SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS11SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS12` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS12_R = crate::BitReader<WKUPIS12SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS12SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS12SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS12SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS12SELECT_A {
        match self.bits {
            false => WKUPIS12SELECT_A::DIS,
            true => WKUPIS12SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS12SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS12SELECT_A::EN
    }
}
#[doc = "Field `WKUPIS13` reader - WKUPx Input Status (cleared on read)"]
pub type WKUPIS13_R = crate::BitReader<WKUPIS13SELECT_A>;
#[doc = "WKUPx Input Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPIS13SELECT_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN = 1,
}
impl From<WKUPIS13SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS13SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPIS13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS13SELECT_A {
        match self.bits {
            false => WKUPIS13SELECT_A::DIS,
            true => WKUPIS13SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS13SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS13SELECT_A::EN
    }
}
impl R {
    #[doc = "Bit 1 - WKUP Wake-up Status (cleared on read)"]
    #[inline(always)]
    pub fn wkups(&self) -> WKUPS_R {
        WKUPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Supply Monitor Detection Wake-up Status (cleared on read)"]
    #[inline(always)]
    pub fn smws(&self) -> SMWS_R {
        SMWS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Brownout Detector Reset Status (cleared on read)"]
    #[inline(always)]
    pub fn bodrsts(&self) -> BODRSTS_R {
        BODRSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Supply Monitor Reset Status (cleared on read)"]
    #[inline(always)]
    pub fn smrsts(&self) -> SMRSTS_R {
        SMRSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Supply Monitor Status (cleared on read)"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Supply Monitor Output Status"]
    #[inline(always)]
    pub fn smos(&self) -> SMOS_R {
        SMOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 32-kHz Oscillator Selection Status"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Low-power Debouncer Wake-up Status on WKUP0 (cleared on read)"]
    #[inline(always)]
    pub fn lpdbcs0(&self) -> LPDBCS0_R {
        LPDBCS0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Low-power Debouncer Wake-up Status on WKUP1 (cleared on read)"]
    #[inline(always)]
    pub fn lpdbcs1(&self) -> LPDBCS1_R {
        LPDBCS1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis0(&self) -> WKUPIS0_R {
        WKUPIS0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis1(&self) -> WKUPIS1_R {
        WKUPIS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis2(&self) -> WKUPIS2_R {
        WKUPIS2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis3(&self) -> WKUPIS3_R {
        WKUPIS3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis4(&self) -> WKUPIS4_R {
        WKUPIS4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis5(&self) -> WKUPIS5_R {
        WKUPIS5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis6(&self) -> WKUPIS6_R {
        WKUPIS6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis7(&self) -> WKUPIS7_R {
        WKUPIS7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis8(&self) -> WKUPIS8_R {
        WKUPIS8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis9(&self) -> WKUPIS9_R {
        WKUPIS9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis10(&self) -> WKUPIS10_R {
        WKUPIS10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis11(&self) -> WKUPIS11_R {
        WKUPIS11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis12(&self) -> WKUPIS12_R {
        WKUPIS12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis13(&self) -> WKUPIS13_R {
        WKUPIS13_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Supply Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
