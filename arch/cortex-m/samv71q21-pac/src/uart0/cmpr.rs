#[doc = "Register `CMPR` reader"]
pub type R = crate::R<CMPR_SPEC>;
#[doc = "Register `CMPR` writer"]
pub type W = crate::W<CMPR_SPEC>;
#[doc = "Field `VAL1` reader - First Comparison Value for Received Character"]
pub type VAL1_R = crate::FieldReader;
#[doc = "Field `VAL1` writer - First Comparison Value for Received Character"]
pub type VAL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub type CMPMODE_R = crate::BitReader<CMPMODESELECT_A>;
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPMODESELECT_A {
    #[doc = "0: Any character is received and comparison function drives CMP flag."]
    FLAG_ONLY = 0,
    #[doc = "1: Comparison condition must be met to start reception."]
    START_CONDITION = 1,
}
impl From<CMPMODESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CMPMODESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMODESELECT_A {
        match self.bits {
            false => CMPMODESELECT_A::FLAG_ONLY,
            true => CMPMODESELECT_A::START_CONDITION,
        }
    }
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline(always)]
    pub fn is_flag_only(&self) -> bool {
        *self == CMPMODESELECT_A::FLAG_ONLY
    }
    #[doc = "Comparison condition must be met to start reception."]
    #[inline(always)]
    pub fn is_start_condition(&self) -> bool {
        *self == CMPMODESELECT_A::START_CONDITION
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub type CMPMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CMPMODESELECT_A>;
impl<'a, REG, const O: u8> CMPMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline(always)]
    pub fn flag_only(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMODESELECT_A::FLAG_ONLY)
    }
    #[doc = "Comparison condition must be met to start reception."]
    #[inline(always)]
    pub fn start_condition(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMODESELECT_A::START_CONDITION)
    }
}
#[doc = "Field `CMPPAR` reader - Compare Parity"]
pub type CMPPAR_R = crate::BitReader;
#[doc = "Field `CMPPAR` writer - Compare Parity"]
pub type CMPPAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VAL2` reader - Second Comparison Value for Received Character"]
pub type VAL2_R = crate::FieldReader;
#[doc = "Field `VAL2` writer - Second Comparison Value for Received Character"]
pub type VAL2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&self) -> VAL1_R {
        VAL1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline(always)]
    pub fn cmppar(&self) -> CMPPAR_R {
        CMPPAR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&self) -> VAL2_R {
        VAL2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - First Comparison Value for Received Character"]
    #[inline(always)]
    #[must_use]
    pub fn val1(&mut self) -> VAL1_W<CMPR_SPEC, 0> {
        VAL1_W::new(self)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmode(&mut self) -> CMPMODE_W<CMPR_SPEC, 12> {
        CMPMODE_W::new(self)
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline(always)]
    #[must_use]
    pub fn cmppar(&mut self) -> CMPPAR_W<CMPR_SPEC, 14> {
        CMPPAR_W::new(self)
    }
    #[doc = "Bits 16:23 - Second Comparison Value for Received Character"]
    #[inline(always)]
    #[must_use]
    pub fn val2(&mut self) -> VAL2_W<CMPR_SPEC, 16> {
        VAL2_W::new(self)
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
#[doc = "Comparison Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPR_SPEC;
impl crate::RegisterSpec for CMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpr::R`](R) reader structure"]
impl crate::Readable for CMPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpr::W`](W) writer structure"]
impl crate::Writable for CMPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPR to value 0"]
impl crate::Resettable for CMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
