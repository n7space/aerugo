#[doc = "Register `CALR` reader"]
pub type R = crate::R<CALR_SPEC>;
#[doc = "Register `CALR` writer"]
pub type W = crate::W<CALR_SPEC>;
#[doc = "Field `CENT` reader - Current Century"]
pub type CENT_R = crate::FieldReader;
#[doc = "Field `CENT` writer - Current Century"]
pub type CENT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `YEAR` reader - Current Year"]
pub type YEAR_R = crate::FieldReader;
#[doc = "Field `YEAR` writer - Current Year"]
pub type YEAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `MONTH` reader - Current Month"]
pub type MONTH_R = crate::FieldReader;
#[doc = "Field `MONTH` writer - Current Month"]
pub type MONTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DAY` reader - Current Day in Current Week"]
pub type DAY_R = crate::FieldReader;
#[doc = "Field `DAY` writer - Current Day in Current Week"]
pub type DAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DATE` reader - Current Day in Current Month"]
pub type DATE_R = crate::FieldReader;
#[doc = "Field `DATE` writer - Current Day in Current Month"]
pub type DATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:6 - Current Century"]
    #[inline(always)]
    pub fn cent(&self) -> CENT_R {
        CENT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Current Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Current Month"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Current Day in Current Week"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:29 - Current Day in Current Month"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Current Century"]
    #[inline(always)]
    #[must_use]
    pub fn cent(&mut self) -> CENT_W<CALR_SPEC, 0> {
        CENT_W::new(self)
    }
    #[doc = "Bits 8:15 - Current Year"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YEAR_W<CALR_SPEC, 8> {
        YEAR_W::new(self)
    }
    #[doc = "Bits 16:20 - Current Month"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MONTH_W<CALR_SPEC, 16> {
        MONTH_W::new(self)
    }
    #[doc = "Bits 21:23 - Current Day in Current Week"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<CALR_SPEC, 21> {
        DAY_W::new(self)
    }
    #[doc = "Bits 24:29 - Current Day in Current Month"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<CALR_SPEC, 24> {
        DATE_W::new(self)
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
#[doc = "Calendar Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALR_SPEC;
impl crate::RegisterSpec for CALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calr::R`](R) reader structure"]
impl crate::Readable for CALR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calr::W`](W) writer structure"]
impl crate::Writable for CALR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALR to value 0"]
impl crate::Resettable for CALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
