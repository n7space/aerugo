#[doc = "Register `WUMR` reader"]
pub struct R(crate::R<WUMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUMR` writer"]
pub struct W(crate::W<WUMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WUMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMEN` reader - Supply Monitor Wake-up Enable"]
pub type SMEN_R = crate::BitReader<SMENSELECT_A>;
#[doc = "Supply Monitor Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMENSELECT_A {
    #[doc = "0: The supply monitor detection has no wake-up effect."]
    NOT_ENABLE = 0,
    #[doc = "1: The supply monitor detection forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<SMENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SMENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMENSELECT_A {
        match self.bits {
            false => SMENSELECT_A::NOT_ENABLE,
            true => SMENSELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMENSELECT_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMENSELECT_A::ENABLE
    }
}
#[doc = "Field `SMEN` writer - Supply Monitor Wake-up Enable"]
pub type SMEN_W<'a, const O: u8> = crate::BitWriter<'a, WUMR_SPEC, O, SMENSELECT_A>;
impl<'a, const O: u8> SMEN_W<'a, O> {
    #[doc = "The supply monitor detection has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMENSELECT_A::NOT_ENABLE)
    }
    #[doc = "The supply monitor detection forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMENSELECT_A::ENABLE)
    }
}
#[doc = "Field `RTTEN` reader - Real-time Timer Wake-up Enable"]
pub type RTTEN_R = crate::BitReader<RTTENSELECT_A>;
#[doc = "Real-time Timer Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTTENSELECT_A {
    #[doc = "0: The RTT alarm signal has no wake-up effect."]
    NOT_ENABLE = 0,
    #[doc = "1: The RTT alarm signal forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<RTTENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RTTENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RTTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTTENSELECT_A {
        match self.bits {
            false => RTTENSELECT_A::NOT_ENABLE,
            true => RTTENSELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTTENSELECT_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTTENSELECT_A::ENABLE
    }
}
#[doc = "Field `RTTEN` writer - Real-time Timer Wake-up Enable"]
pub type RTTEN_W<'a, const O: u8> = crate::BitWriter<'a, WUMR_SPEC, O, RTTENSELECT_A>;
impl<'a, const O: u8> RTTEN_W<'a, O> {
    #[doc = "The RTT alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTTENSELECT_A::NOT_ENABLE)
    }
    #[doc = "The RTT alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTTENSELECT_A::ENABLE)
    }
}
#[doc = "Field `RTCEN` reader - Real-time Clock Wake-up Enable"]
pub type RTCEN_R = crate::BitReader<RTCENSELECT_A>;
#[doc = "Real-time Clock Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCENSELECT_A {
    #[doc = "0: The RTC alarm signal has no wake-up effect."]
    NOT_ENABLE = 0,
    #[doc = "1: The RTC alarm signal forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<RTCENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RTCENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCENSELECT_A {
        match self.bits {
            false => RTCENSELECT_A::NOT_ENABLE,
            true => RTCENSELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTCENSELECT_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTCENSELECT_A::ENABLE
    }
}
#[doc = "Field `RTCEN` writer - Real-time Clock Wake-up Enable"]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, WUMR_SPEC, O, RTCENSELECT_A>;
impl<'a, const O: u8> RTCEN_W<'a, O> {
    #[doc = "The RTC alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTCENSELECT_A::NOT_ENABLE)
    }
    #[doc = "The RTC alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTCENSELECT_A::ENABLE)
    }
}
#[doc = "Field `LPDBCEN0` reader - Low-power Debouncer Enable WKUP0"]
pub type LPDBCEN0_R = crate::BitReader<LPDBCEN0SELECT_A>;
#[doc = "Low-power Debouncer Enable WKUP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPDBCEN0SELECT_A {
    #[doc = "0: The WKUP0 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE = 0,
    #[doc = "1: The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE = 1,
}
impl From<LPDBCEN0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCEN0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LPDBCEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCEN0SELECT_A {
        match self.bits {
            false => LPDBCEN0SELECT_A::NOT_ENABLE,
            true => LPDBCEN0SELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCEN0SELECT_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCEN0SELECT_A::ENABLE
    }
}
#[doc = "Field `LPDBCEN0` writer - Low-power Debouncer Enable WKUP0"]
pub type LPDBCEN0_W<'a, const O: u8> = crate::BitWriter<'a, WUMR_SPEC, O, LPDBCEN0SELECT_A>;
impl<'a, const O: u8> LPDBCEN0_W<'a, O> {
    #[doc = "The WKUP0 input pin is not connected to the low-power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCEN0SELECT_A::NOT_ENABLE)
    }
    #[doc = "The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCEN0SELECT_A::ENABLE)
    }
}
#[doc = "Field `LPDBCEN1` reader - Low-power Debouncer Enable WKUP1"]
pub type LPDBCEN1_R = crate::BitReader<LPDBCEN1SELECT_A>;
#[doc = "Low-power Debouncer Enable WKUP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPDBCEN1SELECT_A {
    #[doc = "0: The WKUP1 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE = 0,
    #[doc = "1: The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE = 1,
}
impl From<LPDBCEN1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCEN1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LPDBCEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCEN1SELECT_A {
        match self.bits {
            false => LPDBCEN1SELECT_A::NOT_ENABLE,
            true => LPDBCEN1SELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCEN1SELECT_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCEN1SELECT_A::ENABLE
    }
}
#[doc = "Field `LPDBCEN1` writer - Low-power Debouncer Enable WKUP1"]
pub type LPDBCEN1_W<'a, const O: u8> = crate::BitWriter<'a, WUMR_SPEC, O, LPDBCEN1SELECT_A>;
impl<'a, const O: u8> LPDBCEN1_W<'a, O> {
    #[doc = "The WKUP1 input pin is not connected to the low-power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCEN1SELECT_A::NOT_ENABLE)
    }
    #[doc = "The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCEN1SELECT_A::ENABLE)
    }
}
#[doc = "Field `LPDBCCLR` reader - Low-power Debouncer Clear"]
pub type LPDBCCLR_R = crate::BitReader<LPDBCCLRSELECT_A>;
#[doc = "Low-power Debouncer Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPDBCCLRSELECT_A {
    #[doc = "0: A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    NOT_ENABLE = 0,
    #[doc = "1: A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    ENABLE = 1,
}
impl From<LPDBCCLRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCCLRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LPDBCCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCCLRSELECT_A {
        match self.bits {
            false => LPDBCCLRSELECT_A::NOT_ENABLE,
            true => LPDBCCLRSELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCCLRSELECT_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCCLRSELECT_A::ENABLE
    }
}
#[doc = "Field `LPDBCCLR` writer - Low-power Debouncer Clear"]
pub type LPDBCCLR_W<'a, const O: u8> = crate::BitWriter<'a, WUMR_SPEC, O, LPDBCCLRSELECT_A>;
impl<'a, const O: u8> LPDBCCLR_W<'a, O> {
    #[doc = "A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCCLRSELECT_A::NOT_ENABLE)
    }
    #[doc = "A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCCLRSELECT_A::ENABLE)
    }
}
#[doc = "Field `WKUPDBC` reader - Wake-up Inputs Debouncer Period"]
pub type WKUPDBC_R = crate::FieldReader<WKUPDBCSELECT_A>;
#[doc = "Wake-up Inputs Debouncer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WKUPDBCSELECT_A {
    #[doc = "0: Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE = 0,
    #[doc = "1: WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3_SLCK = 1,
    #[doc = "2: WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32_SLCK = 2,
    #[doc = "3: WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512_SLCK = 3,
    #[doc = "4: WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096_SLCK = 4,
    #[doc = "5: WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768_SLCK = 5,
}
impl From<WKUPDBCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WKUPDBCSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WKUPDBCSELECT_A {
    type Ux = u8;
}
impl WKUPDBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WKUPDBCSELECT_A> {
        match self.bits {
            0 => Some(WKUPDBCSELECT_A::IMMEDIATE),
            1 => Some(WKUPDBCSELECT_A::_3_SLCK),
            2 => Some(WKUPDBCSELECT_A::_32_SLCK),
            3 => Some(WKUPDBCSELECT_A::_512_SLCK),
            4 => Some(WKUPDBCSELECT_A::_4096_SLCK),
            5 => Some(WKUPDBCSELECT_A::_32768_SLCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == WKUPDBCSELECT_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `_3_SLCK`"]
    #[inline(always)]
    pub fn is_3_slck(&self) -> bool {
        *self == WKUPDBCSELECT_A::_3_SLCK
    }
    #[doc = "Checks if the value of the field is `_32_SLCK`"]
    #[inline(always)]
    pub fn is_32_slck(&self) -> bool {
        *self == WKUPDBCSELECT_A::_32_SLCK
    }
    #[doc = "Checks if the value of the field is `_512_SLCK`"]
    #[inline(always)]
    pub fn is_512_slck(&self) -> bool {
        *self == WKUPDBCSELECT_A::_512_SLCK
    }
    #[doc = "Checks if the value of the field is `_4096_SLCK`"]
    #[inline(always)]
    pub fn is_4096_slck(&self) -> bool {
        *self == WKUPDBCSELECT_A::_4096_SLCK
    }
    #[doc = "Checks if the value of the field is `_32768_SLCK`"]
    #[inline(always)]
    pub fn is_32768_slck(&self) -> bool {
        *self == WKUPDBCSELECT_A::_32768_SLCK
    }
}
#[doc = "Field `WKUPDBC` writer - Wake-up Inputs Debouncer Period"]
pub type WKUPDBC_W<'a, const O: u8> = crate::FieldWriter<'a, WUMR_SPEC, 3, O, WKUPDBCSELECT_A>;
impl<'a, const O: u8> WKUPDBC_W<'a, O> {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(WKUPDBCSELECT_A::IMMEDIATE)
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn _3_slck(self) -> &'a mut W {
        self.variant(WKUPDBCSELECT_A::_3_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn _32_slck(self) -> &'a mut W {
        self.variant(WKUPDBCSELECT_A::_32_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn _512_slck(self) -> &'a mut W {
        self.variant(WKUPDBCSELECT_A::_512_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn _4096_slck(self) -> &'a mut W {
        self.variant(WKUPDBCSELECT_A::_4096_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn _32768_slck(self) -> &'a mut W {
        self.variant(WKUPDBCSELECT_A::_32768_SLCK)
    }
}
#[doc = "Field `LPDBC` reader - Low-power Debouncer Period"]
pub type LPDBC_R = crate::FieldReader<LPDBCSELECT_A>;
#[doc = "Low-power Debouncer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPDBCSELECT_A {
    #[doc = "0: Disable the low-power debouncers."]
    DISABLE = 0,
    #[doc = "1: WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    _2_RTCOUT = 1,
    #[doc = "2: WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    _3_RTCOUT = 2,
    #[doc = "3: WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    _4_RTCOUT = 3,
    #[doc = "4: WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    _5_RTCOUT = 4,
    #[doc = "5: WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    _6_RTCOUT = 5,
    #[doc = "6: WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    _7_RTCOUT = 6,
    #[doc = "7: WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    _8_RTCOUT = 7,
}
impl From<LPDBCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LPDBCSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPDBCSELECT_A {
    type Ux = u8;
}
impl LPDBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCSELECT_A {
        match self.bits {
            0 => LPDBCSELECT_A::DISABLE,
            1 => LPDBCSELECT_A::_2_RTCOUT,
            2 => LPDBCSELECT_A::_3_RTCOUT,
            3 => LPDBCSELECT_A::_4_RTCOUT,
            4 => LPDBCSELECT_A::_5_RTCOUT,
            5 => LPDBCSELECT_A::_6_RTCOUT,
            6 => LPDBCSELECT_A::_7_RTCOUT,
            7 => LPDBCSELECT_A::_8_RTCOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LPDBCSELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `_2_RTCOUT`"]
    #[inline(always)]
    pub fn is_2_rtcout(&self) -> bool {
        *self == LPDBCSELECT_A::_2_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_3_RTCOUT`"]
    #[inline(always)]
    pub fn is_3_rtcout(&self) -> bool {
        *self == LPDBCSELECT_A::_3_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_4_RTCOUT`"]
    #[inline(always)]
    pub fn is_4_rtcout(&self) -> bool {
        *self == LPDBCSELECT_A::_4_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_5_RTCOUT`"]
    #[inline(always)]
    pub fn is_5_rtcout(&self) -> bool {
        *self == LPDBCSELECT_A::_5_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_6_RTCOUT`"]
    #[inline(always)]
    pub fn is_6_rtcout(&self) -> bool {
        *self == LPDBCSELECT_A::_6_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_7_RTCOUT`"]
    #[inline(always)]
    pub fn is_7_rtcout(&self) -> bool {
        *self == LPDBCSELECT_A::_7_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_8_RTCOUT`"]
    #[inline(always)]
    pub fn is_8_rtcout(&self) -> bool {
        *self == LPDBCSELECT_A::_8_RTCOUT
    }
}
#[doc = "Field `LPDBC` writer - Low-power Debouncer Period"]
pub type LPDBC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, WUMR_SPEC, 3, O, LPDBCSELECT_A>;
impl<'a, const O: u8> LPDBC_W<'a, O> {
    #[doc = "Disable the low-power debouncers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LPDBCSELECT_A::DISABLE)
    }
    #[doc = "WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _2_rtcout(self) -> &'a mut W {
        self.variant(LPDBCSELECT_A::_2_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _3_rtcout(self) -> &'a mut W {
        self.variant(LPDBCSELECT_A::_3_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _4_rtcout(self) -> &'a mut W {
        self.variant(LPDBCSELECT_A::_4_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _5_rtcout(self) -> &'a mut W {
        self.variant(LPDBCSELECT_A::_5_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _6_rtcout(self) -> &'a mut W {
        self.variant(LPDBCSELECT_A::_6_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _7_rtcout(self) -> &'a mut W {
        self.variant(LPDBCSELECT_A::_7_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _8_rtcout(self) -> &'a mut W {
        self.variant(LPDBCSELECT_A::_8_RTCOUT)
    }
}
impl R {
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real-time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&self) -> RTTEN_R {
        RTTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real-time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Low-power Debouncer Enable WKUP0"]
    #[inline(always)]
    pub fn lpdbcen0(&self) -> LPDBCEN0_R {
        LPDBCEN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-power Debouncer Enable WKUP1"]
    #[inline(always)]
    pub fn lpdbcen1(&self) -> LPDBCEN1_R {
        LPDBCEN1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low-power Debouncer Clear"]
    #[inline(always)]
    pub fn lpdbcclr(&self) -> LPDBCCLR_R {
        LPDBCCLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&self) -> WKUPDBC_R {
        WKUPDBC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Low-power Debouncer Period"]
    #[inline(always)]
    pub fn lpdbc(&self) -> LPDBC_R {
        LPDBC_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smen(&mut self) -> SMEN_W<1> {
        SMEN_W::new(self)
    }
    #[doc = "Bit 2 - Real-time Timer Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtten(&mut self) -> RTTEN_W<2> {
        RTTEN_W::new(self)
    }
    #[doc = "Bit 3 - Real-time Clock Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<3> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 5 - Low-power Debouncer Enable WKUP0"]
    #[inline(always)]
    #[must_use]
    pub fn lpdbcen0(&mut self) -> LPDBCEN0_W<5> {
        LPDBCEN0_W::new(self)
    }
    #[doc = "Bit 6 - Low-power Debouncer Enable WKUP1"]
    #[inline(always)]
    #[must_use]
    pub fn lpdbcen1(&mut self) -> LPDBCEN1_W<6> {
        LPDBCEN1_W::new(self)
    }
    #[doc = "Bit 7 - Low-power Debouncer Clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpdbcclr(&mut self) -> LPDBCCLR_W<7> {
        LPDBCCLR_W::new(self)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    #[must_use]
    pub fn wkupdbc(&mut self) -> WKUPDBC_W<12> {
        WKUPDBC_W::new(self)
    }
    #[doc = "Bits 16:18 - Low-power Debouncer Period"]
    #[inline(always)]
    #[must_use]
    pub fn lpdbc(&mut self) -> LPDBC_W<16> {
        LPDBC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Wake-up Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wumr](index.html) module"]
pub struct WUMR_SPEC;
impl crate::RegisterSpec for WUMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wumr::R](R) reader structure"]
impl crate::Readable for WUMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wumr::W](W) writer structure"]
impl crate::Writable for WUMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUMR to value 0"]
impl crate::Resettable for WUMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
