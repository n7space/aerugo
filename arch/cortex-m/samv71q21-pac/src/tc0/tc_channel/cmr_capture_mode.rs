#[doc = "Register `CMR_CAPTURE_MODE` reader"]
pub struct R(crate::R<CMR_CAPTURE_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMR_CAPTURE_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMR_CAPTURE_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMR_CAPTURE_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMR_CAPTURE_MODE` writer"]
pub struct W(crate::W<CMR_CAPTURE_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMR_CAPTURE_MODE_SPEC>;
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
impl From<crate::W<CMR_CAPTURE_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMR_CAPTURE_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCCLKS` reader - Clock Selection"]
pub type TCCLKS_R = crate::FieldReader<TCCLKSSELECT_A>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCLKSSELECT_A {
    #[doc = "0: Clock selected: internal PCK6 clock signal (from PMC)"]
    TIMER_CLOCK1 = 0,
    #[doc = "1: Clock selected: internal MCK/8 clock signal (from PMC)"]
    TIMER_CLOCK2 = 1,
    #[doc = "2: Clock selected: internal MCK/32 clock signal (from PMC)"]
    TIMER_CLOCK3 = 2,
    #[doc = "3: Clock selected: internal MCK/128 clock signal (from PMC)"]
    TIMER_CLOCK4 = 3,
    #[doc = "4: Clock selected: internal SLCK clock signal (from PMC)"]
    TIMER_CLOCK5 = 4,
    #[doc = "5: Clock selected: XC0"]
    XC0 = 5,
    #[doc = "6: Clock selected: XC1"]
    XC1 = 6,
    #[doc = "7: Clock selected: XC2"]
    XC2 = 7,
}
impl From<TCCLKSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCLKSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCLKSSELECT_A {
    type Ux = u8;
}
impl TCCLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCCLKSSELECT_A {
        match self.bits {
            0 => TCCLKSSELECT_A::TIMER_CLOCK1,
            1 => TCCLKSSELECT_A::TIMER_CLOCK2,
            2 => TCCLKSSELECT_A::TIMER_CLOCK3,
            3 => TCCLKSSELECT_A::TIMER_CLOCK4,
            4 => TCCLKSSELECT_A::TIMER_CLOCK5,
            5 => TCCLKSSELECT_A::XC0,
            6 => TCCLKSSELECT_A::XC1,
            7 => TCCLKSSELECT_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK1`"]
    #[inline(always)]
    pub fn is_timer_clock1(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_CLOCK1
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK2`"]
    #[inline(always)]
    pub fn is_timer_clock2(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_CLOCK2
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK3`"]
    #[inline(always)]
    pub fn is_timer_clock3(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_CLOCK3
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK4`"]
    #[inline(always)]
    pub fn is_timer_clock4(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_CLOCK4
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK5`"]
    #[inline(always)]
    pub fn is_timer_clock5(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_CLOCK5
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == TCCLKSSELECT_A::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == TCCLKSSELECT_A::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == TCCLKSSELECT_A::XC2
    }
}
#[doc = "Field `TCCLKS` writer - Clock Selection"]
pub type TCCLKS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, CMR_CAPTURE_MODE_SPEC, 3, O, TCCLKSSELECT_A>;
impl<'a, const O: u8> TCCLKS_W<'a, O> {
    #[doc = "Clock selected: internal PCK6 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock1(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_CLOCK1)
    }
    #[doc = "Clock selected: internal MCK/8 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock2(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_CLOCK2)
    }
    #[doc = "Clock selected: internal MCK/32 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock3(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_CLOCK3)
    }
    #[doc = "Clock selected: internal MCK/128 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock4(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_CLOCK4)
    }
    #[doc = "Clock selected: internal SLCK clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock5(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_CLOCK5)
    }
    #[doc = "Clock selected: XC0"]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::XC0)
    }
    #[doc = "Clock selected: XC1"]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::XC1)
    }
    #[doc = "Clock selected: XC2"]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::XC2)
    }
}
#[doc = "Field `CLKI` reader - Clock Invert"]
pub type CLKI_R = crate::BitReader;
#[doc = "Field `CLKI` writer - Clock Invert"]
pub type CLKI_W<'a, const O: u8> = crate::BitWriter<'a, CMR_CAPTURE_MODE_SPEC, O>;
#[doc = "Field `BURST` reader - Burst Signal Selection"]
pub type BURST_R = crate::FieldReader<BURSTSELECT_A>;
#[doc = "Burst Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BURSTSELECT_A {
    #[doc = "0: The clock is not gated by an external signal."]
    NONE = 0,
    #[doc = "1: XC0 is ANDed with the selected clock."]
    XC0 = 1,
    #[doc = "2: XC1 is ANDed with the selected clock."]
    XC1 = 2,
    #[doc = "3: XC2 is ANDed with the selected clock."]
    XC2 = 3,
}
impl From<BURSTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BURSTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BURSTSELECT_A {
    type Ux = u8;
}
impl BURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTSELECT_A {
        match self.bits {
            0 => BURSTSELECT_A::NONE,
            1 => BURSTSELECT_A::XC0,
            2 => BURSTSELECT_A::XC1,
            3 => BURSTSELECT_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BURSTSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == BURSTSELECT_A::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == BURSTSELECT_A::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == BURSTSELECT_A::XC2
    }
}
#[doc = "Field `BURST` writer - Burst Signal Selection"]
pub type BURST_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, CMR_CAPTURE_MODE_SPEC, 2, O, BURSTSELECT_A>;
impl<'a, const O: u8> BURST_W<'a, O> {
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BURSTSELECT_A::NONE)
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(BURSTSELECT_A::XC0)
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(BURSTSELECT_A::XC1)
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(BURSTSELECT_A::XC2)
    }
}
#[doc = "Field `LDBSTOP` reader - Counter Clock Stopped with RB Loading"]
pub type LDBSTOP_R = crate::BitReader;
#[doc = "Field `LDBSTOP` writer - Counter Clock Stopped with RB Loading"]
pub type LDBSTOP_W<'a, const O: u8> = crate::BitWriter<'a, CMR_CAPTURE_MODE_SPEC, O>;
#[doc = "Field `LDBDIS` reader - Counter Clock Disable with RB Loading"]
pub type LDBDIS_R = crate::BitReader;
#[doc = "Field `LDBDIS` writer - Counter Clock Disable with RB Loading"]
pub type LDBDIS_W<'a, const O: u8> = crate::BitWriter<'a, CMR_CAPTURE_MODE_SPEC, O>;
#[doc = "Field `ETRGEDG` reader - External Trigger Edge Selection"]
pub type ETRGEDG_R = crate::FieldReader<ETRGEDGSELECT_A>;
#[doc = "External Trigger Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETRGEDGSELECT_A {
    #[doc = "0: The clock is not gated by an external signal."]
    NONE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Each edge"]
    EDGE = 3,
}
impl From<ETRGEDGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRGEDGSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETRGEDGSELECT_A {
    type Ux = u8;
}
impl ETRGEDG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETRGEDGSELECT_A {
        match self.bits {
            0 => ETRGEDGSELECT_A::NONE,
            1 => ETRGEDGSELECT_A::RISING,
            2 => ETRGEDGSELECT_A::FALLING,
            3 => ETRGEDGSELECT_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ETRGEDGSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ETRGEDGSELECT_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ETRGEDGSELECT_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ETRGEDGSELECT_A::EDGE
    }
}
#[doc = "Field `ETRGEDG` writer - External Trigger Edge Selection"]
pub type ETRGEDG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, CMR_CAPTURE_MODE_SPEC, 2, O, ETRGEDGSELECT_A>;
impl<'a, const O: u8> ETRGEDG_W<'a, O> {
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ETRGEDGSELECT_A::NONE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ETRGEDGSELECT_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ETRGEDGSELECT_A::FALLING)
    }
    #[doc = "Each edge"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ETRGEDGSELECT_A::EDGE)
    }
}
#[doc = "Field `ABETRG` reader - TIOAx or TIOBx External Trigger Selection"]
pub type ABETRG_R = crate::BitReader;
#[doc = "Field `ABETRG` writer - TIOAx or TIOBx External Trigger Selection"]
pub type ABETRG_W<'a, const O: u8> = crate::BitWriter<'a, CMR_CAPTURE_MODE_SPEC, O>;
#[doc = "Field `CPCTRG` reader - RC Compare Trigger Enable"]
pub type CPCTRG_R = crate::BitReader;
#[doc = "Field `CPCTRG` writer - RC Compare Trigger Enable"]
pub type CPCTRG_W<'a, const O: u8> = crate::BitWriter<'a, CMR_CAPTURE_MODE_SPEC, O>;
#[doc = "Field `WAVE` reader - Waveform Mode"]
pub type WAVE_R = crate::BitReader;
#[doc = "Field `WAVE` writer - Waveform Mode"]
pub type WAVE_W<'a, const O: u8> = crate::BitWriter<'a, CMR_CAPTURE_MODE_SPEC, O>;
#[doc = "Field `LDRA` reader - RA Loading Edge Selection"]
pub type LDRA_R = crate::FieldReader<LDRASELECT_A>;
#[doc = "RA Loading Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDRASELECT_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: Rising edge of TIOAx"]
    RISING = 1,
    #[doc = "2: Falling edge of TIOAx"]
    FALLING = 2,
    #[doc = "3: Each edge of TIOAx"]
    EDGE = 3,
}
impl From<LDRASELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LDRASELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LDRASELECT_A {
    type Ux = u8;
}
impl LDRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRASELECT_A {
        match self.bits {
            0 => LDRASELECT_A::NONE,
            1 => LDRASELECT_A::RISING,
            2 => LDRASELECT_A::FALLING,
            3 => LDRASELECT_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LDRASELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == LDRASELECT_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == LDRASELECT_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == LDRASELECT_A::EDGE
    }
}
#[doc = "Field `LDRA` writer - RA Loading Edge Selection"]
pub type LDRA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, CMR_CAPTURE_MODE_SPEC, 2, O, LDRASELECT_A>;
impl<'a, const O: u8> LDRA_W<'a, O> {
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LDRASELECT_A::NONE)
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(LDRASELECT_A::RISING)
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(LDRASELECT_A::FALLING)
    }
    #[doc = "Each edge of TIOAx"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(LDRASELECT_A::EDGE)
    }
}
#[doc = "Field `LDRB` reader - RB Loading Edge Selection"]
pub type LDRB_R = crate::FieldReader<LDRBSELECT_A>;
#[doc = "RB Loading Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDRBSELECT_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: Rising edge of TIOAx"]
    RISING = 1,
    #[doc = "2: Falling edge of TIOAx"]
    FALLING = 2,
    #[doc = "3: Each edge of TIOAx"]
    EDGE = 3,
}
impl From<LDRBSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LDRBSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LDRBSELECT_A {
    type Ux = u8;
}
impl LDRB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRBSELECT_A {
        match self.bits {
            0 => LDRBSELECT_A::NONE,
            1 => LDRBSELECT_A::RISING,
            2 => LDRBSELECT_A::FALLING,
            3 => LDRBSELECT_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LDRBSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == LDRBSELECT_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == LDRBSELECT_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == LDRBSELECT_A::EDGE
    }
}
#[doc = "Field `LDRB` writer - RB Loading Edge Selection"]
pub type LDRB_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, CMR_CAPTURE_MODE_SPEC, 2, O, LDRBSELECT_A>;
impl<'a, const O: u8> LDRB_W<'a, O> {
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LDRBSELECT_A::NONE)
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(LDRBSELECT_A::RISING)
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(LDRBSELECT_A::FALLING)
    }
    #[doc = "Each edge of TIOAx"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(LDRBSELECT_A::EDGE)
    }
}
#[doc = "Field `SBSMPLR` reader - Loading Edge Subsampling Ratio"]
pub type SBSMPLR_R = crate::FieldReader<SBSMPLRSELECT_A>;
#[doc = "Loading Edge Subsampling Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SBSMPLRSELECT_A {
    #[doc = "0: Load a Capture Register each selected edge"]
    ONE = 0,
    #[doc = "1: Load a Capture Register every 2 selected edges"]
    HALF = 1,
    #[doc = "2: Load a Capture Register every 4 selected edges"]
    FOURTH = 2,
    #[doc = "3: Load a Capture Register every 8 selected edges"]
    EIGHTH = 3,
    #[doc = "4: Load a Capture Register every 16 selected edges"]
    SIXTEENTH = 4,
}
impl From<SBSMPLRSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SBSMPLRSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SBSMPLRSELECT_A {
    type Ux = u8;
}
impl SBSMPLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SBSMPLRSELECT_A> {
        match self.bits {
            0 => Some(SBSMPLRSELECT_A::ONE),
            1 => Some(SBSMPLRSELECT_A::HALF),
            2 => Some(SBSMPLRSELECT_A::FOURTH),
            3 => Some(SBSMPLRSELECT_A::EIGHTH),
            4 => Some(SBSMPLRSELECT_A::SIXTEENTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SBSMPLRSELECT_A::ONE
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == SBSMPLRSELECT_A::HALF
    }
    #[doc = "Checks if the value of the field is `FOURTH`"]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == SBSMPLRSELECT_A::FOURTH
    }
    #[doc = "Checks if the value of the field is `EIGHTH`"]
    #[inline(always)]
    pub fn is_eighth(&self) -> bool {
        *self == SBSMPLRSELECT_A::EIGHTH
    }
    #[doc = "Checks if the value of the field is `SIXTEENTH`"]
    #[inline(always)]
    pub fn is_sixteenth(&self) -> bool {
        *self == SBSMPLRSELECT_A::SIXTEENTH
    }
}
#[doc = "Field `SBSMPLR` writer - Loading Edge Subsampling Ratio"]
pub type SBSMPLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, CMR_CAPTURE_MODE_SPEC, 3, O, SBSMPLRSELECT_A>;
impl<'a, const O: u8> SBSMPLR_W<'a, O> {
    #[doc = "Load a Capture Register each selected edge"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SBSMPLRSELECT_A::ONE)
    }
    #[doc = "Load a Capture Register every 2 selected edges"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(SBSMPLRSELECT_A::HALF)
    }
    #[doc = "Load a Capture Register every 4 selected edges"]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut W {
        self.variant(SBSMPLRSELECT_A::FOURTH)
    }
    #[doc = "Load a Capture Register every 8 selected edges"]
    #[inline(always)]
    pub fn eighth(self) -> &'a mut W {
        self.variant(SBSMPLRSELECT_A::EIGHTH)
    }
    #[doc = "Load a Capture Register every 16 selected edges"]
    #[inline(always)]
    pub fn sixteenth(self) -> &'a mut W {
        self.variant(SBSMPLRSELECT_A::SIXTEENTH)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&self) -> TCCLKS_R {
        TCCLKS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&self) -> CLKI_R {
        CLKI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&self) -> LDBSTOP_R {
        LDBSTOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&self) -> LDBDIS_R {
        LDBDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&self) -> ETRGEDG_R {
        ETRGEDG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - TIOAx or TIOBx External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&self) -> ABETRG_R {
        ABETRG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&self) -> CPCTRG_R {
        CPCTRG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    pub fn ldra(&self) -> LDRA_R {
        LDRA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    pub fn ldrb(&self) -> LDRB_R {
        LDRB_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Loading Edge Subsampling Ratio"]
    #[inline(always)]
    pub fn sbsmplr(&self) -> SBSMPLR_R {
        SBSMPLR_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tcclks(&mut self) -> TCCLKS_W<0> {
        TCCLKS_W::new(self)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    #[must_use]
    pub fn clki(&mut self) -> CLKI_W<3> {
        CLKI_W::new(self)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    #[must_use]
    pub fn burst(&mut self) -> BURST_W<4> {
        BURST_W::new(self)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldbstop(&mut self) -> LDBSTOP_W<6> {
        LDBSTOP_W::new(self)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldbdis(&mut self) -> LDBDIS_W<7> {
        LDBDIS_W::new(self)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn etrgedg(&mut self) -> ETRGEDG_W<8> {
        ETRGEDG_W::new(self)
    }
    #[doc = "Bit 10 - TIOAx or TIOBx External Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn abetrg(&mut self) -> ABETRG_W<10> {
        ABETRG_W::new(self)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpctrg(&mut self) -> CPCTRG_W<14> {
        CPCTRG_W::new(self)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WAVE_W<15> {
        WAVE_W::new(self)
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ldra(&mut self) -> LDRA_W<16> {
        LDRA_W::new(self)
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ldrb(&mut self) -> LDRB_W<18> {
        LDRB_W::new(self)
    }
    #[doc = "Bits 20:22 - Loading Edge Subsampling Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn sbsmplr(&mut self) -> SBSMPLR_W<20> {
        SBSMPLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr_capture_mode](index.html) module"]
pub struct CMR_CAPTURE_MODE_SPEC;
impl crate::RegisterSpec for CMR_CAPTURE_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmr_capture_mode::R](R) reader structure"]
impl crate::Readable for CMR_CAPTURE_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmr_capture_mode::W](W) writer structure"]
impl crate::Writable for CMR_CAPTURE_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMR_CAPTURE_MODE to value 0"]
impl crate::Resettable for CMR_CAPTURE_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
