#[doc = "Register `BRGR` reader"]
pub struct R(crate::R<BRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRGR` writer"]
pub struct W(crate::W<BRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRGR_SPEC>;
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
impl From<crate::W<BRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CD` reader - Clock Divisor"]
pub type CD_R = crate::FieldReader<u16>;
#[doc = "Field `CD` writer - Clock Divisor"]
pub type CD_W<'a, const O: u8> = crate::FieldWriter<'a, BRGR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn cd(&mut self) -> CD_W<0> {
        CD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgr](index.html) module"]
pub struct BRGR_SPEC;
impl crate::RegisterSpec for BRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brgr::R](R) reader structure"]
impl crate::Readable for BRGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brgr::W](W) writer structure"]
impl crate::Writable for BRGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRGR to value 0"]
impl crate::Resettable for BRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
