#[doc = "Register `US_BRGR` reader"]
pub struct R(crate::R<US_BRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_BRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_BRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_BRGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_BRGR` writer"]
pub struct W(crate::W<US_BRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_BRGR_SPEC>;
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
impl From<crate::W<US_BRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_BRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CD` reader - Clock Divider"]
pub type CD_R = crate::FieldReader<u16>;
#[doc = "Field `CD` writer - Clock Divider"]
pub type CD_W<'a, const O: u8> = crate::FieldWriter<'a, US_BRGR_SPEC, 16, O, u16>;
#[doc = "Field `FP` reader - Fractional Part"]
pub type FP_R = crate::FieldReader;
#[doc = "Field `FP` writer - Fractional Part"]
pub type FP_W<'a, const O: u8> = crate::FieldWriter<'a, US_BRGR_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:15 - Clock Divider"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FP_R {
        FP_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn cd(&mut self) -> CD_W<0> {
        CD_W::new(self)
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    #[must_use]
    pub fn fp(&mut self) -> FP_W<16> {
        FP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_brgr](index.html) module"]
pub struct US_BRGR_SPEC;
impl crate::RegisterSpec for US_BRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_brgr::R](R) reader structure"]
impl crate::Readable for US_BRGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_brgr::W](W) writer structure"]
impl crate::Writable for US_BRGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_BRGR to value 0"]
impl crate::Resettable for US_BRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
