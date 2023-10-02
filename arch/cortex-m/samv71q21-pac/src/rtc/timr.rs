#[doc = "Register `TIMR` reader"]
pub type R = crate::R<TIMR_SPEC>;
#[doc = "Register `TIMR` writer"]
pub type W = crate::W<TIMR_SPEC>;
#[doc = "Field `SEC` reader - Current Second"]
pub type SEC_R = crate::FieldReader;
#[doc = "Field `SEC` writer - Current Second"]
pub type SEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `MIN` reader - Current Minute"]
pub type MIN_R = crate::FieldReader;
#[doc = "Field `MIN` writer - Current Minute"]
pub type MIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `HOUR` reader - Current Hour"]
pub type HOUR_R = crate::FieldReader;
#[doc = "Field `HOUR` writer - Current Hour"]
pub type HOUR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `AMPM` reader - Ante Meridiem Post Meridiem Indicator"]
pub type AMPM_R = crate::BitReader;
#[doc = "Field `AMPM` writer - Ante Meridiem Post Meridiem Indicator"]
pub type AMPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Current Second"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Current Minute"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - Current Hour"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Ante Meridiem Post Meridiem Indicator"]
    #[inline(always)]
    pub fn ampm(&self) -> AMPM_R {
        AMPM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Current Second"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<TIMR_SPEC, 0> {
        SEC_W::new(self)
    }
    #[doc = "Bits 8:14 - Current Minute"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MIN_W<TIMR_SPEC, 8> {
        MIN_W::new(self)
    }
    #[doc = "Bits 16:21 - Current Hour"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<TIMR_SPEC, 16> {
        HOUR_W::new(self)
    }
    #[doc = "Bit 22 - Ante Meridiem Post Meridiem Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn ampm(&mut self) -> AMPM_W<TIMR_SPEC, 22> {
        AMPM_W::new(self)
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
#[doc = "Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMR_SPEC;
impl crate::RegisterSpec for TIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timr::R`](R) reader structure"]
impl crate::Readable for TIMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timr::W`](W) writer structure"]
impl crate::Writable for TIMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMR to value 0"]
impl crate::Resettable for TIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
