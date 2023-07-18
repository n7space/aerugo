#[doc = "Register `GWAC` reader"]
pub struct R(crate::R<GWAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GWAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GWAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GWAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GWAC` writer"]
pub struct W(crate::W<GWAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GWAC_SPEC>;
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
impl From<crate::W<GWAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GWAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PW0` reader - Pool Weight 0"]
pub type PW0_R = crate::FieldReader;
#[doc = "Field `PW0` writer - Pool Weight 0"]
pub type PW0_W<'a, const O: u8> = crate::FieldWriter<'a, GWAC_SPEC, 4, O>;
#[doc = "Field `PW1` reader - Pool Weight 1"]
pub type PW1_R = crate::FieldReader;
#[doc = "Field `PW1` writer - Pool Weight 1"]
pub type PW1_W<'a, const O: u8> = crate::FieldWriter<'a, GWAC_SPEC, 4, O>;
#[doc = "Field `PW2` reader - Pool Weight 2"]
pub type PW2_R = crate::FieldReader;
#[doc = "Field `PW2` writer - Pool Weight 2"]
pub type PW2_W<'a, const O: u8> = crate::FieldWriter<'a, GWAC_SPEC, 4, O>;
#[doc = "Field `PW3` reader - Pool Weight 3"]
pub type PW3_R = crate::FieldReader;
#[doc = "Field `PW3` writer - Pool Weight 3"]
pub type PW3_W<'a, const O: u8> = crate::FieldWriter<'a, GWAC_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Pool Weight 0"]
    #[inline(always)]
    pub fn pw0(&self) -> PW0_R {
        PW0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pool Weight 1"]
    #[inline(always)]
    pub fn pw1(&self) -> PW1_R {
        PW1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pool Weight 2"]
    #[inline(always)]
    pub fn pw2(&self) -> PW2_R {
        PW2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pool Weight 3"]
    #[inline(always)]
    pub fn pw3(&self) -> PW3_R {
        PW3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pool Weight 0"]
    #[inline(always)]
    #[must_use]
    pub fn pw0(&mut self) -> PW0_W<0> {
        PW0_W::new(self)
    }
    #[doc = "Bits 4:7 - Pool Weight 1"]
    #[inline(always)]
    #[must_use]
    pub fn pw1(&mut self) -> PW1_W<4> {
        PW1_W::new(self)
    }
    #[doc = "Bits 8:11 - Pool Weight 2"]
    #[inline(always)]
    #[must_use]
    pub fn pw2(&mut self) -> PW2_W<8> {
        PW2_W::new(self)
    }
    #[doc = "Bits 12:15 - Pool Weight 3"]
    #[inline(always)]
    #[must_use]
    pub fn pw3(&mut self) -> PW3_W<12> {
        PW3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Weighted Arbiter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gwac](index.html) module"]
pub struct GWAC_SPEC;
impl crate::RegisterSpec for GWAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gwac::R](R) reader structure"]
impl crate::Readable for GWAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gwac::W](W) writer structure"]
impl crate::Writable for GWAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GWAC to value 0"]
impl crate::Resettable for GWAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
