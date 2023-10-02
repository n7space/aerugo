#[doc = "Register `ETRG2` reader"]
pub type R = crate::R<ETRG2_SPEC>;
#[doc = "Register `ETRG2` writer"]
pub type W = crate::W<ETRG2_SPEC>;
#[doc = "Field `MAXCNT` reader - Maximum Counter value"]
pub type MAXCNT_R = crate::FieldReader<u32>;
#[doc = "Field `MAXCNT` writer - Maximum Counter value"]
pub type MAXCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
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
    #[doc = "External trigger is not enabled."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == TRGMODESELECT_A::OFF
    }
    #[doc = "External PWM Reset Mode"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == TRGMODESELECT_A::MODE1
    }
    #[doc = "External PWM Start Mode"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == TRGMODESELECT_A::MODE2
    }
    #[doc = "Cycle-by-cycle Duty Mode"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == TRGMODESELECT_A::MODE3
    }
}
#[doc = "Field `TRGMODE` writer - External Trigger Mode"]
pub type TRGMODE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, TRGMODESELECT_A>;
impl<'a, REG, const O: u8> TRGMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External trigger is not enabled."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(TRGMODESELECT_A::OFF)
    }
    #[doc = "External PWM Reset Mode"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(TRGMODESELECT_A::MODE1)
    }
    #[doc = "External PWM Start Mode"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(TRGMODESELECT_A::MODE2)
    }
    #[doc = "Cycle-by-cycle Duty Mode"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
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
    #[doc = "TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    #[inline(always)]
    pub fn is_falling_zero(&self) -> bool {
        *self == TRGEDGESELECT_A::FALLING_ZERO
    }
    #[doc = "TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    #[inline(always)]
    pub fn is_rising_one(&self) -> bool {
        *self == TRGEDGESELECT_A::RISING_ONE
    }
}
#[doc = "Field `TRGEDGE` writer - Edge Selection"]
pub type TRGEDGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRGEDGESELECT_A>;
impl<'a, REG, const O: u8> TRGEDGE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    #[inline(always)]
    pub fn falling_zero(self) -> &'a mut crate::W<REG> {
        self.variant(TRGEDGESELECT_A::FALLING_ZERO)
    }
    #[doc = "TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    #[inline(always)]
    pub fn rising_one(self) -> &'a mut crate::W<REG> {
        self.variant(TRGEDGESELECT_A::RISING_ONE)
    }
}
#[doc = "Field `TRGFILT` reader - Filtered input"]
pub type TRGFILT_R = crate::BitReader;
#[doc = "Field `TRGFILT` writer - Filtered input"]
pub type TRGFILT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRGSRC` reader - Trigger Source"]
pub type TRGSRC_R = crate::BitReader;
#[doc = "Field `TRGSRC` writer - Trigger Source"]
pub type TRGSRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFEN` reader - Recoverable Fault Enable"]
pub type RFEN_R = crate::BitReader;
#[doc = "Field `RFEN` writer - Recoverable Fault Enable"]
pub type RFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn maxcnt(&mut self) -> MAXCNT_W<ETRG2_SPEC, 0> {
        MAXCNT_W::new(self)
    }
    #[doc = "Bits 24:25 - External Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn trgmode(&mut self) -> TRGMODE_W<ETRG2_SPEC, 24> {
        TRGMODE_W::new(self)
    }
    #[doc = "Bit 28 - Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgedge(&mut self) -> TRGEDGE_W<ETRG2_SPEC, 28> {
        TRGEDGE_W::new(self)
    }
    #[doc = "Bit 29 - Filtered input"]
    #[inline(always)]
    #[must_use]
    pub fn trgfilt(&mut self) -> TRGFILT_W<ETRG2_SPEC, 29> {
        TRGFILT_W::new(self)
    }
    #[doc = "Bit 30 - Trigger Source"]
    #[inline(always)]
    #[must_use]
    pub fn trgsrc(&mut self) -> TRGSRC_W<ETRG2_SPEC, 30> {
        TRGSRC_W::new(self)
    }
    #[doc = "Bit 31 - Recoverable Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfen(&mut self) -> RFEN_W<ETRG2_SPEC, 31> {
        RFEN_W::new(self)
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
#[doc = "PWM External Trigger Register (trg_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etrg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etrg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETRG2_SPEC;
impl crate::RegisterSpec for ETRG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etrg2::R`](R) reader structure"]
impl crate::Readable for ETRG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etrg2::W`](W) writer structure"]
impl crate::Writable for ETRG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETRG2 to value 0"]
impl crate::Resettable for ETRG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
