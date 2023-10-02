#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub type TRGEN_R = crate::BitReader<TRGENSELECT_A>;
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGENSELECT_A {
    #[doc = "0: Hardware triggers are disabled. Starting a conversion is only possible by software."]
    DIS = 0,
    #[doc = "1: Hardware trigger selected by TRGSEL field is enabled."]
    EN = 1,
}
impl From<TRGENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TRGENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGENSELECT_A {
        match self.bits {
            false => TRGENSELECT_A::DIS,
            true => TRGENSELECT_A::EN,
        }
    }
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRGENSELECT_A::DIS
    }
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRGENSELECT_A::EN
    }
}
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub type TRGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRGENSELECT_A>;
impl<'a, REG, const O: u8> TRGEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TRGENSELECT_A::DIS)
    }
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TRGENSELECT_A::EN)
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub type TRGSEL_R = crate::FieldReader<TRGSELSELECT_A>;
#[doc = "Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGSELSELECT_A {
    #[doc = "0: AFE0_ADTRG for AFEC0 / AFE1_ADTRG for AFEC1"]
    AFEC_TRIG0 = 0,
    #[doc = "1: TIOA Output of the Timer Counter Channel 0 for AFEC0/TIOA Output of the Timer Counter Channel 3 for AFEC1"]
    AFEC_TRIG1 = 1,
    #[doc = "2: TIOA Output of the Timer Counter Channel 1 for AFEC0/TIOA Output of the Timer Counter Channel 4 for AFEC1"]
    AFEC_TRIG2 = 2,
    #[doc = "3: TIOA Output of the Timer Counter Channel 2 for AFEC0/TIOA Output of the Timer Counter Channel 5 for AFEC1"]
    AFEC_TRIG3 = 3,
    #[doc = "4: PWM0 event line 0 for AFEC0 / PWM1 event line 0 for AFEC1"]
    AFEC_TRIG4 = 4,
    #[doc = "5: PWM0 event line 1 for AFEC0 / PWM1 event line 1 for AFEC1"]
    AFEC_TRIG5 = 5,
    #[doc = "6: Analog Comparator"]
    AFEC_TRIG6 = 6,
}
impl From<TRGSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRGSELSELECT_A {
    type Ux = u8;
}
impl TRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRGSELSELECT_A> {
        match self.bits {
            0 => Some(TRGSELSELECT_A::AFEC_TRIG0),
            1 => Some(TRGSELSELECT_A::AFEC_TRIG1),
            2 => Some(TRGSELSELECT_A::AFEC_TRIG2),
            3 => Some(TRGSELSELECT_A::AFEC_TRIG3),
            4 => Some(TRGSELSELECT_A::AFEC_TRIG4),
            5 => Some(TRGSELSELECT_A::AFEC_TRIG5),
            6 => Some(TRGSELSELECT_A::AFEC_TRIG6),
            _ => None,
        }
    }
    #[doc = "AFE0_ADTRG for AFEC0 / AFE1_ADTRG for AFEC1"]
    #[inline(always)]
    pub fn is_afec_trig0(&self) -> bool {
        *self == TRGSELSELECT_A::AFEC_TRIG0
    }
    #[doc = "TIOA Output of the Timer Counter Channel 0 for AFEC0/TIOA Output of the Timer Counter Channel 3 for AFEC1"]
    #[inline(always)]
    pub fn is_afec_trig1(&self) -> bool {
        *self == TRGSELSELECT_A::AFEC_TRIG1
    }
    #[doc = "TIOA Output of the Timer Counter Channel 1 for AFEC0/TIOA Output of the Timer Counter Channel 4 for AFEC1"]
    #[inline(always)]
    pub fn is_afec_trig2(&self) -> bool {
        *self == TRGSELSELECT_A::AFEC_TRIG2
    }
    #[doc = "TIOA Output of the Timer Counter Channel 2 for AFEC0/TIOA Output of the Timer Counter Channel 5 for AFEC1"]
    #[inline(always)]
    pub fn is_afec_trig3(&self) -> bool {
        *self == TRGSELSELECT_A::AFEC_TRIG3
    }
    #[doc = "PWM0 event line 0 for AFEC0 / PWM1 event line 0 for AFEC1"]
    #[inline(always)]
    pub fn is_afec_trig4(&self) -> bool {
        *self == TRGSELSELECT_A::AFEC_TRIG4
    }
    #[doc = "PWM0 event line 1 for AFEC0 / PWM1 event line 1 for AFEC1"]
    #[inline(always)]
    pub fn is_afec_trig5(&self) -> bool {
        *self == TRGSELSELECT_A::AFEC_TRIG5
    }
    #[doc = "Analog Comparator"]
    #[inline(always)]
    pub fn is_afec_trig6(&self) -> bool {
        *self == TRGSELSELECT_A::AFEC_TRIG6
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub type TRGSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, TRGSELSELECT_A>;
impl<'a, REG, const O: u8> TRGSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AFE0_ADTRG for AFEC0 / AFE1_ADTRG for AFEC1"]
    #[inline(always)]
    pub fn afec_trig0(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSELSELECT_A::AFEC_TRIG0)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 0 for AFEC0/TIOA Output of the Timer Counter Channel 3 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig1(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSELSELECT_A::AFEC_TRIG1)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 1 for AFEC0/TIOA Output of the Timer Counter Channel 4 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig2(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSELSELECT_A::AFEC_TRIG2)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 2 for AFEC0/TIOA Output of the Timer Counter Channel 5 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig3(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSELSELECT_A::AFEC_TRIG3)
    }
    #[doc = "PWM0 event line 0 for AFEC0 / PWM1 event line 0 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig4(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSELSELECT_A::AFEC_TRIG4)
    }
    #[doc = "PWM0 event line 1 for AFEC0 / PWM1 event line 1 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig5(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSELSELECT_A::AFEC_TRIG5)
    }
    #[doc = "Analog Comparator"]
    #[inline(always)]
    pub fn afec_trig6(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSELSELECT_A::AFEC_TRIG6)
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode"]
pub type SLEEP_R = crate::BitReader<SLEEPSELECT_A>;
#[doc = "Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEPSELECT_A {
    #[doc = "0: Normal mode: The AFE and reference voltage circuitry are kept ON between conversions."]
    NORMAL = 0,
    #[doc = "1: Sleep mode: The AFE and reference voltage circuitry are OFF between conversions."]
    SLEEP = 1,
}
impl From<SLEEPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPSELECT_A {
        match self.bits {
            false => SLEEPSELECT_A::NORMAL,
            true => SLEEPSELECT_A::SLEEP,
        }
    }
    #[doc = "Normal mode: The AFE and reference voltage circuitry are kept ON between conversions."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SLEEPSELECT_A::NORMAL
    }
    #[doc = "Sleep mode: The AFE and reference voltage circuitry are OFF between conversions."]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == SLEEPSELECT_A::SLEEP
    }
}
#[doc = "Field `SLEEP` writer - Sleep Mode"]
pub type SLEEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLEEPSELECT_A>;
impl<'a, REG, const O: u8> SLEEP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode: The AFE and reference voltage circuitry are kept ON between conversions."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEPSELECT_A::NORMAL)
    }
    #[doc = "Sleep mode: The AFE and reference voltage circuitry are OFF between conversions."]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEPSELECT_A::SLEEP)
    }
}
#[doc = "Field `FWUP` reader - Fast Wake-up"]
pub type FWUP_R = crate::BitReader<FWUPSELECT_A>;
#[doc = "Fast Wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWUPSELECT_A {
    #[doc = "0: Normal Sleep mode: The sleep mode is defined by the SLEEP bit."]
    OFF = 0,
    #[doc = "1: Fast wake-up Sleep mode: The voltage reference is ON between conversions and AFE is OFF."]
    ON = 1,
}
impl From<FWUPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FWUPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FWUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWUPSELECT_A {
        match self.bits {
            false => FWUPSELECT_A::OFF,
            true => FWUPSELECT_A::ON,
        }
    }
    #[doc = "Normal Sleep mode: The sleep mode is defined by the SLEEP bit."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FWUPSELECT_A::OFF
    }
    #[doc = "Fast wake-up Sleep mode: The voltage reference is ON between conversions and AFE is OFF."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FWUPSELECT_A::ON
    }
}
#[doc = "Field `FWUP` writer - Fast Wake-up"]
pub type FWUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FWUPSELECT_A>;
impl<'a, REG, const O: u8> FWUP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Sleep mode: The sleep mode is defined by the SLEEP bit."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(FWUPSELECT_A::OFF)
    }
    #[doc = "Fast wake-up Sleep mode: The voltage reference is ON between conversions and AFE is OFF."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(FWUPSELECT_A::ON)
    }
}
#[doc = "Field `FREERUN` reader - Free Run Mode"]
pub type FREERUN_R = crate::BitReader<FREERUNSELECT_A>;
#[doc = "Free Run Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREERUNSELECT_A {
    #[doc = "0: Normal mode"]
    OFF = 0,
    #[doc = "1: Free Run mode: Never wait for any trigger."]
    ON = 1,
}
impl From<FREERUNSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FREERUNSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FREERUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREERUNSELECT_A {
        match self.bits {
            false => FREERUNSELECT_A::OFF,
            true => FREERUNSELECT_A::ON,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FREERUNSELECT_A::OFF
    }
    #[doc = "Free Run mode: Never wait for any trigger."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FREERUNSELECT_A::ON
    }
}
#[doc = "Field `FREERUN` writer - Free Run Mode"]
pub type FREERUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FREERUNSELECT_A>;
impl<'a, REG, const O: u8> FREERUN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(FREERUNSELECT_A::OFF)
    }
    #[doc = "Free Run mode: Never wait for any trigger."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(FREERUNSELECT_A::ON)
    }
}
#[doc = "Field `PRESCAL` reader - Prescaler Rate Selection"]
pub type PRESCAL_R = crate::FieldReader;
#[doc = "Field `PRESCAL` writer - Prescaler Rate Selection"]
pub type PRESCAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `STARTUP` reader - Start-up Time"]
pub type STARTUP_R = crate::FieldReader<STARTUPSELECT_A>;
#[doc = "Start-up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STARTUPSELECT_A {
    #[doc = "0: 0 periods of AFE clock"]
    SUT0 = 0,
    #[doc = "1: 8 periods of AFE clock"]
    SUT8 = 1,
    #[doc = "2: 16 periods of AFE clock"]
    SUT16 = 2,
    #[doc = "3: 24 periods of AFE clock"]
    SUT24 = 3,
    #[doc = "4: 64 periods of AFE clock"]
    SUT64 = 4,
    #[doc = "5: 80 periods of AFE clock"]
    SUT80 = 5,
    #[doc = "6: 96 periods of AFE clock"]
    SUT96 = 6,
    #[doc = "7: 112 periods of AFE clock"]
    SUT112 = 7,
    #[doc = "8: 512 periods of AFE clock"]
    SUT512 = 8,
    #[doc = "9: 576 periods of AFE clock"]
    SUT576 = 9,
    #[doc = "10: 640 periods of AFE clock"]
    SUT640 = 10,
    #[doc = "11: 704 periods of AFE clock"]
    SUT704 = 11,
    #[doc = "12: 768 periods of AFE clock"]
    SUT768 = 12,
    #[doc = "13: 832 periods of AFE clock"]
    SUT832 = 13,
    #[doc = "14: 896 periods of AFE clock"]
    SUT896 = 14,
    #[doc = "15: 960 periods of AFE clock"]
    SUT960 = 15,
}
impl From<STARTUPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STARTUPSELECT_A {
    type Ux = u8;
}
impl STARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTUPSELECT_A {
        match self.bits {
            0 => STARTUPSELECT_A::SUT0,
            1 => STARTUPSELECT_A::SUT8,
            2 => STARTUPSELECT_A::SUT16,
            3 => STARTUPSELECT_A::SUT24,
            4 => STARTUPSELECT_A::SUT64,
            5 => STARTUPSELECT_A::SUT80,
            6 => STARTUPSELECT_A::SUT96,
            7 => STARTUPSELECT_A::SUT112,
            8 => STARTUPSELECT_A::SUT512,
            9 => STARTUPSELECT_A::SUT576,
            10 => STARTUPSELECT_A::SUT640,
            11 => STARTUPSELECT_A::SUT704,
            12 => STARTUPSELECT_A::SUT768,
            13 => STARTUPSELECT_A::SUT832,
            14 => STARTUPSELECT_A::SUT896,
            15 => STARTUPSELECT_A::SUT960,
            _ => unreachable!(),
        }
    }
    #[doc = "0 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut0(&self) -> bool {
        *self == STARTUPSELECT_A::SUT0
    }
    #[doc = "8 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut8(&self) -> bool {
        *self == STARTUPSELECT_A::SUT8
    }
    #[doc = "16 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut16(&self) -> bool {
        *self == STARTUPSELECT_A::SUT16
    }
    #[doc = "24 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut24(&self) -> bool {
        *self == STARTUPSELECT_A::SUT24
    }
    #[doc = "64 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut64(&self) -> bool {
        *self == STARTUPSELECT_A::SUT64
    }
    #[doc = "80 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut80(&self) -> bool {
        *self == STARTUPSELECT_A::SUT80
    }
    #[doc = "96 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut96(&self) -> bool {
        *self == STARTUPSELECT_A::SUT96
    }
    #[doc = "112 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut112(&self) -> bool {
        *self == STARTUPSELECT_A::SUT112
    }
    #[doc = "512 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut512(&self) -> bool {
        *self == STARTUPSELECT_A::SUT512
    }
    #[doc = "576 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut576(&self) -> bool {
        *self == STARTUPSELECT_A::SUT576
    }
    #[doc = "640 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut640(&self) -> bool {
        *self == STARTUPSELECT_A::SUT640
    }
    #[doc = "704 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut704(&self) -> bool {
        *self == STARTUPSELECT_A::SUT704
    }
    #[doc = "768 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut768(&self) -> bool {
        *self == STARTUPSELECT_A::SUT768
    }
    #[doc = "832 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut832(&self) -> bool {
        *self == STARTUPSELECT_A::SUT832
    }
    #[doc = "896 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut896(&self) -> bool {
        *self == STARTUPSELECT_A::SUT896
    }
    #[doc = "960 periods of AFE clock"]
    #[inline(always)]
    pub fn is_sut960(&self) -> bool {
        *self == STARTUPSELECT_A::SUT960
    }
}
#[doc = "Field `STARTUP` writer - Start-up Time"]
pub type STARTUP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O, STARTUPSELECT_A>;
impl<'a, REG, const O: u8> STARTUP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 periods of AFE clock"]
    #[inline(always)]
    pub fn sut0(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT0)
    }
    #[doc = "8 periods of AFE clock"]
    #[inline(always)]
    pub fn sut8(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT8)
    }
    #[doc = "16 periods of AFE clock"]
    #[inline(always)]
    pub fn sut16(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT16)
    }
    #[doc = "24 periods of AFE clock"]
    #[inline(always)]
    pub fn sut24(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT24)
    }
    #[doc = "64 periods of AFE clock"]
    #[inline(always)]
    pub fn sut64(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT64)
    }
    #[doc = "80 periods of AFE clock"]
    #[inline(always)]
    pub fn sut80(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT80)
    }
    #[doc = "96 periods of AFE clock"]
    #[inline(always)]
    pub fn sut96(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT96)
    }
    #[doc = "112 periods of AFE clock"]
    #[inline(always)]
    pub fn sut112(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT112)
    }
    #[doc = "512 periods of AFE clock"]
    #[inline(always)]
    pub fn sut512(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT512)
    }
    #[doc = "576 periods of AFE clock"]
    #[inline(always)]
    pub fn sut576(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT576)
    }
    #[doc = "640 periods of AFE clock"]
    #[inline(always)]
    pub fn sut640(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT640)
    }
    #[doc = "704 periods of AFE clock"]
    #[inline(always)]
    pub fn sut704(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT704)
    }
    #[doc = "768 periods of AFE clock"]
    #[inline(always)]
    pub fn sut768(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT768)
    }
    #[doc = "832 periods of AFE clock"]
    #[inline(always)]
    pub fn sut832(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT832)
    }
    #[doc = "896 periods of AFE clock"]
    #[inline(always)]
    pub fn sut896(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT896)
    }
    #[doc = "960 periods of AFE clock"]
    #[inline(always)]
    pub fn sut960(self) -> &'a mut crate::W<REG> {
        self.variant(STARTUPSELECT_A::SUT960)
    }
}
#[doc = "Field `ONE` reader - One"]
pub type ONE_R = crate::BitReader;
#[doc = "Field `ONE` writer - One"]
pub type ONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRACKTIM` reader - Tracking Time"]
pub type TRACKTIM_R = crate::FieldReader;
#[doc = "Field `TRACKTIM` writer - Tracking Time"]
pub type TRACKTIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TRANSFER` reader - Transfer Period"]
pub type TRANSFER_R = crate::FieldReader;
#[doc = "Field `TRANSFER` writer - Transfer Period"]
pub type TRANSFER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `USEQ` reader - User Sequence Enable"]
pub type USEQ_R = crate::BitReader<USEQSELECT_A>;
#[doc = "User Sequence Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USEQSELECT_A {
    #[doc = "0: Normal mode: The controller converts channels in a simple numeric order."]
    NUM_ORDER = 0,
    #[doc = "1: User Sequence mode: The sequence respects what is defined in AFEC_SEQ1R and AFEC_SEQ1R."]
    REG_ORDER = 1,
}
impl From<USEQSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: USEQSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl USEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USEQSELECT_A {
        match self.bits {
            false => USEQSELECT_A::NUM_ORDER,
            true => USEQSELECT_A::REG_ORDER,
        }
    }
    #[doc = "Normal mode: The controller converts channels in a simple numeric order."]
    #[inline(always)]
    pub fn is_num_order(&self) -> bool {
        *self == USEQSELECT_A::NUM_ORDER
    }
    #[doc = "User Sequence mode: The sequence respects what is defined in AFEC_SEQ1R and AFEC_SEQ1R."]
    #[inline(always)]
    pub fn is_reg_order(&self) -> bool {
        *self == USEQSELECT_A::REG_ORDER
    }
}
#[doc = "Field `USEQ` writer - User Sequence Enable"]
pub type USEQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USEQSELECT_A>;
impl<'a, REG, const O: u8> USEQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode: The controller converts channels in a simple numeric order."]
    #[inline(always)]
    pub fn num_order(self) -> &'a mut crate::W<REG> {
        self.variant(USEQSELECT_A::NUM_ORDER)
    }
    #[doc = "User Sequence mode: The sequence respects what is defined in AFEC_SEQ1R and AFEC_SEQ1R."]
    #[inline(always)]
    pub fn reg_order(self) -> &'a mut crate::W<REG> {
        self.variant(USEQSELECT_A::REG_ORDER)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Wake-up"]
    #[inline(always)]
    pub fn fwup(&self) -> FWUP_R {
        FWUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&self) -> PRESCAL_R {
        PRESCAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Start-up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - One"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&self) -> TRACKTIM_R {
        TRACKTIM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline(always)]
    pub fn transfer(&self) -> TRANSFER_R {
        TRANSFER_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - User Sequence Enable"]
    #[inline(always)]
    pub fn useq(&self) -> USEQ_R {
        USEQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgen(&mut self) -> TRGEN_W<MR_SPEC, 0> {
        TRGEN_W::new(self)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TRGSEL_W<MR_SPEC, 1> {
        TRGSEL_W::new(self)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<MR_SPEC, 5> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 6 - Fast Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn fwup(&mut self) -> FWUP_W<MR_SPEC, 6> {
        FWUP_W::new(self)
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FREERUN_W<MR_SPEC, 7> {
        FREERUN_W::new(self)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    #[must_use]
    pub fn prescal(&mut self) -> PRESCAL_W<MR_SPEC, 8> {
        PRESCAL_W::new(self)
    }
    #[doc = "Bits 16:19 - Start-up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<MR_SPEC, 16> {
        STARTUP_W::new(self)
    }
    #[doc = "Bit 23 - One"]
    #[inline(always)]
    #[must_use]
    pub fn one(&mut self) -> ONE_W<MR_SPEC, 23> {
        ONE_W::new(self)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    #[must_use]
    pub fn tracktim(&mut self) -> TRACKTIM_W<MR_SPEC, 24> {
        TRACKTIM_W::new(self)
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline(always)]
    #[must_use]
    pub fn transfer(&mut self) -> TRANSFER_W<MR_SPEC, 28> {
        TRANSFER_W::new(self)
    }
    #[doc = "Bit 31 - User Sequence Enable"]
    #[inline(always)]
    #[must_use]
    pub fn useq(&mut self) -> USEQ_W<MR_SPEC, 31> {
        USEQ_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AFEC Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
