#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `UPDTIM` reader - Update Request Time Register"]
pub type UPDTIM_R = crate::BitReader;
#[doc = "Field `UPDTIM` writer - Update Request Time Register"]
pub type UPDTIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPDCAL` reader - Update Request Calendar Register"]
pub type UPDCAL_R = crate::BitReader;
#[doc = "Field `UPDCAL` writer - Update Request Calendar Register"]
pub type UPDCAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMEVSEL` reader - Time Event Selection"]
pub type TIMEVSEL_R = crate::FieldReader<TIMEVSELSELECT_A>;
#[doc = "Time Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEVSELSELECT_A {
    #[doc = "0: Minute change"]
    MINUTE = 0,
    #[doc = "1: Hour change"]
    HOUR = 1,
    #[doc = "2: Every day at midnight"]
    MIDNIGHT = 2,
    #[doc = "3: Every day at noon"]
    NOON = 3,
}
impl From<TIMEVSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEVSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMEVSELSELECT_A {
    type Ux = u8;
}
impl TIMEVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEVSELSELECT_A {
        match self.bits {
            0 => TIMEVSELSELECT_A::MINUTE,
            1 => TIMEVSELSELECT_A::HOUR,
            2 => TIMEVSELSELECT_A::MIDNIGHT,
            3 => TIMEVSELSELECT_A::NOON,
            _ => unreachable!(),
        }
    }
    #[doc = "Minute change"]
    #[inline(always)]
    pub fn is_minute(&self) -> bool {
        *self == TIMEVSELSELECT_A::MINUTE
    }
    #[doc = "Hour change"]
    #[inline(always)]
    pub fn is_hour(&self) -> bool {
        *self == TIMEVSELSELECT_A::HOUR
    }
    #[doc = "Every day at midnight"]
    #[inline(always)]
    pub fn is_midnight(&self) -> bool {
        *self == TIMEVSELSELECT_A::MIDNIGHT
    }
    #[doc = "Every day at noon"]
    #[inline(always)]
    pub fn is_noon(&self) -> bool {
        *self == TIMEVSELSELECT_A::NOON
    }
}
#[doc = "Field `TIMEVSEL` writer - Time Event Selection"]
pub type TIMEVSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, TIMEVSELSELECT_A>;
impl<'a, REG, const O: u8> TIMEVSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minute change"]
    #[inline(always)]
    pub fn minute(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEVSELSELECT_A::MINUTE)
    }
    #[doc = "Hour change"]
    #[inline(always)]
    pub fn hour(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEVSELSELECT_A::HOUR)
    }
    #[doc = "Every day at midnight"]
    #[inline(always)]
    pub fn midnight(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEVSELSELECT_A::MIDNIGHT)
    }
    #[doc = "Every day at noon"]
    #[inline(always)]
    pub fn noon(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEVSELSELECT_A::NOON)
    }
}
#[doc = "Field `CALEVSEL` reader - Calendar Event Selection"]
pub type CALEVSEL_R = crate::FieldReader<CALEVSELSELECT_A>;
#[doc = "Calendar Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALEVSELSELECT_A {
    #[doc = "0: Week change (every Monday at time 00:00:00)"]
    WEEK = 0,
    #[doc = "1: Month change (every 01 of each month at time 00:00:00)"]
    MONTH = 1,
    #[doc = "2: Year change (every January 1 at time 00:00:00)"]
    YEAR = 2,
}
impl From<CALEVSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CALEVSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CALEVSELSELECT_A {
    type Ux = u8;
}
impl CALEVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CALEVSELSELECT_A> {
        match self.bits {
            0 => Some(CALEVSELSELECT_A::WEEK),
            1 => Some(CALEVSELSELECT_A::MONTH),
            2 => Some(CALEVSELSELECT_A::YEAR),
            _ => None,
        }
    }
    #[doc = "Week change (every Monday at time 00:00:00)"]
    #[inline(always)]
    pub fn is_week(&self) -> bool {
        *self == CALEVSELSELECT_A::WEEK
    }
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    #[inline(always)]
    pub fn is_month(&self) -> bool {
        *self == CALEVSELSELECT_A::MONTH
    }
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    #[inline(always)]
    pub fn is_year(&self) -> bool {
        *self == CALEVSELSELECT_A::YEAR
    }
}
#[doc = "Field `CALEVSEL` writer - Calendar Event Selection"]
pub type CALEVSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CALEVSELSELECT_A>;
impl<'a, REG, const O: u8> CALEVSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Week change (every Monday at time 00:00:00)"]
    #[inline(always)]
    pub fn week(self) -> &'a mut crate::W<REG> {
        self.variant(CALEVSELSELECT_A::WEEK)
    }
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    #[inline(always)]
    pub fn month(self) -> &'a mut crate::W<REG> {
        self.variant(CALEVSELSELECT_A::MONTH)
    }
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    #[inline(always)]
    pub fn year(self) -> &'a mut crate::W<REG> {
        self.variant(CALEVSELSELECT_A::YEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    pub fn updtim(&self) -> UPDTIM_R {
        UPDTIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    pub fn updcal(&self) -> UPDCAL_R {
        UPDCAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    pub fn timevsel(&self) -> TIMEVSEL_R {
        TIMEVSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    pub fn calevsel(&self) -> CALEVSEL_R {
        CALEVSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    #[must_use]
    pub fn updtim(&mut self) -> UPDTIM_W<CR_SPEC, 0> {
        UPDTIM_W::new(self)
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    #[must_use]
    pub fn updcal(&mut self) -> UPDCAL_W<CR_SPEC, 1> {
        UPDCAL_W::new(self)
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn timevsel(&mut self) -> TIMEVSEL_W<CR_SPEC, 8> {
        TIMEVSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn calevsel(&mut self) -> CALEVSEL_W<CR_SPEC, 16> {
        CALEVSEL_W::new(self)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
