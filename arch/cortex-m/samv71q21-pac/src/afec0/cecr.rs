#[doc = "Register `CECR` reader"]
pub struct R(crate::R<CECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CECR` writer"]
pub struct W(crate::W<CECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CECR_SPEC>;
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
impl From<crate::W<CECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECORR0` reader - Error Correction Enable for channel 0"]
pub type ECORR0_R = crate::BitReader;
#[doc = "Field `ECORR0` writer - Error Correction Enable for channel 0"]
pub type ECORR0_W<'a, const O: u8> = crate::BitWriter<'a, CECR_SPEC, O>;
#[doc = "Field `ECORR1` reader - Error Correction Enable for channel 1"]
pub type ECORR1_R = crate::BitReader;
#[doc = "Field `ECORR1` writer - Error Correction Enable for channel 1"]
pub type ECORR1_W<'a, const O: u8> = crate::BitWriter<'a, CECR_SPEC, O>;
#[doc = "Field `ECORR2` reader - Error Correction Enable for channel 2"]
pub type ECORR2_R = crate::BitReader;
#[doc = "Field `ECORR2` writer - Error Correction Enable for channel 2"]
pub type ECORR2_W<'a, const O: u8> = crate::BitWriter<'a, CECR_SPEC, O>;
#[doc = "Field `ECORR3` reader - Error Correction Enable for channel 3"]
pub type ECORR3_R = crate::BitReader;
#[doc = "Field `ECORR3` writer - Error Correction Enable for channel 3"]
pub type ECORR3_W<'a, const O: u8> = crate::BitWriter<'a, CECR_SPEC, O>;
#[doc = "Field `ECORR4` reader - Error Correction Enable for channel 4"]
pub type ECORR4_R = crate::BitReader;
#[doc = "Field `ECORR4` writer - Error Correction Enable for channel 4"]
pub type ECORR4_W<'a, const O: u8> = crate::BitWriter<'a, CECR_SPEC, O>;
#[doc = "Field `ECORR5` reader - Error Correction Enable for channel 5"]
pub type ECORR5_R = crate::BitReader;
#[doc = "Field `ECORR5` writer - Error Correction Enable for channel 5"]
pub type ECORR5_W<'a, const O: u8> = crate::BitWriter<'a, CECR_SPEC, O>;
#[doc = "Field `ECORR6` reader - Error Correction Enable for channel 6"]
pub type ECORR6_R = crate::BitReader;
#[doc = "Field `ECORR6` writer - Error Correction Enable for channel 6"]
pub type ECORR6_W<'a, const O: u8> = crate::BitWriter<'a, CECR_SPEC, O>;
#[doc = "Field `ECORR7` reader - Error Correction Enable for channel 7"]
pub type ECORR7_R = crate::BitReader;
#[doc = "Field `ECORR7` writer - Error Correction Enable for channel 7"]
pub type ECORR7_W<'a, const O: u8> = crate::BitWriter<'a, CECR_SPEC, O>;
#[doc = "Field `ECORR8` reader - Error Correction Enable for channel 8"]
pub type ECORR8_R = crate::BitReader;
#[doc = "Field `ECORR8` writer - Error Correction Enable for channel 8"]
pub type ECORR8_W<'a, const O: u8> = crate::BitWriter<'a, CECR_SPEC, O>;
#[doc = "Field `ECORR9` reader - Error Correction Enable for channel 9"]
pub type ECORR9_R = crate::BitReader;
#[doc = "Field `ECORR9` writer - Error Correction Enable for channel 9"]
pub type ECORR9_W<'a, const O: u8> = crate::BitWriter<'a, CECR_SPEC, O>;
#[doc = "Field `ECORR10` reader - Error Correction Enable for channel 10"]
pub type ECORR10_R = crate::BitReader;
#[doc = "Field `ECORR10` writer - Error Correction Enable for channel 10"]
pub type ECORR10_W<'a, const O: u8> = crate::BitWriter<'a, CECR_SPEC, O>;
#[doc = "Field `ECORR11` reader - Error Correction Enable for channel 11"]
pub type ECORR11_R = crate::BitReader;
#[doc = "Field `ECORR11` writer - Error Correction Enable for channel 11"]
pub type ECORR11_W<'a, const O: u8> = crate::BitWriter<'a, CECR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Error Correction Enable for channel 0"]
    #[inline(always)]
    pub fn ecorr0(&self) -> ECORR0_R {
        ECORR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Correction Enable for channel 1"]
    #[inline(always)]
    pub fn ecorr1(&self) -> ECORR1_R {
        ECORR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Correction Enable for channel 2"]
    #[inline(always)]
    pub fn ecorr2(&self) -> ECORR2_R {
        ECORR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Correction Enable for channel 3"]
    #[inline(always)]
    pub fn ecorr3(&self) -> ECORR3_R {
        ECORR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Error Correction Enable for channel 4"]
    #[inline(always)]
    pub fn ecorr4(&self) -> ECORR4_R {
        ECORR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Correction Enable for channel 5"]
    #[inline(always)]
    pub fn ecorr5(&self) -> ECORR5_R {
        ECORR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error Correction Enable for channel 6"]
    #[inline(always)]
    pub fn ecorr6(&self) -> ECORR6_R {
        ECORR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error Correction Enable for channel 7"]
    #[inline(always)]
    pub fn ecorr7(&self) -> ECORR7_R {
        ECORR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error Correction Enable for channel 8"]
    #[inline(always)]
    pub fn ecorr8(&self) -> ECORR8_R {
        ECORR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error Correction Enable for channel 9"]
    #[inline(always)]
    pub fn ecorr9(&self) -> ECORR9_R {
        ECORR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error Correction Enable for channel 10"]
    #[inline(always)]
    pub fn ecorr10(&self) -> ECORR10_R {
        ECORR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error Correction Enable for channel 11"]
    #[inline(always)]
    pub fn ecorr11(&self) -> ECORR11_R {
        ECORR11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Correction Enable for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ecorr0(&mut self) -> ECORR0_W<0> {
        ECORR0_W::new(self)
    }
    #[doc = "Bit 1 - Error Correction Enable for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ecorr1(&mut self) -> ECORR1_W<1> {
        ECORR1_W::new(self)
    }
    #[doc = "Bit 2 - Error Correction Enable for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ecorr2(&mut self) -> ECORR2_W<2> {
        ECORR2_W::new(self)
    }
    #[doc = "Bit 3 - Error Correction Enable for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ecorr3(&mut self) -> ECORR3_W<3> {
        ECORR3_W::new(self)
    }
    #[doc = "Bit 4 - Error Correction Enable for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ecorr4(&mut self) -> ECORR4_W<4> {
        ECORR4_W::new(self)
    }
    #[doc = "Bit 5 - Error Correction Enable for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ecorr5(&mut self) -> ECORR5_W<5> {
        ECORR5_W::new(self)
    }
    #[doc = "Bit 6 - Error Correction Enable for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ecorr6(&mut self) -> ECORR6_W<6> {
        ECORR6_W::new(self)
    }
    #[doc = "Bit 7 - Error Correction Enable for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ecorr7(&mut self) -> ECORR7_W<7> {
        ECORR7_W::new(self)
    }
    #[doc = "Bit 8 - Error Correction Enable for channel 8"]
    #[inline(always)]
    #[must_use]
    pub fn ecorr8(&mut self) -> ECORR8_W<8> {
        ECORR8_W::new(self)
    }
    #[doc = "Bit 9 - Error Correction Enable for channel 9"]
    #[inline(always)]
    #[must_use]
    pub fn ecorr9(&mut self) -> ECORR9_W<9> {
        ECORR9_W::new(self)
    }
    #[doc = "Bit 10 - Error Correction Enable for channel 10"]
    #[inline(always)]
    #[must_use]
    pub fn ecorr10(&mut self) -> ECORR10_W<10> {
        ECORR10_W::new(self)
    }
    #[doc = "Bit 11 - Error Correction Enable for channel 11"]
    #[inline(always)]
    #[must_use]
    pub fn ecorr11(&mut self) -> ECORR11_W<11> {
        ECORR11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Error Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cecr](index.html) module"]
pub struct CECR_SPEC;
impl crate::RegisterSpec for CECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cecr::R](R) reader structure"]
impl crate::Readable for CECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cecr::W](W) writer structure"]
impl crate::Writable for CECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CECR to value 0"]
impl crate::Resettable for CECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
