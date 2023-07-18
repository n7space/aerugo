#[doc = "Register `SHMR` reader"]
pub struct R(crate::R<SHMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHMR` writer"]
pub struct W(crate::W<SHMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHMR_SPEC>;
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
impl From<crate::W<SHMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUAL0` reader - Dual Sample &amp; Hold for channel 0"]
pub type DUAL0_R = crate::BitReader;
#[doc = "Field `DUAL0` writer - Dual Sample &amp; Hold for channel 0"]
pub type DUAL0_W<'a, const O: u8> = crate::BitWriter<'a, SHMR_SPEC, O>;
#[doc = "Field `DUAL1` reader - Dual Sample &amp; Hold for channel 1"]
pub type DUAL1_R = crate::BitReader;
#[doc = "Field `DUAL1` writer - Dual Sample &amp; Hold for channel 1"]
pub type DUAL1_W<'a, const O: u8> = crate::BitWriter<'a, SHMR_SPEC, O>;
#[doc = "Field `DUAL2` reader - Dual Sample &amp; Hold for channel 2"]
pub type DUAL2_R = crate::BitReader;
#[doc = "Field `DUAL2` writer - Dual Sample &amp; Hold for channel 2"]
pub type DUAL2_W<'a, const O: u8> = crate::BitWriter<'a, SHMR_SPEC, O>;
#[doc = "Field `DUAL3` reader - Dual Sample &amp; Hold for channel 3"]
pub type DUAL3_R = crate::BitReader;
#[doc = "Field `DUAL3` writer - Dual Sample &amp; Hold for channel 3"]
pub type DUAL3_W<'a, const O: u8> = crate::BitWriter<'a, SHMR_SPEC, O>;
#[doc = "Field `DUAL4` reader - Dual Sample &amp; Hold for channel 4"]
pub type DUAL4_R = crate::BitReader;
#[doc = "Field `DUAL4` writer - Dual Sample &amp; Hold for channel 4"]
pub type DUAL4_W<'a, const O: u8> = crate::BitWriter<'a, SHMR_SPEC, O>;
#[doc = "Field `DUAL5` reader - Dual Sample &amp; Hold for channel 5"]
pub type DUAL5_R = crate::BitReader;
#[doc = "Field `DUAL5` writer - Dual Sample &amp; Hold for channel 5"]
pub type DUAL5_W<'a, const O: u8> = crate::BitWriter<'a, SHMR_SPEC, O>;
#[doc = "Field `DUAL6` reader - Dual Sample &amp; Hold for channel 6"]
pub type DUAL6_R = crate::BitReader;
#[doc = "Field `DUAL6` writer - Dual Sample &amp; Hold for channel 6"]
pub type DUAL6_W<'a, const O: u8> = crate::BitWriter<'a, SHMR_SPEC, O>;
#[doc = "Field `DUAL7` reader - Dual Sample &amp; Hold for channel 7"]
pub type DUAL7_R = crate::BitReader;
#[doc = "Field `DUAL7` writer - Dual Sample &amp; Hold for channel 7"]
pub type DUAL7_W<'a, const O: u8> = crate::BitWriter<'a, SHMR_SPEC, O>;
#[doc = "Field `DUAL8` reader - Dual Sample &amp; Hold for channel 8"]
pub type DUAL8_R = crate::BitReader;
#[doc = "Field `DUAL8` writer - Dual Sample &amp; Hold for channel 8"]
pub type DUAL8_W<'a, const O: u8> = crate::BitWriter<'a, SHMR_SPEC, O>;
#[doc = "Field `DUAL9` reader - Dual Sample &amp; Hold for channel 9"]
pub type DUAL9_R = crate::BitReader;
#[doc = "Field `DUAL9` writer - Dual Sample &amp; Hold for channel 9"]
pub type DUAL9_W<'a, const O: u8> = crate::BitWriter<'a, SHMR_SPEC, O>;
#[doc = "Field `DUAL10` reader - Dual Sample &amp; Hold for channel 10"]
pub type DUAL10_R = crate::BitReader;
#[doc = "Field `DUAL10` writer - Dual Sample &amp; Hold for channel 10"]
pub type DUAL10_W<'a, const O: u8> = crate::BitWriter<'a, SHMR_SPEC, O>;
#[doc = "Field `DUAL11` reader - Dual Sample &amp; Hold for channel 11"]
pub type DUAL11_R = crate::BitReader;
#[doc = "Field `DUAL11` writer - Dual Sample &amp; Hold for channel 11"]
pub type DUAL11_W<'a, const O: u8> = crate::BitWriter<'a, SHMR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Dual Sample &amp; Hold for channel 0"]
    #[inline(always)]
    pub fn dual0(&self) -> DUAL0_R {
        DUAL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dual Sample &amp; Hold for channel 1"]
    #[inline(always)]
    pub fn dual1(&self) -> DUAL1_R {
        DUAL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dual Sample &amp; Hold for channel 2"]
    #[inline(always)]
    pub fn dual2(&self) -> DUAL2_R {
        DUAL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dual Sample &amp; Hold for channel 3"]
    #[inline(always)]
    pub fn dual3(&self) -> DUAL3_R {
        DUAL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Dual Sample &amp; Hold for channel 4"]
    #[inline(always)]
    pub fn dual4(&self) -> DUAL4_R {
        DUAL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dual Sample &amp; Hold for channel 5"]
    #[inline(always)]
    pub fn dual5(&self) -> DUAL5_R {
        DUAL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Dual Sample &amp; Hold for channel 6"]
    #[inline(always)]
    pub fn dual6(&self) -> DUAL6_R {
        DUAL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dual Sample &amp; Hold for channel 7"]
    #[inline(always)]
    pub fn dual7(&self) -> DUAL7_R {
        DUAL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Dual Sample &amp; Hold for channel 8"]
    #[inline(always)]
    pub fn dual8(&self) -> DUAL8_R {
        DUAL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Dual Sample &amp; Hold for channel 9"]
    #[inline(always)]
    pub fn dual9(&self) -> DUAL9_R {
        DUAL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Dual Sample &amp; Hold for channel 10"]
    #[inline(always)]
    pub fn dual10(&self) -> DUAL10_R {
        DUAL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Dual Sample &amp; Hold for channel 11"]
    #[inline(always)]
    pub fn dual11(&self) -> DUAL11_R {
        DUAL11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dual Sample &amp; Hold for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn dual0(&mut self) -> DUAL0_W<0> {
        DUAL0_W::new(self)
    }
    #[doc = "Bit 1 - Dual Sample &amp; Hold for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn dual1(&mut self) -> DUAL1_W<1> {
        DUAL1_W::new(self)
    }
    #[doc = "Bit 2 - Dual Sample &amp; Hold for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn dual2(&mut self) -> DUAL2_W<2> {
        DUAL2_W::new(self)
    }
    #[doc = "Bit 3 - Dual Sample &amp; Hold for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn dual3(&mut self) -> DUAL3_W<3> {
        DUAL3_W::new(self)
    }
    #[doc = "Bit 4 - Dual Sample &amp; Hold for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn dual4(&mut self) -> DUAL4_W<4> {
        DUAL4_W::new(self)
    }
    #[doc = "Bit 5 - Dual Sample &amp; Hold for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn dual5(&mut self) -> DUAL5_W<5> {
        DUAL5_W::new(self)
    }
    #[doc = "Bit 6 - Dual Sample &amp; Hold for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn dual6(&mut self) -> DUAL6_W<6> {
        DUAL6_W::new(self)
    }
    #[doc = "Bit 7 - Dual Sample &amp; Hold for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn dual7(&mut self) -> DUAL7_W<7> {
        DUAL7_W::new(self)
    }
    #[doc = "Bit 8 - Dual Sample &amp; Hold for channel 8"]
    #[inline(always)]
    #[must_use]
    pub fn dual8(&mut self) -> DUAL8_W<8> {
        DUAL8_W::new(self)
    }
    #[doc = "Bit 9 - Dual Sample &amp; Hold for channel 9"]
    #[inline(always)]
    #[must_use]
    pub fn dual9(&mut self) -> DUAL9_W<9> {
        DUAL9_W::new(self)
    }
    #[doc = "Bit 10 - Dual Sample &amp; Hold for channel 10"]
    #[inline(always)]
    #[must_use]
    pub fn dual10(&mut self) -> DUAL10_W<10> {
        DUAL10_W::new(self)
    }
    #[doc = "Bit 11 - Dual Sample &amp; Hold for channel 11"]
    #[inline(always)]
    #[must_use]
    pub fn dual11(&mut self) -> DUAL11_W<11> {
        DUAL11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Sample &amp; Hold Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shmr](index.html) module"]
pub struct SHMR_SPEC;
impl crate::RegisterSpec for SHMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shmr::R](R) reader structure"]
impl crate::Readable for SHMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shmr::W](W) writer structure"]
impl crate::Writable for SHMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHMR to value 0"]
impl crate::Resettable for SHMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
