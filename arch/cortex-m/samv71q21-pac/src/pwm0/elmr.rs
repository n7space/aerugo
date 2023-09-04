#[doc = "Register `ELMR[%s]` reader"]
pub type R = crate::R<ELMR_SPEC>;
#[doc = "Register `ELMR[%s]` writer"]
pub type W = crate::W<ELMR_SPEC>;
#[doc = "Field `CSEL0` reader - Comparison 0 Selection"]
pub type CSEL0_R = crate::BitReader;
#[doc = "Field `CSEL0` writer - Comparison 0 Selection"]
pub type CSEL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSEL1` reader - Comparison 1 Selection"]
pub type CSEL1_R = crate::BitReader;
#[doc = "Field `CSEL1` writer - Comparison 1 Selection"]
pub type CSEL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSEL2` reader - Comparison 2 Selection"]
pub type CSEL2_R = crate::BitReader;
#[doc = "Field `CSEL2` writer - Comparison 2 Selection"]
pub type CSEL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSEL3` reader - Comparison 3 Selection"]
pub type CSEL3_R = crate::BitReader;
#[doc = "Field `CSEL3` writer - Comparison 3 Selection"]
pub type CSEL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSEL4` reader - Comparison 4 Selection"]
pub type CSEL4_R = crate::BitReader;
#[doc = "Field `CSEL4` writer - Comparison 4 Selection"]
pub type CSEL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSEL5` reader - Comparison 5 Selection"]
pub type CSEL5_R = crate::BitReader;
#[doc = "Field `CSEL5` writer - Comparison 5 Selection"]
pub type CSEL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSEL6` reader - Comparison 6 Selection"]
pub type CSEL6_R = crate::BitReader;
#[doc = "Field `CSEL6` writer - Comparison 6 Selection"]
pub type CSEL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSEL7` reader - Comparison 7 Selection"]
pub type CSEL7_R = crate::BitReader;
#[doc = "Field `CSEL7` writer - Comparison 7 Selection"]
pub type CSEL7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Comparison 0 Selection"]
    #[inline(always)]
    pub fn csel0(&self) -> CSEL0_R {
        CSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparison 1 Selection"]
    #[inline(always)]
    pub fn csel1(&self) -> CSEL1_R {
        CSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparison 2 Selection"]
    #[inline(always)]
    pub fn csel2(&self) -> CSEL2_R {
        CSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparison 3 Selection"]
    #[inline(always)]
    pub fn csel3(&self) -> CSEL3_R {
        CSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparison 4 Selection"]
    #[inline(always)]
    pub fn csel4(&self) -> CSEL4_R {
        CSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparison 5 Selection"]
    #[inline(always)]
    pub fn csel5(&self) -> CSEL5_R {
        CSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparison 6 Selection"]
    #[inline(always)]
    pub fn csel6(&self) -> CSEL6_R {
        CSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparison 7 Selection"]
    #[inline(always)]
    pub fn csel7(&self) -> CSEL7_R {
        CSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison 0 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel0(&mut self) -> CSEL0_W<ELMR_SPEC, 0> {
        CSEL0_W::new(self)
    }
    #[doc = "Bit 1 - Comparison 1 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel1(&mut self) -> CSEL1_W<ELMR_SPEC, 1> {
        CSEL1_W::new(self)
    }
    #[doc = "Bit 2 - Comparison 2 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel2(&mut self) -> CSEL2_W<ELMR_SPEC, 2> {
        CSEL2_W::new(self)
    }
    #[doc = "Bit 3 - Comparison 3 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel3(&mut self) -> CSEL3_W<ELMR_SPEC, 3> {
        CSEL3_W::new(self)
    }
    #[doc = "Bit 4 - Comparison 4 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel4(&mut self) -> CSEL4_W<ELMR_SPEC, 4> {
        CSEL4_W::new(self)
    }
    #[doc = "Bit 5 - Comparison 5 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel5(&mut self) -> CSEL5_W<ELMR_SPEC, 5> {
        CSEL5_W::new(self)
    }
    #[doc = "Bit 6 - Comparison 6 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel6(&mut self) -> CSEL6_W<ELMR_SPEC, 6> {
        CSEL6_W::new(self)
    }
    #[doc = "Bit 7 - Comparison 7 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel7(&mut self) -> CSEL7_W<ELMR_SPEC, 7> {
        CSEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Event Line 0 Mode Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ELMR_SPEC;
impl crate::RegisterSpec for ELMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elmr::R`](R) reader structure"]
impl crate::Readable for ELMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`elmr::W`](W) writer structure"]
impl crate::Writable for ELMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELMR[%s]
to value 0"]
impl crate::Resettable for ELMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
