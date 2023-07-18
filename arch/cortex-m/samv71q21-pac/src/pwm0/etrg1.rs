#[doc = "Register `ETRG1` reader"]
pub struct R(crate::R<ETRG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETRG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETRG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETRG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETRG1` writer"]
pub struct W(crate::W<ETRG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETRG1_SPEC>;
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
impl From<crate::W<ETRG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETRG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXCNT` reader - Maximum Counter value"]
pub type MAXCNT_R = crate::FieldReader<u32>;
#[doc = "Field `MAXCNT` writer - Maximum Counter value"]
pub type MAXCNT_W<'a, const O: u8> = crate::FieldWriter<'a, ETRG1_SPEC, 24, O, u32>;
#[doc = "Field `TRGMODE` reader - External Trigger Mode"]
pub type TRGMODE_R = crate::FieldReader<TRGMODESELECT_A>;
#[doc = "External Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGMODESELECT_A {
    #[doc = "0: External trigger is not enabled."]
    OFF = 0,
    #[doc = "1: External PWM Reset Mode"]
    MODE1 = 1,
    #[doc = "2: External PWM Start Mode"]
    MODE2 = 2,
    #[doc = "3: Cycle-by-cycle Duty Mode"]
    MODE3 = 3,
}
impl From<TRGMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRGMODESELECT_A {
    type Ux = u8;
}
impl TRGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGMODESELECT_A {
        match self.bits {
            0 => TRGMODESELECT_A::OFF,
            1 => TRGMODESELECT_A::MODE1,
            2 => TRGMODESELECT_A::MODE2,
            3 => TRGMODESELECT_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == TRGMODESELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == TRGMODESELECT_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == TRGMODESELECT_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == TRGMODESELECT_A::MODE3
    }
}
#[doc = "Field `TRGMODE` writer - External Trigger Mode"]
pub type TRGMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, ETRG1_SPEC, 2, O, TRGMODESELECT_A>;
impl<'a, const O: u8> TRGMODE_W<'a, O> {
    #[doc = "External trigger is not enabled."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(TRGMODESELECT_A::OFF)
    }
    #[doc = "External PWM Reset Mode"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(TRGMODESELECT_A::MODE1)
    }
    #[doc = "External PWM Start Mode"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(TRGMODESELECT_A::MODE2)
    }
    #[doc = "Cycle-by-cycle Duty Mode"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(TRGMODESELECT_A::MODE3)
    }
}
#[doc = "Field `TRGEDGE` reader - Edge Selection"]
pub type TRGEDGE_R = crate::BitReader<TRGEDGESELECT_A>;
#[doc = "Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGEDGESELECT_A {
    #[doc = "0: TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    FALLING_ZERO = 0,
    #[doc = "1: TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    RISING_ONE = 1,
}
impl From<TRGEDGESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEDGESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEDGESELECT_A {
        match self.bits {
            false => TRGEDGESELECT_A::FALLING_ZERO,
            true => TRGEDGESELECT_A::RISING_ONE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_ZERO`"]
    #[inline(always)]
    pub fn is_falling_zero(&self) -> bool {
        *self == TRGEDGESELECT_A::FALLING_ZERO
    }
    #[doc = "Checks if the value of the field is `RISING_ONE`"]
    #[inline(always)]
    pub fn is_rising_one(&self) -> bool {
        *self == TRGEDGESELECT_A::RISING_ONE
    }
}
#[doc = "Field `TRGEDGE` writer - Edge Selection"]
pub type TRGEDGE_W<'a, const O: u8> = crate::BitWriter<'a, ETRG1_SPEC, O, TRGEDGESELECT_A>;
impl<'a, const O: u8> TRGEDGE_W<'a, O> {
    #[doc = "TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    #[inline(always)]
    pub fn falling_zero(self) -> &'a mut W {
        self.variant(TRGEDGESELECT_A::FALLING_ZERO)
    }
    #[doc = "TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    #[inline(always)]
    pub fn rising_one(self) -> &'a mut W {
        self.variant(TRGEDGESELECT_A::RISING_ONE)
    }
}
#[doc = "Field `TRGFILT` reader - Filtered input"]
pub type TRGFILT_R = crate::BitReader;
#[doc = "Field `TRGFILT` writer - Filtered input"]
pub type TRGFILT_W<'a, const O: u8> = crate::BitWriter<'a, ETRG1_SPEC, O>;
#[doc = "Field `TRGSRC` reader - Trigger Source"]
pub type TRGSRC_R = crate::BitReader;
#[doc = "Field `TRGSRC` writer - Trigger Source"]
pub type TRGSRC_W<'a, const O: u8> = crate::BitWriter<'a, ETRG1_SPEC, O>;
#[doc = "Field `RFEN` reader - Recoverable Fault Enable"]
pub type RFEN_R = crate::BitReader;
#[doc = "Field `RFEN` writer - Recoverable Fault Enable"]
pub type RFEN_W<'a, const O: u8> = crate::BitWriter<'a, ETRG1_SPEC, O>;
impl R {
    #[doc = "Bits 0:23 - Maximum Counter value"]
    #[inline(always)]
    pub fn maxcnt(&self) -> MAXCNT_R {
        MAXCNT_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25 - External Trigger Mode"]
    #[inline(always)]
    pub fn trgmode(&self) -> TRGMODE_R {
        TRGMODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Edge Selection"]
    #[inline(always)]
    pub fn trgedge(&self) -> TRGEDGE_R {
        TRGEDGE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Filtered input"]
    #[inline(always)]
    pub fn trgfilt(&self) -> TRGFILT_R {
        TRGFILT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Recoverable Fault Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Maximum Counter value"]
    #[inline(always)]
    #[must_use]
    pub fn maxcnt(&mut self) -> MAXCNT_W<0> {
        MAXCNT_W::new(self)
    }
    #[doc = "Bits 24:25 - External Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn trgmode(&mut self) -> TRGMODE_W<24> {
        TRGMODE_W::new(self)
    }
    #[doc = "Bit 28 - Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgedge(&mut self) -> TRGEDGE_W<28> {
        TRGEDGE_W::new(self)
    }
    #[doc = "Bit 29 - Filtered input"]
    #[inline(always)]
    #[must_use]
    pub fn trgfilt(&mut self) -> TRGFILT_W<29> {
        TRGFILT_W::new(self)
    }
    #[doc = "Bit 30 - Trigger Source"]
    #[inline(always)]
    #[must_use]
    pub fn trgsrc(&mut self) -> TRGSRC_W<30> {
        TRGSRC_W::new(self)
    }
    #[doc = "Bit 31 - Recoverable Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfen(&mut self) -> RFEN_W<31> {
        RFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM External Trigger Register (trg_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etrg1](index.html) module"]
pub struct ETRG1_SPEC;
impl crate::RegisterSpec for ETRG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etrg1::R](R) reader structure"]
impl crate::Readable for ETRG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etrg1::W](W) writer structure"]
impl crate::Writable for ETRG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETRG1 to value 0"]
impl crate::Resettable for ETRG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
