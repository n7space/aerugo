#[doc = "Register `R2Y_SET1` reader"]
pub struct R(crate::R<R2Y_SET1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R2Y_SET1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R2Y_SET1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R2Y_SET1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R2Y_SET1` writer"]
pub struct W(crate::W<R2Y_SET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R2Y_SET1_SPEC>;
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
impl From<crate::W<R2Y_SET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R2Y_SET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C3` reader - Color Space Conversion Matrix Coefficient C3"]
pub type C3_R = crate::FieldReader;
#[doc = "Field `C3` writer - Color Space Conversion Matrix Coefficient C3"]
pub type C3_W<'a, const O: u8> = crate::FieldWriter<'a, R2Y_SET1_SPEC, 7, O>;
#[doc = "Field `C4` reader - Color Space Conversion Matrix Coefficient C4"]
pub type C4_R = crate::FieldReader;
#[doc = "Field `C4` writer - Color Space Conversion Matrix Coefficient C4"]
pub type C4_W<'a, const O: u8> = crate::FieldWriter<'a, R2Y_SET1_SPEC, 7, O>;
#[doc = "Field `C5` reader - Color Space Conversion Matrix Coefficient C5"]
pub type C5_R = crate::FieldReader;
#[doc = "Field `C5` writer - Color Space Conversion Matrix Coefficient C5"]
pub type C5_W<'a, const O: u8> = crate::FieldWriter<'a, R2Y_SET1_SPEC, 7, O>;
#[doc = "Field `Goff` reader - Color Space Conversion Green Component Offset"]
pub type GOFF_R = crate::BitReader;
#[doc = "Field `Goff` writer - Color Space Conversion Green Component Offset"]
pub type GOFF_W<'a, const O: u8> = crate::BitWriter<'a, R2Y_SET1_SPEC, O>;
impl R {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C3"]
    #[inline(always)]
    pub fn c3(&self) -> C3_R {
        C3_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    pub fn c4(&self) -> C4_R {
        C4_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C5"]
    #[inline(always)]
    pub fn c5(&self) -> C5_R {
        C5_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Color Space Conversion Green Component Offset"]
    #[inline(always)]
    pub fn goff(&self) -> GOFF_R {
        GOFF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C3"]
    #[inline(always)]
    #[must_use]
    pub fn c3(&mut self) -> C3_W<0> {
        C3_W::new(self)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    #[must_use]
    pub fn c4(&mut self) -> C4_W<8> {
        C4_W::new(self)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C5"]
    #[inline(always)]
    #[must_use]
    pub fn c5(&mut self) -> C5_W<16> {
        C5_W::new(self)
    }
    #[doc = "Bit 24 - Color Space Conversion Green Component Offset"]
    #[inline(always)]
    #[must_use]
    pub fn goff(&mut self) -> GOFF_W<24> {
        GOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2y_set1](index.html) module"]
pub struct R2Y_SET1_SPEC;
impl crate::RegisterSpec for R2Y_SET1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r2y_set1::R](R) reader structure"]
impl crate::Readable for R2Y_SET1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r2y_set1::W](W) writer structure"]
impl crate::Writable for R2Y_SET1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R2Y_SET1 to value 0"]
impl crate::Resettable for R2Y_SET1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
