#[doc = "Register `TRIGR` reader"]
pub struct R(crate::R<TRIGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIGR` writer"]
pub struct W(crate::W<TRIGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIGR_SPEC>;
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
impl From<crate::W<TRIGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGEN0` reader - Trigger Enable of Channel 0"]
pub type TRGEN0_R = crate::BitReader<TRGEN0SELECT_A>;
#[doc = "Trigger Enable of Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGEN0SELECT_A {
    #[doc = "0: External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    DIS = 0,
    #[doc = "1: External trigger mode enabled."]
    EN = 1,
}
impl From<TRGEN0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN0SELECT_A {
        match self.bits {
            false => TRGEN0SELECT_A::DIS,
            true => TRGEN0SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRGEN0SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRGEN0SELECT_A::EN
    }
}
#[doc = "Field `TRGEN0` writer - Trigger Enable of Channel 0"]
pub type TRGEN0_W<'a, const O: u8> = crate::BitWriter<'a, TRIGR_SPEC, O, TRGEN0SELECT_A>;
impl<'a, const O: u8> TRGEN0_W<'a, O> {
    #[doc = "External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGEN0SELECT_A::DIS)
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TRGEN0SELECT_A::EN)
    }
}
#[doc = "Field `TRGEN1` reader - Trigger Enable of Channel 1"]
pub type TRGEN1_R = crate::BitReader<TRGEN1SELECT_A>;
#[doc = "Trigger Enable of Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGEN1SELECT_A {
    #[doc = "0: External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    DIS = 0,
    #[doc = "1: External trigger mode enabled."]
    EN = 1,
}
impl From<TRGEN1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN1SELECT_A {
        match self.bits {
            false => TRGEN1SELECT_A::DIS,
            true => TRGEN1SELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRGEN1SELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRGEN1SELECT_A::EN
    }
}
#[doc = "Field `TRGEN1` writer - Trigger Enable of Channel 1"]
pub type TRGEN1_W<'a, const O: u8> = crate::BitWriter<'a, TRIGR_SPEC, O, TRGEN1SELECT_A>;
impl<'a, const O: u8> TRGEN1_W<'a, O> {
    #[doc = "External trigger mode disabled. DACC is in Free-running mode or Max speed mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGEN1SELECT_A::DIS)
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TRGEN1SELECT_A::EN)
    }
}
#[doc = "Field `TRGSEL0` reader - Trigger Selection of Channel 0"]
pub type TRGSEL0_R = crate::FieldReader<TRGSEL0SELECT_A>;
#[doc = "Trigger Selection of Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGSEL0SELECT_A {
    #[doc = "0: DAC External Trigger Input (DATRG)"]
    TRGSEL0 = 0,
    #[doc = "1: TC0 Channel 0 Output (TIOA0)"]
    TRGSEL1 = 1,
    #[doc = "2: TC0 Channel 1 Output (TIOA1)"]
    TRGSEL2 = 2,
    #[doc = "3: TC0 Channel 2 Output (TIOA2)"]
    TRGSEL3 = 3,
    #[doc = "4: PWM0 Event Line 0"]
    TRGSEL4 = 4,
    #[doc = "5: PWM0 Event Line 1"]
    TRGSEL5 = 5,
    #[doc = "6: PWM1 Event Line 0"]
    TRGSEL6 = 6,
    #[doc = "7: PWM1 Event Line 1"]
    TRGSEL7 = 7,
}
impl From<TRGSEL0SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL0SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRGSEL0SELECT_A {
    type Ux = u8;
}
impl TRGSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL0SELECT_A {
        match self.bits {
            0 => TRGSEL0SELECT_A::TRGSEL0,
            1 => TRGSEL0SELECT_A::TRGSEL1,
            2 => TRGSEL0SELECT_A::TRGSEL2,
            3 => TRGSEL0SELECT_A::TRGSEL3,
            4 => TRGSEL0SELECT_A::TRGSEL4,
            5 => TRGSEL0SELECT_A::TRGSEL5,
            6 => TRGSEL0SELECT_A::TRGSEL6,
            7 => TRGSEL0SELECT_A::TRGSEL7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRGSEL0`"]
    #[inline(always)]
    pub fn is_trgsel0(&self) -> bool {
        *self == TRGSEL0SELECT_A::TRGSEL0
    }
    #[doc = "Checks if the value of the field is `TRGSEL1`"]
    #[inline(always)]
    pub fn is_trgsel1(&self) -> bool {
        *self == TRGSEL0SELECT_A::TRGSEL1
    }
    #[doc = "Checks if the value of the field is `TRGSEL2`"]
    #[inline(always)]
    pub fn is_trgsel2(&self) -> bool {
        *self == TRGSEL0SELECT_A::TRGSEL2
    }
    #[doc = "Checks if the value of the field is `TRGSEL3`"]
    #[inline(always)]
    pub fn is_trgsel3(&self) -> bool {
        *self == TRGSEL0SELECT_A::TRGSEL3
    }
    #[doc = "Checks if the value of the field is `TRGSEL4`"]
    #[inline(always)]
    pub fn is_trgsel4(&self) -> bool {
        *self == TRGSEL0SELECT_A::TRGSEL4
    }
    #[doc = "Checks if the value of the field is `TRGSEL5`"]
    #[inline(always)]
    pub fn is_trgsel5(&self) -> bool {
        *self == TRGSEL0SELECT_A::TRGSEL5
    }
    #[doc = "Checks if the value of the field is `TRGSEL6`"]
    #[inline(always)]
    pub fn is_trgsel6(&self) -> bool {
        *self == TRGSEL0SELECT_A::TRGSEL6
    }
    #[doc = "Checks if the value of the field is `TRGSEL7`"]
    #[inline(always)]
    pub fn is_trgsel7(&self) -> bool {
        *self == TRGSEL0SELECT_A::TRGSEL7
    }
}
#[doc = "Field `TRGSEL0` writer - Trigger Selection of Channel 0"]
pub type TRGSEL0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, TRIGR_SPEC, 3, O, TRGSEL0SELECT_A>;
impl<'a, const O: u8> TRGSEL0_W<'a, O> {
    #[doc = "DAC External Trigger Input (DATRG)"]
    #[inline(always)]
    pub fn trgsel0(self) -> &'a mut W {
        self.variant(TRGSEL0SELECT_A::TRGSEL0)
    }
    #[doc = "TC0 Channel 0 Output (TIOA0)"]
    #[inline(always)]
    pub fn trgsel1(self) -> &'a mut W {
        self.variant(TRGSEL0SELECT_A::TRGSEL1)
    }
    #[doc = "TC0 Channel 1 Output (TIOA1)"]
    #[inline(always)]
    pub fn trgsel2(self) -> &'a mut W {
        self.variant(TRGSEL0SELECT_A::TRGSEL2)
    }
    #[doc = "TC0 Channel 2 Output (TIOA2)"]
    #[inline(always)]
    pub fn trgsel3(self) -> &'a mut W {
        self.variant(TRGSEL0SELECT_A::TRGSEL3)
    }
    #[doc = "PWM0 Event Line 0"]
    #[inline(always)]
    pub fn trgsel4(self) -> &'a mut W {
        self.variant(TRGSEL0SELECT_A::TRGSEL4)
    }
    #[doc = "PWM0 Event Line 1"]
    #[inline(always)]
    pub fn trgsel5(self) -> &'a mut W {
        self.variant(TRGSEL0SELECT_A::TRGSEL5)
    }
    #[doc = "PWM1 Event Line 0"]
    #[inline(always)]
    pub fn trgsel6(self) -> &'a mut W {
        self.variant(TRGSEL0SELECT_A::TRGSEL6)
    }
    #[doc = "PWM1 Event Line 1"]
    #[inline(always)]
    pub fn trgsel7(self) -> &'a mut W {
        self.variant(TRGSEL0SELECT_A::TRGSEL7)
    }
}
#[doc = "Field `TRGSEL1` reader - Trigger Selection of Channel 1"]
pub type TRGSEL1_R = crate::FieldReader<TRGSEL1SELECT_A>;
#[doc = "Trigger Selection of Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGSEL1SELECT_A {
    #[doc = "0: DAC External Trigger Input (DATRG)"]
    TRGSEL0 = 0,
    #[doc = "1: TC0 Channel 0 Output (TIOA0)"]
    TRGSEL1 = 1,
    #[doc = "2: TC0 Channel 1 Output (TIOA1)"]
    TRGSEL2 = 2,
    #[doc = "3: TC0 Channel 2 Output (TIOA2)"]
    TRGSEL3 = 3,
    #[doc = "4: PWM0 Event Line 0"]
    TRGSEL4 = 4,
    #[doc = "5: PWM0 Event Line 1"]
    TRGSEL5 = 5,
    #[doc = "6: PWM1 Event Line 0"]
    TRGSEL6 = 6,
    #[doc = "7: PWM1 Event Line 1"]
    TRGSEL7 = 7,
}
impl From<TRGSEL1SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL1SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRGSEL1SELECT_A {
    type Ux = u8;
}
impl TRGSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL1SELECT_A {
        match self.bits {
            0 => TRGSEL1SELECT_A::TRGSEL0,
            1 => TRGSEL1SELECT_A::TRGSEL1,
            2 => TRGSEL1SELECT_A::TRGSEL2,
            3 => TRGSEL1SELECT_A::TRGSEL3,
            4 => TRGSEL1SELECT_A::TRGSEL4,
            5 => TRGSEL1SELECT_A::TRGSEL5,
            6 => TRGSEL1SELECT_A::TRGSEL6,
            7 => TRGSEL1SELECT_A::TRGSEL7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRGSEL0`"]
    #[inline(always)]
    pub fn is_trgsel0(&self) -> bool {
        *self == TRGSEL1SELECT_A::TRGSEL0
    }
    #[doc = "Checks if the value of the field is `TRGSEL1`"]
    #[inline(always)]
    pub fn is_trgsel1(&self) -> bool {
        *self == TRGSEL1SELECT_A::TRGSEL1
    }
    #[doc = "Checks if the value of the field is `TRGSEL2`"]
    #[inline(always)]
    pub fn is_trgsel2(&self) -> bool {
        *self == TRGSEL1SELECT_A::TRGSEL2
    }
    #[doc = "Checks if the value of the field is `TRGSEL3`"]
    #[inline(always)]
    pub fn is_trgsel3(&self) -> bool {
        *self == TRGSEL1SELECT_A::TRGSEL3
    }
    #[doc = "Checks if the value of the field is `TRGSEL4`"]
    #[inline(always)]
    pub fn is_trgsel4(&self) -> bool {
        *self == TRGSEL1SELECT_A::TRGSEL4
    }
    #[doc = "Checks if the value of the field is `TRGSEL5`"]
    #[inline(always)]
    pub fn is_trgsel5(&self) -> bool {
        *self == TRGSEL1SELECT_A::TRGSEL5
    }
    #[doc = "Checks if the value of the field is `TRGSEL6`"]
    #[inline(always)]
    pub fn is_trgsel6(&self) -> bool {
        *self == TRGSEL1SELECT_A::TRGSEL6
    }
    #[doc = "Checks if the value of the field is `TRGSEL7`"]
    #[inline(always)]
    pub fn is_trgsel7(&self) -> bool {
        *self == TRGSEL1SELECT_A::TRGSEL7
    }
}
#[doc = "Field `TRGSEL1` writer - Trigger Selection of Channel 1"]
pub type TRGSEL1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, TRIGR_SPEC, 3, O, TRGSEL1SELECT_A>;
impl<'a, const O: u8> TRGSEL1_W<'a, O> {
    #[doc = "DAC External Trigger Input (DATRG)"]
    #[inline(always)]
    pub fn trgsel0(self) -> &'a mut W {
        self.variant(TRGSEL1SELECT_A::TRGSEL0)
    }
    #[doc = "TC0 Channel 0 Output (TIOA0)"]
    #[inline(always)]
    pub fn trgsel1(self) -> &'a mut W {
        self.variant(TRGSEL1SELECT_A::TRGSEL1)
    }
    #[doc = "TC0 Channel 1 Output (TIOA1)"]
    #[inline(always)]
    pub fn trgsel2(self) -> &'a mut W {
        self.variant(TRGSEL1SELECT_A::TRGSEL2)
    }
    #[doc = "TC0 Channel 2 Output (TIOA2)"]
    #[inline(always)]
    pub fn trgsel3(self) -> &'a mut W {
        self.variant(TRGSEL1SELECT_A::TRGSEL3)
    }
    #[doc = "PWM0 Event Line 0"]
    #[inline(always)]
    pub fn trgsel4(self) -> &'a mut W {
        self.variant(TRGSEL1SELECT_A::TRGSEL4)
    }
    #[doc = "PWM0 Event Line 1"]
    #[inline(always)]
    pub fn trgsel5(self) -> &'a mut W {
        self.variant(TRGSEL1SELECT_A::TRGSEL5)
    }
    #[doc = "PWM1 Event Line 0"]
    #[inline(always)]
    pub fn trgsel6(self) -> &'a mut W {
        self.variant(TRGSEL1SELECT_A::TRGSEL6)
    }
    #[doc = "PWM1 Event Line 1"]
    #[inline(always)]
    pub fn trgsel7(self) -> &'a mut W {
        self.variant(TRGSEL1SELECT_A::TRGSEL7)
    }
}
#[doc = "Field `OSR0` reader - Over Sampling Ratio of Channel 0"]
pub type OSR0_R = crate::FieldReader<OSR0SELECT_A>;
#[doc = "Over Sampling Ratio of Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSR0SELECT_A {
    #[doc = "0: OSR = 1"]
    OSR_1 = 0,
    #[doc = "1: OSR = 2"]
    OSR_2 = 1,
    #[doc = "2: OSR = 4"]
    OSR_4 = 2,
    #[doc = "3: OSR = 8"]
    OSR_8 = 3,
    #[doc = "4: OSR = 16"]
    OSR_16 = 4,
    #[doc = "5: OSR = 32"]
    OSR_32 = 5,
}
impl From<OSR0SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: OSR0SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSR0SELECT_A {
    type Ux = u8;
}
impl OSR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSR0SELECT_A> {
        match self.bits {
            0 => Some(OSR0SELECT_A::OSR_1),
            1 => Some(OSR0SELECT_A::OSR_2),
            2 => Some(OSR0SELECT_A::OSR_4),
            3 => Some(OSR0SELECT_A::OSR_8),
            4 => Some(OSR0SELECT_A::OSR_16),
            5 => Some(OSR0SELECT_A::OSR_32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OSR_1`"]
    #[inline(always)]
    pub fn is_osr_1(&self) -> bool {
        *self == OSR0SELECT_A::OSR_1
    }
    #[doc = "Checks if the value of the field is `OSR_2`"]
    #[inline(always)]
    pub fn is_osr_2(&self) -> bool {
        *self == OSR0SELECT_A::OSR_2
    }
    #[doc = "Checks if the value of the field is `OSR_4`"]
    #[inline(always)]
    pub fn is_osr_4(&self) -> bool {
        *self == OSR0SELECT_A::OSR_4
    }
    #[doc = "Checks if the value of the field is `OSR_8`"]
    #[inline(always)]
    pub fn is_osr_8(&self) -> bool {
        *self == OSR0SELECT_A::OSR_8
    }
    #[doc = "Checks if the value of the field is `OSR_16`"]
    #[inline(always)]
    pub fn is_osr_16(&self) -> bool {
        *self == OSR0SELECT_A::OSR_16
    }
    #[doc = "Checks if the value of the field is `OSR_32`"]
    #[inline(always)]
    pub fn is_osr_32(&self) -> bool {
        *self == OSR0SELECT_A::OSR_32
    }
}
#[doc = "Field `OSR0` writer - Over Sampling Ratio of Channel 0"]
pub type OSR0_W<'a, const O: u8> = crate::FieldWriter<'a, TRIGR_SPEC, 3, O, OSR0SELECT_A>;
impl<'a, const O: u8> OSR0_W<'a, O> {
    #[doc = "OSR = 1"]
    #[inline(always)]
    pub fn osr_1(self) -> &'a mut W {
        self.variant(OSR0SELECT_A::OSR_1)
    }
    #[doc = "OSR = 2"]
    #[inline(always)]
    pub fn osr_2(self) -> &'a mut W {
        self.variant(OSR0SELECT_A::OSR_2)
    }
    #[doc = "OSR = 4"]
    #[inline(always)]
    pub fn osr_4(self) -> &'a mut W {
        self.variant(OSR0SELECT_A::OSR_4)
    }
    #[doc = "OSR = 8"]
    #[inline(always)]
    pub fn osr_8(self) -> &'a mut W {
        self.variant(OSR0SELECT_A::OSR_8)
    }
    #[doc = "OSR = 16"]
    #[inline(always)]
    pub fn osr_16(self) -> &'a mut W {
        self.variant(OSR0SELECT_A::OSR_16)
    }
    #[doc = "OSR = 32"]
    #[inline(always)]
    pub fn osr_32(self) -> &'a mut W {
        self.variant(OSR0SELECT_A::OSR_32)
    }
}
#[doc = "Field `OSR1` reader - Over Sampling Ratio of Channel 1"]
pub type OSR1_R = crate::FieldReader<OSR1SELECT_A>;
#[doc = "Over Sampling Ratio of Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSR1SELECT_A {
    #[doc = "0: OSR = 1"]
    OSR_1 = 0,
    #[doc = "1: OSR = 2"]
    OSR_2 = 1,
    #[doc = "2: OSR = 4"]
    OSR_4 = 2,
    #[doc = "3: OSR = 8"]
    OSR_8 = 3,
    #[doc = "4: OSR = 16"]
    OSR_16 = 4,
    #[doc = "5: OSR = 32"]
    OSR_32 = 5,
}
impl From<OSR1SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: OSR1SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSR1SELECT_A {
    type Ux = u8;
}
impl OSR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSR1SELECT_A> {
        match self.bits {
            0 => Some(OSR1SELECT_A::OSR_1),
            1 => Some(OSR1SELECT_A::OSR_2),
            2 => Some(OSR1SELECT_A::OSR_4),
            3 => Some(OSR1SELECT_A::OSR_8),
            4 => Some(OSR1SELECT_A::OSR_16),
            5 => Some(OSR1SELECT_A::OSR_32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OSR_1`"]
    #[inline(always)]
    pub fn is_osr_1(&self) -> bool {
        *self == OSR1SELECT_A::OSR_1
    }
    #[doc = "Checks if the value of the field is `OSR_2`"]
    #[inline(always)]
    pub fn is_osr_2(&self) -> bool {
        *self == OSR1SELECT_A::OSR_2
    }
    #[doc = "Checks if the value of the field is `OSR_4`"]
    #[inline(always)]
    pub fn is_osr_4(&self) -> bool {
        *self == OSR1SELECT_A::OSR_4
    }
    #[doc = "Checks if the value of the field is `OSR_8`"]
    #[inline(always)]
    pub fn is_osr_8(&self) -> bool {
        *self == OSR1SELECT_A::OSR_8
    }
    #[doc = "Checks if the value of the field is `OSR_16`"]
    #[inline(always)]
    pub fn is_osr_16(&self) -> bool {
        *self == OSR1SELECT_A::OSR_16
    }
    #[doc = "Checks if the value of the field is `OSR_32`"]
    #[inline(always)]
    pub fn is_osr_32(&self) -> bool {
        *self == OSR1SELECT_A::OSR_32
    }
}
#[doc = "Field `OSR1` writer - Over Sampling Ratio of Channel 1"]
pub type OSR1_W<'a, const O: u8> = crate::FieldWriter<'a, TRIGR_SPEC, 3, O, OSR1SELECT_A>;
impl<'a, const O: u8> OSR1_W<'a, O> {
    #[doc = "OSR = 1"]
    #[inline(always)]
    pub fn osr_1(self) -> &'a mut W {
        self.variant(OSR1SELECT_A::OSR_1)
    }
    #[doc = "OSR = 2"]
    #[inline(always)]
    pub fn osr_2(self) -> &'a mut W {
        self.variant(OSR1SELECT_A::OSR_2)
    }
    #[doc = "OSR = 4"]
    #[inline(always)]
    pub fn osr_4(self) -> &'a mut W {
        self.variant(OSR1SELECT_A::OSR_4)
    }
    #[doc = "OSR = 8"]
    #[inline(always)]
    pub fn osr_8(self) -> &'a mut W {
        self.variant(OSR1SELECT_A::OSR_8)
    }
    #[doc = "OSR = 16"]
    #[inline(always)]
    pub fn osr_16(self) -> &'a mut W {
        self.variant(OSR1SELECT_A::OSR_16)
    }
    #[doc = "OSR = 32"]
    #[inline(always)]
    pub fn osr_32(self) -> &'a mut W {
        self.variant(OSR1SELECT_A::OSR_32)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable of Channel 0"]
    #[inline(always)]
    pub fn trgen0(&self) -> TRGEN0_R {
        TRGEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger Enable of Channel 1"]
    #[inline(always)]
    pub fn trgen1(&self) -> TRGEN1_R {
        TRGEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger Selection of Channel 0"]
    #[inline(always)]
    pub fn trgsel0(&self) -> TRGSEL0_R {
        TRGSEL0_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Trigger Selection of Channel 1"]
    #[inline(always)]
    pub fn trgsel1(&self) -> TRGSEL1_R {
        TRGSEL1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Over Sampling Ratio of Channel 0"]
    #[inline(always)]
    pub fn osr0(&self) -> OSR0_R {
        OSR0_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Over Sampling Ratio of Channel 1"]
    #[inline(always)]
    pub fn osr1(&self) -> OSR1_R {
        OSR1_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable of Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn trgen0(&mut self) -> TRGEN0_W<0> {
        TRGEN0_W::new(self)
    }
    #[doc = "Bit 1 - Trigger Enable of Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn trgen1(&mut self) -> TRGEN1_W<1> {
        TRGEN1_W::new(self)
    }
    #[doc = "Bits 4:6 - Trigger Selection of Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel0(&mut self) -> TRGSEL0_W<4> {
        TRGSEL0_W::new(self)
    }
    #[doc = "Bits 8:10 - Trigger Selection of Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel1(&mut self) -> TRGSEL1_W<8> {
        TRGSEL1_W::new(self)
    }
    #[doc = "Bits 16:18 - Over Sampling Ratio of Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn osr0(&mut self) -> OSR0_W<16> {
        OSR0_W::new(self)
    }
    #[doc = "Bits 20:22 - Over Sampling Ratio of Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn osr1(&mut self) -> OSR1_W<20> {
        OSR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigr](index.html) module"]
pub struct TRIGR_SPEC;
impl crate::RegisterSpec for TRIGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigr::R](R) reader structure"]
impl crate::Readable for TRIGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trigr::W](W) writer structure"]
impl crate::Writable for TRIGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIGR to value 0"]
impl crate::Resettable for TRIGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
