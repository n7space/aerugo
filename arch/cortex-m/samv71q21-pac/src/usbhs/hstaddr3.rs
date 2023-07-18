#[doc = "Register `HSTADDR3` reader"]
pub struct R(crate::R<HSTADDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTADDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTADDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTADDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTADDR3` writer"]
pub struct W(crate::W<HSTADDR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTADDR3_SPEC>;
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
impl From<crate::W<HSTADDR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTADDR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTADDRP8` reader - USB Host Address"]
pub type HSTADDRP8_R = crate::FieldReader;
#[doc = "Field `HSTADDRP8` writer - USB Host Address"]
pub type HSTADDRP8_W<'a, const O: u8> = crate::FieldWriter<'a, HSTADDR3_SPEC, 7, O>;
#[doc = "Field `HSTADDRP9` reader - USB Host Address"]
pub type HSTADDRP9_R = crate::FieldReader;
#[doc = "Field `HSTADDRP9` writer - USB Host Address"]
pub type HSTADDRP9_W<'a, const O: u8> = crate::FieldWriter<'a, HSTADDR3_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp8(&self) -> HSTADDRP8_R {
        HSTADDRP8_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp9(&self) -> HSTADDRP9_R {
        HSTADDRP9_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp8(&mut self) -> HSTADDRP8_W<0> {
        HSTADDRP8_W::new(self)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    #[must_use]
    pub fn hstaddrp9(&mut self) -> HSTADDRP9_W<8> {
        HSTADDRP9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Address 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstaddr3](index.html) module"]
pub struct HSTADDR3_SPEC;
impl crate::RegisterSpec for HSTADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstaddr3::R](R) reader structure"]
impl crate::Readable for HSTADDR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstaddr3::W](W) writer structure"]
impl crate::Writable for HSTADDR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTADDR3 to value 0"]
impl crate::Resettable for HSTADDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
