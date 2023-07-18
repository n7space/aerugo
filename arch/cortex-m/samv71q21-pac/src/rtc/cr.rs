#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDTIM` reader - Update Request Time Register"]
pub type UPDTIM_R = crate::BitReader;
#[doc = "Field `UPDTIM` writer - Update Request Time Register"]
pub type UPDTIM_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `UPDCAL` reader - Update Request Calendar Register"]
pub type UPDCAL_R = crate::BitReader;
#[doc = "Field `UPDCAL` writer - Update Request Calendar Register"]
pub type UPDCAL_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
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
    #[doc = "Checks if the value of the field is `MINUTE`"]
    #[inline(always)]
    pub fn is_minute(&self) -> bool {
        *self == TIMEVSELSELECT_A::MINUTE
    }
    #[doc = "Checks if the value of the field is `HOUR`"]
    #[inline(always)]
    pub fn is_hour(&self) -> bool {
        *self == TIMEVSELSELECT_A::HOUR
    }
    #[doc = "Checks if the value of the field is `MIDNIGHT`"]
    #[inline(always)]
    pub fn is_midnight(&self) -> bool {
        *self == TIMEVSELSELECT_A::MIDNIGHT
    }
    #[doc = "Checks if the value of the field is `NOON`"]
    #[inline(always)]
    pub fn is_noon(&self) -> bool {
        *self == TIMEVSELSELECT_A::NOON
    }
}
#[doc = "Field `TIMEVSEL` writer - Time Event Selection"]
pub type TIMEVSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR_SPEC, 2, O, TIMEVSELSELECT_A>;
impl<'a, const O: u8> TIMEVSEL_W<'a, O> {
    #[doc = "Minute change"]
    #[inline(always)]
    pub fn minute(self) -> &'a mut W {
        self.variant(TIMEVSELSELECT_A::MINUTE)
    }
    #[doc = "Hour change"]
    #[inline(always)]
    pub fn hour(self) -> &'a mut W {
        self.variant(TIMEVSELSELECT_A::HOUR)
    }
    #[doc = "Every day at midnight"]
    #[inline(always)]
    pub fn midnight(self) -> &'a mut W {
        self.variant(TIMEVSELSELECT_A::MIDNIGHT)
    }
    #[doc = "Every day at noon"]
    #[inline(always)]
    pub fn noon(self) -> &'a mut W {
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
    #[doc = "Checks if the value of the field is `WEEK`"]
    #[inline(always)]
    pub fn is_week(&self) -> bool {
        *self == CALEVSELSELECT_A::WEEK
    }
    #[doc = "Checks if the value of the field is `MONTH`"]
    #[inline(always)]
    pub fn is_month(&self) -> bool {
        *self == CALEVSELSELECT_A::MONTH
    }
    #[doc = "Checks if the value of the field is `YEAR`"]
    #[inline(always)]
    pub fn is_year(&self) -> bool {
        *self == CALEVSELSELECT_A::YEAR
    }
}
#[doc = "Field `CALEVSEL` writer - Calendar Event Selection"]
pub type CALEVSEL_W<'a, const O: u8> = crate::FieldWriter<'a, CR_SPEC, 2, O, CALEVSELSELECT_A>;
impl<'a, const O: u8> CALEVSEL_W<'a, O> {
    #[doc = "Week change (every Monday at time 00:00:00)"]
    #[inline(always)]
    pub fn week(self) -> &'a mut W {
        self.variant(CALEVSELSELECT_A::WEEK)
    }
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    #[inline(always)]
    pub fn month(self) -> &'a mut W {
        self.variant(CALEVSELSELECT_A::MONTH)
    }
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    #[inline(always)]
    pub fn year(self) -> &'a mut W {
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
    pub fn updtim(&mut self) -> UPDTIM_W<0> {
        UPDTIM_W::new(self)
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    #[must_use]
    pub fn updcal(&mut self) -> UPDCAL_W<1> {
        UPDCAL_W::new(self)
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn timevsel(&mut self) -> TIMEVSEL_W<8> {
        TIMEVSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn calevsel(&mut self) -> CALEVSEL_W<16> {
        CALEVSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
