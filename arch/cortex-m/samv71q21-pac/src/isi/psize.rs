#[doc = "Register `PSIZE` reader"]
pub struct R(crate::R<PSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSIZE` writer"]
pub struct W(crate::W<PSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSIZE_SPEC>;
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
impl From<crate::W<PSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PREV_VSIZE` reader - Vertical Size for the Preview Path"]
pub type PREV_VSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PREV_VSIZE` writer - Vertical Size for the Preview Path"]
pub type PREV_VSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, PSIZE_SPEC, 10, O, u16>;
#[doc = "Field `PREV_HSIZE` reader - Horizontal Size for the Preview Path"]
pub type PREV_HSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PREV_HSIZE` writer - Horizontal Size for the Preview Path"]
pub type PREV_HSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, PSIZE_SPEC, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - Vertical Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_vsize(&self) -> PREV_VSIZE_R {
        PREV_VSIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Horizontal Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_hsize(&self) -> PREV_HSIZE_R {
        PREV_HSIZE_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Size for the Preview Path"]
    #[inline(always)]
    #[must_use]
    pub fn prev_vsize(&mut self) -> PREV_VSIZE_W<0> {
        PREV_VSIZE_W::new(self)
    }
    #[doc = "Bits 16:25 - Horizontal Size for the Preview Path"]
    #[inline(always)]
    #[must_use]
    pub fn prev_hsize(&mut self) -> PREV_HSIZE_W<16> {
        PREV_HSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Preview Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psize](index.html) module"]
pub struct PSIZE_SPEC;
impl crate::RegisterSpec for PSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psize::R](R) reader structure"]
impl crate::Readable for PSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psize::W](W) writer structure"]
impl crate::Writable for PSIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSIZE to value 0"]
impl crate::Resettable for PSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
