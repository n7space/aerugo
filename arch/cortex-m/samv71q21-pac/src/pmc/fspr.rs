#[doc = "Register `FSPR` reader"]
pub struct R(crate::R<FSPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSPR` writer"]
pub struct W(crate::W<FSPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSPR_SPEC>;
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
impl From<crate::W<FSPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSTP0` reader - Fast Startup Input Polarity 0"]
pub type FSTP0_R = crate::BitReader;
#[doc = "Field `FSTP0` writer - Fast Startup Input Polarity 0"]
pub type FSTP0_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP1` reader - Fast Startup Input Polarity 1"]
pub type FSTP1_R = crate::BitReader;
#[doc = "Field `FSTP1` writer - Fast Startup Input Polarity 1"]
pub type FSTP1_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP2` reader - Fast Startup Input Polarity 2"]
pub type FSTP2_R = crate::BitReader;
#[doc = "Field `FSTP2` writer - Fast Startup Input Polarity 2"]
pub type FSTP2_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP3` reader - Fast Startup Input Polarity 3"]
pub type FSTP3_R = crate::BitReader;
#[doc = "Field `FSTP3` writer - Fast Startup Input Polarity 3"]
pub type FSTP3_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP4` reader - Fast Startup Input Polarity 4"]
pub type FSTP4_R = crate::BitReader;
#[doc = "Field `FSTP4` writer - Fast Startup Input Polarity 4"]
pub type FSTP4_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP5` reader - Fast Startup Input Polarity 5"]
pub type FSTP5_R = crate::BitReader;
#[doc = "Field `FSTP5` writer - Fast Startup Input Polarity 5"]
pub type FSTP5_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP6` reader - Fast Startup Input Polarity 6"]
pub type FSTP6_R = crate::BitReader;
#[doc = "Field `FSTP6` writer - Fast Startup Input Polarity 6"]
pub type FSTP6_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP7` reader - Fast Startup Input Polarity 7"]
pub type FSTP7_R = crate::BitReader;
#[doc = "Field `FSTP7` writer - Fast Startup Input Polarity 7"]
pub type FSTP7_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP8` reader - Fast Startup Input Polarity 8"]
pub type FSTP8_R = crate::BitReader;
#[doc = "Field `FSTP8` writer - Fast Startup Input Polarity 8"]
pub type FSTP8_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP9` reader - Fast Startup Input Polarity 9"]
pub type FSTP9_R = crate::BitReader;
#[doc = "Field `FSTP9` writer - Fast Startup Input Polarity 9"]
pub type FSTP9_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP10` reader - Fast Startup Input Polarity 10"]
pub type FSTP10_R = crate::BitReader;
#[doc = "Field `FSTP10` writer - Fast Startup Input Polarity 10"]
pub type FSTP10_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP11` reader - Fast Startup Input Polarity 11"]
pub type FSTP11_R = crate::BitReader;
#[doc = "Field `FSTP11` writer - Fast Startup Input Polarity 11"]
pub type FSTP11_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP12` reader - Fast Startup Input Polarity 12"]
pub type FSTP12_R = crate::BitReader;
#[doc = "Field `FSTP12` writer - Fast Startup Input Polarity 12"]
pub type FSTP12_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP13` reader - Fast Startup Input Polarity 13"]
pub type FSTP13_R = crate::BitReader;
#[doc = "Field `FSTP13` writer - Fast Startup Input Polarity 13"]
pub type FSTP13_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP14` reader - Fast Startup Input Polarity 14"]
pub type FSTP14_R = crate::BitReader;
#[doc = "Field `FSTP14` writer - Fast Startup Input Polarity 14"]
pub type FSTP14_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
#[doc = "Field `FSTP15` reader - Fast Startup Input Polarity 15"]
pub type FSTP15_R = crate::BitReader;
#[doc = "Field `FSTP15` writer - Fast Startup Input Polarity 15"]
pub type FSTP15_W<'a, const O: u8> = crate::BitWriter<'a, FSPR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Fast Startup Input Polarity 0"]
    #[inline(always)]
    pub fn fstp0(&self) -> FSTP0_R {
        FSTP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast Startup Input Polarity 1"]
    #[inline(always)]
    pub fn fstp1(&self) -> FSTP1_R {
        FSTP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fast Startup Input Polarity 2"]
    #[inline(always)]
    pub fn fstp2(&self) -> FSTP2_R {
        FSTP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fast Startup Input Polarity 3"]
    #[inline(always)]
    pub fn fstp3(&self) -> FSTP3_R {
        FSTP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fast Startup Input Polarity 4"]
    #[inline(always)]
    pub fn fstp4(&self) -> FSTP4_R {
        FSTP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fast Startup Input Polarity 5"]
    #[inline(always)]
    pub fn fstp5(&self) -> FSTP5_R {
        FSTP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Startup Input Polarity 6"]
    #[inline(always)]
    pub fn fstp6(&self) -> FSTP6_R {
        FSTP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast Startup Input Polarity 7"]
    #[inline(always)]
    pub fn fstp7(&self) -> FSTP7_R {
        FSTP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast Startup Input Polarity 8"]
    #[inline(always)]
    pub fn fstp8(&self) -> FSTP8_R {
        FSTP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast Startup Input Polarity 9"]
    #[inline(always)]
    pub fn fstp9(&self) -> FSTP9_R {
        FSTP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fast Startup Input Polarity 10"]
    #[inline(always)]
    pub fn fstp10(&self) -> FSTP10_R {
        FSTP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fast Startup Input Polarity 11"]
    #[inline(always)]
    pub fn fstp11(&self) -> FSTP11_R {
        FSTP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fast Startup Input Polarity 12"]
    #[inline(always)]
    pub fn fstp12(&self) -> FSTP12_R {
        FSTP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fast Startup Input Polarity 13"]
    #[inline(always)]
    pub fn fstp13(&self) -> FSTP13_R {
        FSTP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Startup Input Polarity 14"]
    #[inline(always)]
    pub fn fstp14(&self) -> FSTP14_R {
        FSTP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast Startup Input Polarity 15"]
    #[inline(always)]
    pub fn fstp15(&self) -> FSTP15_R {
        FSTP15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast Startup Input Polarity 0"]
    #[inline(always)]
    #[must_use]
    pub fn fstp0(&mut self) -> FSTP0_W<0> {
        FSTP0_W::new(self)
    }
    #[doc = "Bit 1 - Fast Startup Input Polarity 1"]
    #[inline(always)]
    #[must_use]
    pub fn fstp1(&mut self) -> FSTP1_W<1> {
        FSTP1_W::new(self)
    }
    #[doc = "Bit 2 - Fast Startup Input Polarity 2"]
    #[inline(always)]
    #[must_use]
    pub fn fstp2(&mut self) -> FSTP2_W<2> {
        FSTP2_W::new(self)
    }
    #[doc = "Bit 3 - Fast Startup Input Polarity 3"]
    #[inline(always)]
    #[must_use]
    pub fn fstp3(&mut self) -> FSTP3_W<3> {
        FSTP3_W::new(self)
    }
    #[doc = "Bit 4 - Fast Startup Input Polarity 4"]
    #[inline(always)]
    #[must_use]
    pub fn fstp4(&mut self) -> FSTP4_W<4> {
        FSTP4_W::new(self)
    }
    #[doc = "Bit 5 - Fast Startup Input Polarity 5"]
    #[inline(always)]
    #[must_use]
    pub fn fstp5(&mut self) -> FSTP5_W<5> {
        FSTP5_W::new(self)
    }
    #[doc = "Bit 6 - Fast Startup Input Polarity 6"]
    #[inline(always)]
    #[must_use]
    pub fn fstp6(&mut self) -> FSTP6_W<6> {
        FSTP6_W::new(self)
    }
    #[doc = "Bit 7 - Fast Startup Input Polarity 7"]
    #[inline(always)]
    #[must_use]
    pub fn fstp7(&mut self) -> FSTP7_W<7> {
        FSTP7_W::new(self)
    }
    #[doc = "Bit 8 - Fast Startup Input Polarity 8"]
    #[inline(always)]
    #[must_use]
    pub fn fstp8(&mut self) -> FSTP8_W<8> {
        FSTP8_W::new(self)
    }
    #[doc = "Bit 9 - Fast Startup Input Polarity 9"]
    #[inline(always)]
    #[must_use]
    pub fn fstp9(&mut self) -> FSTP9_W<9> {
        FSTP9_W::new(self)
    }
    #[doc = "Bit 10 - Fast Startup Input Polarity 10"]
    #[inline(always)]
    #[must_use]
    pub fn fstp10(&mut self) -> FSTP10_W<10> {
        FSTP10_W::new(self)
    }
    #[doc = "Bit 11 - Fast Startup Input Polarity 11"]
    #[inline(always)]
    #[must_use]
    pub fn fstp11(&mut self) -> FSTP11_W<11> {
        FSTP11_W::new(self)
    }
    #[doc = "Bit 12 - Fast Startup Input Polarity 12"]
    #[inline(always)]
    #[must_use]
    pub fn fstp12(&mut self) -> FSTP12_W<12> {
        FSTP12_W::new(self)
    }
    #[doc = "Bit 13 - Fast Startup Input Polarity 13"]
    #[inline(always)]
    #[must_use]
    pub fn fstp13(&mut self) -> FSTP13_W<13> {
        FSTP13_W::new(self)
    }
    #[doc = "Bit 14 - Fast Startup Input Polarity 14"]
    #[inline(always)]
    #[must_use]
    pub fn fstp14(&mut self) -> FSTP14_W<14> {
        FSTP14_W::new(self)
    }
    #[doc = "Bit 15 - Fast Startup Input Polarity 15"]
    #[inline(always)]
    #[must_use]
    pub fn fstp15(&mut self) -> FSTP15_W<15> {
        FSTP15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast Startup Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fspr](index.html) module"]
pub struct FSPR_SPEC;
impl crate::RegisterSpec for FSPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fspr::R](R) reader structure"]
impl crate::Readable for FSPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fspr::W](W) writer structure"]
impl crate::Writable for FSPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSPR to value 0"]
impl crate::Resettable for FSPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
