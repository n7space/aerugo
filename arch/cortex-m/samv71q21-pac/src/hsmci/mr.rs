#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - Clock Divider"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock Divider"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, MR_SPEC, 8, O>;
#[doc = "Field `PWSDIV` reader - Power Saving Divider"]
pub type PWSDIV_R = crate::FieldReader;
#[doc = "Field `PWSDIV` writer - Power Saving Divider"]
pub type PWSDIV_W<'a, const O: u8> = crate::FieldWriter<'a, MR_SPEC, 3, O>;
#[doc = "Field `RDPROOF` reader - Read Proof Enable"]
pub type RDPROOF_R = crate::BitReader;
#[doc = "Field `RDPROOF` writer - Read Proof Enable"]
pub type RDPROOF_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `WRPROOF` reader - Write Proof Enable"]
pub type WRPROOF_R = crate::BitReader;
#[doc = "Field `WRPROOF` writer - Write Proof Enable"]
pub type WRPROOF_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `FBYTE` reader - Force Byte Transfer"]
pub type FBYTE_R = crate::BitReader;
#[doc = "Field `FBYTE` writer - Force Byte Transfer"]
pub type FBYTE_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `PADV` reader - Padding Value"]
pub type PADV_R = crate::BitReader;
#[doc = "Field `PADV` writer - Padding Value"]
pub type PADV_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `CLKODD` reader - Clock divider is odd"]
pub type CLKODD_R = crate::BitReader;
#[doc = "Field `CLKODD` writer - Clock divider is odd"]
pub type CLKODD_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Power Saving Divider"]
    #[inline(always)]
    pub fn pwsdiv(&self) -> PWSDIV_R {
        PWSDIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Read Proof Enable"]
    #[inline(always)]
    pub fn rdproof(&self) -> RDPROOF_R {
        RDPROOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write Proof Enable"]
    #[inline(always)]
    pub fn wrproof(&self) -> WRPROOF_R {
        WRPROOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force Byte Transfer"]
    #[inline(always)]
    pub fn fbyte(&self) -> FBYTE_R {
        FBYTE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Padding Value"]
    #[inline(always)]
    pub fn padv(&self) -> PADV_R {
        PADV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Clock divider is odd"]
    #[inline(always)]
    pub fn clkodd(&self) -> CLKODD_R {
        CLKODD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 8:10 - Power Saving Divider"]
    #[inline(always)]
    #[must_use]
    pub fn pwsdiv(&mut self) -> PWSDIV_W<8> {
        PWSDIV_W::new(self)
    }
    #[doc = "Bit 11 - Read Proof Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdproof(&mut self) -> RDPROOF_W<11> {
        RDPROOF_W::new(self)
    }
    #[doc = "Bit 12 - Write Proof Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wrproof(&mut self) -> WRPROOF_W<12> {
        WRPROOF_W::new(self)
    }
    #[doc = "Bit 13 - Force Byte Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn fbyte(&mut self) -> FBYTE_W<13> {
        FBYTE_W::new(self)
    }
    #[doc = "Bit 14 - Padding Value"]
    #[inline(always)]
    #[must_use]
    pub fn padv(&mut self) -> PADV_W<14> {
        PADV_W::new(self)
    }
    #[doc = "Bit 16 - Clock divider is odd"]
    #[inline(always)]
    #[must_use]
    pub fn clkodd(&mut self) -> CLKODD_W<16> {
        CLKODD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
