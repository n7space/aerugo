#[doc = "Register `CGR` reader"]
pub struct R(crate::R<CGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CGR` writer"]
pub struct W(crate::W<CGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGR_SPEC>;
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
impl From<crate::W<CGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAIN0` reader - Gain for Channel 0"]
pub type GAIN0_R = crate::FieldReader;
#[doc = "Field `GAIN0` writer - Gain for Channel 0"]
pub type GAIN0_W<'a, const O: u8> = crate::FieldWriter<'a, CGR_SPEC, 2, O>;
#[doc = "Field `GAIN1` reader - Gain for Channel 1"]
pub type GAIN1_R = crate::FieldReader;
#[doc = "Field `GAIN1` writer - Gain for Channel 1"]
pub type GAIN1_W<'a, const O: u8> = crate::FieldWriter<'a, CGR_SPEC, 2, O>;
#[doc = "Field `GAIN2` reader - Gain for Channel 2"]
pub type GAIN2_R = crate::FieldReader;
#[doc = "Field `GAIN2` writer - Gain for Channel 2"]
pub type GAIN2_W<'a, const O: u8> = crate::FieldWriter<'a, CGR_SPEC, 2, O>;
#[doc = "Field `GAIN3` reader - Gain for Channel 3"]
pub type GAIN3_R = crate::FieldReader;
#[doc = "Field `GAIN3` writer - Gain for Channel 3"]
pub type GAIN3_W<'a, const O: u8> = crate::FieldWriter<'a, CGR_SPEC, 2, O>;
#[doc = "Field `GAIN4` reader - Gain for Channel 4"]
pub type GAIN4_R = crate::FieldReader;
#[doc = "Field `GAIN4` writer - Gain for Channel 4"]
pub type GAIN4_W<'a, const O: u8> = crate::FieldWriter<'a, CGR_SPEC, 2, O>;
#[doc = "Field `GAIN5` reader - Gain for Channel 5"]
pub type GAIN5_R = crate::FieldReader;
#[doc = "Field `GAIN5` writer - Gain for Channel 5"]
pub type GAIN5_W<'a, const O: u8> = crate::FieldWriter<'a, CGR_SPEC, 2, O>;
#[doc = "Field `GAIN6` reader - Gain for Channel 6"]
pub type GAIN6_R = crate::FieldReader;
#[doc = "Field `GAIN6` writer - Gain for Channel 6"]
pub type GAIN6_W<'a, const O: u8> = crate::FieldWriter<'a, CGR_SPEC, 2, O>;
#[doc = "Field `GAIN7` reader - Gain for Channel 7"]
pub type GAIN7_R = crate::FieldReader;
#[doc = "Field `GAIN7` writer - Gain for Channel 7"]
pub type GAIN7_W<'a, const O: u8> = crate::FieldWriter<'a, CGR_SPEC, 2, O>;
#[doc = "Field `GAIN8` reader - Gain for Channel 8"]
pub type GAIN8_R = crate::FieldReader;
#[doc = "Field `GAIN8` writer - Gain for Channel 8"]
pub type GAIN8_W<'a, const O: u8> = crate::FieldWriter<'a, CGR_SPEC, 2, O>;
#[doc = "Field `GAIN9` reader - Gain for Channel 9"]
pub type GAIN9_R = crate::FieldReader;
#[doc = "Field `GAIN9` writer - Gain for Channel 9"]
pub type GAIN9_W<'a, const O: u8> = crate::FieldWriter<'a, CGR_SPEC, 2, O>;
#[doc = "Field `GAIN10` reader - Gain for Channel 10"]
pub type GAIN10_R = crate::FieldReader;
#[doc = "Field `GAIN10` writer - Gain for Channel 10"]
pub type GAIN10_W<'a, const O: u8> = crate::FieldWriter<'a, CGR_SPEC, 2, O>;
#[doc = "Field `GAIN11` reader - Gain for Channel 11"]
pub type GAIN11_R = crate::FieldReader;
#[doc = "Field `GAIN11` writer - Gain for Channel 11"]
pub type GAIN11_W<'a, const O: u8> = crate::FieldWriter<'a, CGR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Gain for Channel 0"]
    #[inline(always)]
    pub fn gain0(&self) -> GAIN0_R {
        GAIN0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Gain for Channel 1"]
    #[inline(always)]
    pub fn gain1(&self) -> GAIN1_R {
        GAIN1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Gain for Channel 2"]
    #[inline(always)]
    pub fn gain2(&self) -> GAIN2_R {
        GAIN2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Gain for Channel 3"]
    #[inline(always)]
    pub fn gain3(&self) -> GAIN3_R {
        GAIN3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Gain for Channel 4"]
    #[inline(always)]
    pub fn gain4(&self) -> GAIN4_R {
        GAIN4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Gain for Channel 5"]
    #[inline(always)]
    pub fn gain5(&self) -> GAIN5_R {
        GAIN5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Gain for Channel 6"]
    #[inline(always)]
    pub fn gain6(&self) -> GAIN6_R {
        GAIN6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Gain for Channel 7"]
    #[inline(always)]
    pub fn gain7(&self) -> GAIN7_R {
        GAIN7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Gain for Channel 8"]
    #[inline(always)]
    pub fn gain8(&self) -> GAIN8_R {
        GAIN8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Gain for Channel 9"]
    #[inline(always)]
    pub fn gain9(&self) -> GAIN9_R {
        GAIN9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Gain for Channel 10"]
    #[inline(always)]
    pub fn gain10(&self) -> GAIN10_R {
        GAIN10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Gain for Channel 11"]
    #[inline(always)]
    pub fn gain11(&self) -> GAIN11_R {
        GAIN11_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Gain for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn gain0(&mut self) -> GAIN0_W<0> {
        GAIN0_W::new(self)
    }
    #[doc = "Bits 2:3 - Gain for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn gain1(&mut self) -> GAIN1_W<2> {
        GAIN1_W::new(self)
    }
    #[doc = "Bits 4:5 - Gain for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn gain2(&mut self) -> GAIN2_W<4> {
        GAIN2_W::new(self)
    }
    #[doc = "Bits 6:7 - Gain for Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn gain3(&mut self) -> GAIN3_W<6> {
        GAIN3_W::new(self)
    }
    #[doc = "Bits 8:9 - Gain for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn gain4(&mut self) -> GAIN4_W<8> {
        GAIN4_W::new(self)
    }
    #[doc = "Bits 10:11 - Gain for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn gain5(&mut self) -> GAIN5_W<10> {
        GAIN5_W::new(self)
    }
    #[doc = "Bits 12:13 - Gain for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn gain6(&mut self) -> GAIN6_W<12> {
        GAIN6_W::new(self)
    }
    #[doc = "Bits 14:15 - Gain for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn gain7(&mut self) -> GAIN7_W<14> {
        GAIN7_W::new(self)
    }
    #[doc = "Bits 16:17 - Gain for Channel 8"]
    #[inline(always)]
    #[must_use]
    pub fn gain8(&mut self) -> GAIN8_W<16> {
        GAIN8_W::new(self)
    }
    #[doc = "Bits 18:19 - Gain for Channel 9"]
    #[inline(always)]
    #[must_use]
    pub fn gain9(&mut self) -> GAIN9_W<18> {
        GAIN9_W::new(self)
    }
    #[doc = "Bits 20:21 - Gain for Channel 10"]
    #[inline(always)]
    #[must_use]
    pub fn gain10(&mut self) -> GAIN10_W<20> {
        GAIN10_W::new(self)
    }
    #[doc = "Bits 22:23 - Gain for Channel 11"]
    #[inline(always)]
    #[must_use]
    pub fn gain11(&mut self) -> GAIN11_W<22> {
        GAIN11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgr](index.html) module"]
pub struct CGR_SPEC;
impl crate::RegisterSpec for CGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgr::R](R) reader structure"]
impl crate::Readable for CGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgr::W](W) writer structure"]
impl crate::Writable for CGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGR to value 0"]
impl crate::Resettable for CGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
