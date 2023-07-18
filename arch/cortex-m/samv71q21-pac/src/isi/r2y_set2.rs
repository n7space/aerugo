#[doc = "Register `R2Y_SET2` reader"]
pub struct R(crate::R<R2Y_SET2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R2Y_SET2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R2Y_SET2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R2Y_SET2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R2Y_SET2` writer"]
pub struct W(crate::W<R2Y_SET2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R2Y_SET2_SPEC>;
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
impl From<crate::W<R2Y_SET2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R2Y_SET2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C6` reader - Color Space Conversion Matrix Coefficient C6"]
pub type C6_R = crate::FieldReader;
#[doc = "Field `C6` writer - Color Space Conversion Matrix Coefficient C6"]
pub type C6_W<'a, const O: u8> = crate::FieldWriter<'a, R2Y_SET2_SPEC, 7, O>;
#[doc = "Field `C7` reader - Color Space Conversion Matrix Coefficient C7"]
pub type C7_R = crate::FieldReader;
#[doc = "Field `C7` writer - Color Space Conversion Matrix Coefficient C7"]
pub type C7_W<'a, const O: u8> = crate::FieldWriter<'a, R2Y_SET2_SPEC, 7, O>;
#[doc = "Field `C8` reader - Color Space Conversion Matrix Coefficient C8"]
pub type C8_R = crate::FieldReader;
#[doc = "Field `C8` writer - Color Space Conversion Matrix Coefficient C8"]
pub type C8_W<'a, const O: u8> = crate::FieldWriter<'a, R2Y_SET2_SPEC, 7, O>;
#[doc = "Field `Boff` reader - Color Space Conversion Blue Component Offset"]
pub type BOFF_R = crate::BitReader;
#[doc = "Field `Boff` writer - Color Space Conversion Blue Component Offset"]
pub type BOFF_W<'a, const O: u8> = crate::BitWriter<'a, R2Y_SET2_SPEC, O>;
impl R {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C6"]
    #[inline(always)]
    pub fn c6(&self) -> C6_R {
        C6_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C7"]
    #[inline(always)]
    pub fn c7(&self) -> C7_R {
        C7_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C8"]
    #[inline(always)]
    pub fn c8(&self) -> C8_R {
        C8_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Color Space Conversion Blue Component Offset"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C6"]
    #[inline(always)]
    #[must_use]
    pub fn c6(&mut self) -> C6_W<0> {
        C6_W::new(self)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C7"]
    #[inline(always)]
    #[must_use]
    pub fn c7(&mut self) -> C7_W<8> {
        C7_W::new(self)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C8"]
    #[inline(always)]
    #[must_use]
    pub fn c8(&mut self) -> C8_W<16> {
        C8_W::new(self)
    }
    #[doc = "Bit 24 - Color Space Conversion Blue Component Offset"]
    #[inline(always)]
    #[must_use]
    pub fn boff(&mut self) -> BOFF_W<24> {
        BOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2y_set2](index.html) module"]
pub struct R2Y_SET2_SPEC;
impl crate::RegisterSpec for R2Y_SET2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r2y_set2::R](R) reader structure"]
impl crate::Readable for R2Y_SET2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r2y_set2::W](W) writer structure"]
impl crate::Writable for R2Y_SET2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R2Y_SET2 to value 0"]
impl crate::Resettable for R2Y_SET2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
