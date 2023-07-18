#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HRMOD` reader - 12-/24-hour Mode"]
pub type HRMOD_R = crate::BitReader;
#[doc = "Field `HRMOD` writer - 12-/24-hour Mode"]
pub type HRMOD_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `PERSIAN` reader - PERSIAN Calendar"]
pub type PERSIAN_R = crate::BitReader;
#[doc = "Field `PERSIAN` writer - PERSIAN Calendar"]
pub type PERSIAN_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `NEGPPM` reader - NEGative PPM Correction"]
pub type NEGPPM_R = crate::BitReader;
#[doc = "Field `NEGPPM` writer - NEGative PPM Correction"]
pub type NEGPPM_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `CORRECTION` reader - Slow Clock Correction"]
pub type CORRECTION_R = crate::FieldReader;
#[doc = "Field `CORRECTION` writer - Slow Clock Correction"]
pub type CORRECTION_W<'a, const O: u8> = crate::FieldWriter<'a, MR_SPEC, 7, O>;
#[doc = "Field `HIGHPPM` reader - HIGH PPM Correction"]
pub type HIGHPPM_R = crate::BitReader;
#[doc = "Field `HIGHPPM` writer - HIGH PPM Correction"]
pub type HIGHPPM_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `OUT0` reader - RTCOUT0 OutputSource Selection"]
pub type OUT0_R = crate::FieldReader<OUT0SELECT_A>;
#[doc = "RTCOUT0 OutputSource Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT0SELECT_A {
    #[doc = "0: No waveform, stuck at '0'"]
    NO_WAVE = 0,
    #[doc = "1: 1 Hz square wave"]
    FREQ1HZ = 1,
    #[doc = "2: 32 Hz square wave"]
    FREQ32HZ = 2,
    #[doc = "3: 64 Hz square wave"]
    FREQ64HZ = 3,
    #[doc = "4: 512 Hz square wave"]
    FREQ512HZ = 4,
    #[doc = "5: Output toggles when alarm flag rises"]
    ALARM_TOGGLE = 5,
    #[doc = "6: Output is a copy of the alarm flag"]
    ALARM_FLAG = 6,
    #[doc = "7: Duty cycle programmable pulse"]
    PROG_PULSE = 7,
}
impl From<OUT0SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT0SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OUT0SELECT_A {
    type Ux = u8;
}
impl OUT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT0SELECT_A {
        match self.bits {
            0 => OUT0SELECT_A::NO_WAVE,
            1 => OUT0SELECT_A::FREQ1HZ,
            2 => OUT0SELECT_A::FREQ32HZ,
            3 => OUT0SELECT_A::FREQ64HZ,
            4 => OUT0SELECT_A::FREQ512HZ,
            5 => OUT0SELECT_A::ALARM_TOGGLE,
            6 => OUT0SELECT_A::ALARM_FLAG,
            7 => OUT0SELECT_A::PROG_PULSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_WAVE`"]
    #[inline(always)]
    pub fn is_no_wave(&self) -> bool {
        *self == OUT0SELECT_A::NO_WAVE
    }
    #[doc = "Checks if the value of the field is `FREQ1HZ`"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        *self == OUT0SELECT_A::FREQ1HZ
    }
    #[doc = "Checks if the value of the field is `FREQ32HZ`"]
    #[inline(always)]
    pub fn is_freq32hz(&self) -> bool {
        *self == OUT0SELECT_A::FREQ32HZ
    }
    #[doc = "Checks if the value of the field is `FREQ64HZ`"]
    #[inline(always)]
    pub fn is_freq64hz(&self) -> bool {
        *self == OUT0SELECT_A::FREQ64HZ
    }
    #[doc = "Checks if the value of the field is `FREQ512HZ`"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        *self == OUT0SELECT_A::FREQ512HZ
    }
    #[doc = "Checks if the value of the field is `ALARM_TOGGLE`"]
    #[inline(always)]
    pub fn is_alarm_toggle(&self) -> bool {
        *self == OUT0SELECT_A::ALARM_TOGGLE
    }
    #[doc = "Checks if the value of the field is `ALARM_FLAG`"]
    #[inline(always)]
    pub fn is_alarm_flag(&self) -> bool {
        *self == OUT0SELECT_A::ALARM_FLAG
    }
    #[doc = "Checks if the value of the field is `PROG_PULSE`"]
    #[inline(always)]
    pub fn is_prog_pulse(&self) -> bool {
        *self == OUT0SELECT_A::PROG_PULSE
    }
}
#[doc = "Field `OUT0` writer - RTCOUT0 OutputSource Selection"]
pub type OUT0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, MR_SPEC, 3, O, OUT0SELECT_A>;
impl<'a, const O: u8> OUT0_W<'a, O> {
    #[doc = "No waveform, stuck at '0'"]
    #[inline(always)]
    pub fn no_wave(self) -> &'a mut W {
        self.variant(OUT0SELECT_A::NO_WAVE)
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut W {
        self.variant(OUT0SELECT_A::FREQ1HZ)
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn freq32hz(self) -> &'a mut W {
        self.variant(OUT0SELECT_A::FREQ32HZ)
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn freq64hz(self) -> &'a mut W {
        self.variant(OUT0SELECT_A::FREQ64HZ)
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut W {
        self.variant(OUT0SELECT_A::FREQ512HZ)
    }
    #[doc = "Output toggles when alarm flag rises"]
    #[inline(always)]
    pub fn alarm_toggle(self) -> &'a mut W {
        self.variant(OUT0SELECT_A::ALARM_TOGGLE)
    }
    #[doc = "Output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn alarm_flag(self) -> &'a mut W {
        self.variant(OUT0SELECT_A::ALARM_FLAG)
    }
    #[doc = "Duty cycle programmable pulse"]
    #[inline(always)]
    pub fn prog_pulse(self) -> &'a mut W {
        self.variant(OUT0SELECT_A::PROG_PULSE)
    }
}
#[doc = "Field `OUT1` reader - RTCOUT1 Output Source Selection"]
pub type OUT1_R = crate::FieldReader<OUT1SELECT_A>;
#[doc = "RTCOUT1 Output Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT1SELECT_A {
    #[doc = "0: No waveform, stuck at '0'"]
    NO_WAVE = 0,
    #[doc = "1: 1 Hz square wave"]
    FREQ1HZ = 1,
    #[doc = "2: 32 Hz square wave"]
    FREQ32HZ = 2,
    #[doc = "3: 64 Hz square wave"]
    FREQ64HZ = 3,
    #[doc = "4: 512 Hz square wave"]
    FREQ512HZ = 4,
    #[doc = "5: Output toggles when alarm flag rises"]
    ALARM_TOGGLE = 5,
    #[doc = "6: Output is a copy of the alarm flag"]
    ALARM_FLAG = 6,
    #[doc = "7: Duty cycle programmable pulse"]
    PROG_PULSE = 7,
}
impl From<OUT1SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT1SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OUT1SELECT_A {
    type Ux = u8;
}
impl OUT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT1SELECT_A {
        match self.bits {
            0 => OUT1SELECT_A::NO_WAVE,
            1 => OUT1SELECT_A::FREQ1HZ,
            2 => OUT1SELECT_A::FREQ32HZ,
            3 => OUT1SELECT_A::FREQ64HZ,
            4 => OUT1SELECT_A::FREQ512HZ,
            5 => OUT1SELECT_A::ALARM_TOGGLE,
            6 => OUT1SELECT_A::ALARM_FLAG,
            7 => OUT1SELECT_A::PROG_PULSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_WAVE`"]
    #[inline(always)]
    pub fn is_no_wave(&self) -> bool {
        *self == OUT1SELECT_A::NO_WAVE
    }
    #[doc = "Checks if the value of the field is `FREQ1HZ`"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        *self == OUT1SELECT_A::FREQ1HZ
    }
    #[doc = "Checks if the value of the field is `FREQ32HZ`"]
    #[inline(always)]
    pub fn is_freq32hz(&self) -> bool {
        *self == OUT1SELECT_A::FREQ32HZ
    }
    #[doc = "Checks if the value of the field is `FREQ64HZ`"]
    #[inline(always)]
    pub fn is_freq64hz(&self) -> bool {
        *self == OUT1SELECT_A::FREQ64HZ
    }
    #[doc = "Checks if the value of the field is `FREQ512HZ`"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        *self == OUT1SELECT_A::FREQ512HZ
    }
    #[doc = "Checks if the value of the field is `ALARM_TOGGLE`"]
    #[inline(always)]
    pub fn is_alarm_toggle(&self) -> bool {
        *self == OUT1SELECT_A::ALARM_TOGGLE
    }
    #[doc = "Checks if the value of the field is `ALARM_FLAG`"]
    #[inline(always)]
    pub fn is_alarm_flag(&self) -> bool {
        *self == OUT1SELECT_A::ALARM_FLAG
    }
    #[doc = "Checks if the value of the field is `PROG_PULSE`"]
    #[inline(always)]
    pub fn is_prog_pulse(&self) -> bool {
        *self == OUT1SELECT_A::PROG_PULSE
    }
}
#[doc = "Field `OUT1` writer - RTCOUT1 Output Source Selection"]
pub type OUT1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, MR_SPEC, 3, O, OUT1SELECT_A>;
impl<'a, const O: u8> OUT1_W<'a, O> {
    #[doc = "No waveform, stuck at '0'"]
    #[inline(always)]
    pub fn no_wave(self) -> &'a mut W {
        self.variant(OUT1SELECT_A::NO_WAVE)
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut W {
        self.variant(OUT1SELECT_A::FREQ1HZ)
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn freq32hz(self) -> &'a mut W {
        self.variant(OUT1SELECT_A::FREQ32HZ)
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn freq64hz(self) -> &'a mut W {
        self.variant(OUT1SELECT_A::FREQ64HZ)
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut W {
        self.variant(OUT1SELECT_A::FREQ512HZ)
    }
    #[doc = "Output toggles when alarm flag rises"]
    #[inline(always)]
    pub fn alarm_toggle(self) -> &'a mut W {
        self.variant(OUT1SELECT_A::ALARM_TOGGLE)
    }
    #[doc = "Output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn alarm_flag(self) -> &'a mut W {
        self.variant(OUT1SELECT_A::ALARM_FLAG)
    }
    #[doc = "Duty cycle programmable pulse"]
    #[inline(always)]
    pub fn prog_pulse(self) -> &'a mut W {
        self.variant(OUT1SELECT_A::PROG_PULSE)
    }
}
#[doc = "Field `THIGH` reader - High Duration of the Output Pulse"]
pub type THIGH_R = crate::FieldReader<THIGHSELECT_A>;
#[doc = "High Duration of the Output Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THIGHSELECT_A {
    #[doc = "0: 31.2 ms"]
    H_31MS = 0,
    #[doc = "1: 15.6 ms"]
    H_16MS = 1,
    #[doc = "2: 3.91 ms"]
    H_4MS = 2,
    #[doc = "3: 976 us"]
    H_976US = 3,
    #[doc = "4: 488 us"]
    H_488US = 4,
    #[doc = "5: 122 us"]
    H_122US = 5,
    #[doc = "6: 30.5 us"]
    H_30US = 6,
    #[doc = "7: 15.2 us"]
    H_15US = 7,
}
impl From<THIGHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: THIGHSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for THIGHSELECT_A {
    type Ux = u8;
}
impl THIGH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THIGHSELECT_A {
        match self.bits {
            0 => THIGHSELECT_A::H_31MS,
            1 => THIGHSELECT_A::H_16MS,
            2 => THIGHSELECT_A::H_4MS,
            3 => THIGHSELECT_A::H_976US,
            4 => THIGHSELECT_A::H_488US,
            5 => THIGHSELECT_A::H_122US,
            6 => THIGHSELECT_A::H_30US,
            7 => THIGHSELECT_A::H_15US,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `H_31MS`"]
    #[inline(always)]
    pub fn is_h_31ms(&self) -> bool {
        *self == THIGHSELECT_A::H_31MS
    }
    #[doc = "Checks if the value of the field is `H_16MS`"]
    #[inline(always)]
    pub fn is_h_16ms(&self) -> bool {
        *self == THIGHSELECT_A::H_16MS
    }
    #[doc = "Checks if the value of the field is `H_4MS`"]
    #[inline(always)]
    pub fn is_h_4ms(&self) -> bool {
        *self == THIGHSELECT_A::H_4MS
    }
    #[doc = "Checks if the value of the field is `H_976US`"]
    #[inline(always)]
    pub fn is_h_976us(&self) -> bool {
        *self == THIGHSELECT_A::H_976US
    }
    #[doc = "Checks if the value of the field is `H_488US`"]
    #[inline(always)]
    pub fn is_h_488us(&self) -> bool {
        *self == THIGHSELECT_A::H_488US
    }
    #[doc = "Checks if the value of the field is `H_122US`"]
    #[inline(always)]
    pub fn is_h_122us(&self) -> bool {
        *self == THIGHSELECT_A::H_122US
    }
    #[doc = "Checks if the value of the field is `H_30US`"]
    #[inline(always)]
    pub fn is_h_30us(&self) -> bool {
        *self == THIGHSELECT_A::H_30US
    }
    #[doc = "Checks if the value of the field is `H_15US`"]
    #[inline(always)]
    pub fn is_h_15us(&self) -> bool {
        *self == THIGHSELECT_A::H_15US
    }
}
#[doc = "Field `THIGH` writer - High Duration of the Output Pulse"]
pub type THIGH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, MR_SPEC, 3, O, THIGHSELECT_A>;
impl<'a, const O: u8> THIGH_W<'a, O> {
    #[doc = "31.2 ms"]
    #[inline(always)]
    pub fn h_31ms(self) -> &'a mut W {
        self.variant(THIGHSELECT_A::H_31MS)
    }
    #[doc = "15.6 ms"]
    #[inline(always)]
    pub fn h_16ms(self) -> &'a mut W {
        self.variant(THIGHSELECT_A::H_16MS)
    }
    #[doc = "3.91 ms"]
    #[inline(always)]
    pub fn h_4ms(self) -> &'a mut W {
        self.variant(THIGHSELECT_A::H_4MS)
    }
    #[doc = "976 us"]
    #[inline(always)]
    pub fn h_976us(self) -> &'a mut W {
        self.variant(THIGHSELECT_A::H_976US)
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn h_488us(self) -> &'a mut W {
        self.variant(THIGHSELECT_A::H_488US)
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn h_122us(self) -> &'a mut W {
        self.variant(THIGHSELECT_A::H_122US)
    }
    #[doc = "30.5 us"]
    #[inline(always)]
    pub fn h_30us(self) -> &'a mut W {
        self.variant(THIGHSELECT_A::H_30US)
    }
    #[doc = "15.2 us"]
    #[inline(always)]
    pub fn h_15us(self) -> &'a mut W {
        self.variant(THIGHSELECT_A::H_15US)
    }
}
#[doc = "Field `TPERIOD` reader - Period of the Output Pulse"]
pub type TPERIOD_R = crate::FieldReader<TPERIODSELECT_A>;
#[doc = "Period of the Output Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPERIODSELECT_A {
    #[doc = "0: 1 second"]
    P_1S = 0,
    #[doc = "1: 500 ms"]
    P_500MS = 1,
    #[doc = "2: 250 ms"]
    P_250MS = 2,
    #[doc = "3: 125 ms"]
    P_125MS = 3,
}
impl From<TPERIODSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TPERIODSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPERIODSELECT_A {
    type Ux = u8;
}
impl TPERIOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPERIODSELECT_A {
        match self.bits {
            0 => TPERIODSELECT_A::P_1S,
            1 => TPERIODSELECT_A::P_500MS,
            2 => TPERIODSELECT_A::P_250MS,
            3 => TPERIODSELECT_A::P_125MS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P_1S`"]
    #[inline(always)]
    pub fn is_p_1s(&self) -> bool {
        *self == TPERIODSELECT_A::P_1S
    }
    #[doc = "Checks if the value of the field is `P_500MS`"]
    #[inline(always)]
    pub fn is_p_500ms(&self) -> bool {
        *self == TPERIODSELECT_A::P_500MS
    }
    #[doc = "Checks if the value of the field is `P_250MS`"]
    #[inline(always)]
    pub fn is_p_250ms(&self) -> bool {
        *self == TPERIODSELECT_A::P_250MS
    }
    #[doc = "Checks if the value of the field is `P_125MS`"]
    #[inline(always)]
    pub fn is_p_125ms(&self) -> bool {
        *self == TPERIODSELECT_A::P_125MS
    }
}
#[doc = "Field `TPERIOD` writer - Period of the Output Pulse"]
pub type TPERIOD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, MR_SPEC, 2, O, TPERIODSELECT_A>;
impl<'a, const O: u8> TPERIOD_W<'a, O> {
    #[doc = "1 second"]
    #[inline(always)]
    pub fn p_1s(self) -> &'a mut W {
        self.variant(TPERIODSELECT_A::P_1S)
    }
    #[doc = "500 ms"]
    #[inline(always)]
    pub fn p_500ms(self) -> &'a mut W {
        self.variant(TPERIODSELECT_A::P_500MS)
    }
    #[doc = "250 ms"]
    #[inline(always)]
    pub fn p_250ms(self) -> &'a mut W {
        self.variant(TPERIODSELECT_A::P_250MS)
    }
    #[doc = "125 ms"]
    #[inline(always)]
    pub fn p_125ms(self) -> &'a mut W {
        self.variant(TPERIODSELECT_A::P_125MS)
    }
}
impl R {
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    pub fn hrmod(&self) -> HRMOD_R {
        HRMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    pub fn persian(&self) -> PERSIAN_R {
        PERSIAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    pub fn negppm(&self) -> NEGPPM_R {
        NEGPPM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    pub fn correction(&self) -> CORRECTION_R {
        CORRECTION_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    pub fn highppm(&self) -> HIGHPPM_R {
        HIGHPPM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - RTCOUT0 OutputSource Selection"]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - RTCOUT1 Output Source Selection"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - High Duration of the Output Pulse"]
    #[inline(always)]
    pub fn thigh(&self) -> THIGH_R {
        THIGH_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Period of the Output Pulse"]
    #[inline(always)]
    pub fn tperiod(&self) -> TPERIOD_R {
        TPERIOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hrmod(&mut self) -> HRMOD_W<0> {
        HRMOD_W::new(self)
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    #[must_use]
    pub fn persian(&mut self) -> PERSIAN_W<1> {
        PERSIAN_W::new(self)
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    #[must_use]
    pub fn negppm(&mut self) -> NEGPPM_W<4> {
        NEGPPM_W::new(self)
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    #[must_use]
    pub fn correction(&mut self) -> CORRECTION_W<8> {
        CORRECTION_W::new(self)
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    #[must_use]
    pub fn highppm(&mut self) -> HIGHPPM_W<15> {
        HIGHPPM_W::new(self)
    }
    #[doc = "Bits 16:18 - RTCOUT0 OutputSource Selection"]
    #[inline(always)]
    #[must_use]
    pub fn out0(&mut self) -> OUT0_W<16> {
        OUT0_W::new(self)
    }
    #[doc = "Bits 20:22 - RTCOUT1 Output Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> OUT1_W<20> {
        OUT1_W::new(self)
    }
    #[doc = "Bits 24:26 - High Duration of the Output Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn thigh(&mut self) -> THIGH_W<24> {
        THIGH_W::new(self)
    }
    #[doc = "Bits 28:29 - Period of the Output Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn tperiod(&mut self) -> TPERIOD_W<28> {
        TPERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
