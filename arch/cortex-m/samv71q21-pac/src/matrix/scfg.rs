#[doc = "Register `SCFG[%s]` reader"]
pub type R = crate::R<SCFG_SPEC>;
#[doc = "Register `SCFG[%s]` writer"]
pub type W = crate::W<SCFG_SPEC>;
#[doc = "Field `SLOT_CYCLE` reader - Maximum Bus Grant Duration for Masters"]
pub type SLOT_CYCLE_R = crate::FieldReader<u16>;
#[doc = "Field `SLOT_CYCLE` writer - Maximum Bus Grant Duration for Masters"]
pub type SLOT_CYCLE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub type DEFMSTR_TYPE_R = crate::FieldReader<DEFMSTR_TYPESELECT_A>;
#[doc = "Default Master Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEFMSTR_TYPESELECT_A {
    #[doc = "0: No Default Master-At the end of the current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in a one clock cycle latency for the first access of a burst transfer or for a single access."]
    NONE = 0,
    #[doc = "1: Last Default Master-At the end of the current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having one clock cycle latency when the last master tries to access the slave again."]
    LAST = 1,
    #[doc = "2: Fixed Default Master-At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having one clock cycle latency when the fixed master tries to access the slave again."]
    FIXED = 2,
}
impl From<DEFMSTR_TYPESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DEFMSTR_TYPESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DEFMSTR_TYPESELECT_A {
    type Ux = u8;
}
impl DEFMSTR_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEFMSTR_TYPESELECT_A> {
        match self.bits {
            0 => Some(DEFMSTR_TYPESELECT_A::NONE),
            1 => Some(DEFMSTR_TYPESELECT_A::LAST),
            2 => Some(DEFMSTR_TYPESELECT_A::FIXED),
            _ => None,
        }
    }
    #[doc = "No Default Master-At the end of the current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in a one clock cycle latency for the first access of a burst transfer or for a single access."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DEFMSTR_TYPESELECT_A::NONE
    }
    #[doc = "Last Default Master-At the end of the current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having one clock cycle latency when the last master tries to access the slave again."]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == DEFMSTR_TYPESELECT_A::LAST
    }
    #[doc = "Fixed Default Master-At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having one clock cycle latency when the fixed master tries to access the slave again."]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == DEFMSTR_TYPESELECT_A::FIXED
    }
}
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub type DEFMSTR_TYPE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, DEFMSTR_TYPESELECT_A>;
impl<'a, REG, const O: u8> DEFMSTR_TYPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Default Master-At the end of the current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in a one clock cycle latency for the first access of a burst transfer or for a single access."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DEFMSTR_TYPESELECT_A::NONE)
    }
    #[doc = "Last Default Master-At the end of the current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having one clock cycle latency when the last master tries to access the slave again."]
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(DEFMSTR_TYPESELECT_A::LAST)
    }
    #[doc = "Fixed Default Master-At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having one clock cycle latency when the fixed master tries to access the slave again."]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(DEFMSTR_TYPESELECT_A::FIXED)
    }
}
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Default Master"]
pub type FIXED_DEFMSTR_R = crate::FieldReader;
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Default Master"]
pub type FIXED_DEFMSTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:8 - Maximum Bus Grant Duration for Masters"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SLOT_CYCLE_R {
        SLOT_CYCLE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DEFMSTR_TYPE_R {
        DEFMSTR_TYPE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FIXED_DEFMSTR_R {
        FIXED_DEFMSTR_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Maximum Bus Grant Duration for Masters"]
    #[inline(always)]
    #[must_use]
    pub fn slot_cycle(&mut self) -> SLOT_CYCLE_W<SCFG_SPEC, 0> {
        SLOT_CYCLE_W::new(self)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    #[must_use]
    pub fn defmstr_type(&mut self) -> DEFMSTR_TYPE_W<SCFG_SPEC, 16> {
        DEFMSTR_TYPE_W::new(self)
    }
    #[doc = "Bits 18:21 - Fixed Default Master"]
    #[inline(always)]
    #[must_use]
    pub fn fixed_defmstr(&mut self) -> FIXED_DEFMSTR_W<SCFG_SPEC, 18> {
        FIXED_DEFMSTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Slave Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCFG_SPEC;
impl crate::RegisterSpec for SCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scfg::R`](R) reader structure"]
impl crate::Readable for SCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scfg::W`](W) writer structure"]
impl crate::Writable for SCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCFG[%s]
to value 0"]
impl crate::Resettable for SCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
