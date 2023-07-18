#[doc = "Register `PMMR` reader"]
pub struct R(crate::R<PMMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMR` writer"]
pub struct W(crate::W<PMMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMR_SPEC>;
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
impl From<crate::W<PMMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLA_MMAX` reader - PLLA Maximum Allowed Multiplier Value"]
pub type PLLA_MMAX_R = crate::FieldReader<u16>;
#[doc = "Field `PLLA_MMAX` writer - PLLA Maximum Allowed Multiplier Value"]
pub type PLLA_MMAX_W<'a, const O: u8> = crate::FieldWriter<'a, PMMR_SPEC, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - PLLA Maximum Allowed Multiplier Value"]
    #[inline(always)]
    pub fn plla_mmax(&self) -> PLLA_MMAX_R {
        PLLA_MMAX_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - PLLA Maximum Allowed Multiplier Value"]
    #[inline(always)]
    #[must_use]
    pub fn plla_mmax(&mut self) -> PLLA_MMAX_W<0> {
        PLLA_MMAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Maximum Multiplier Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmr](index.html) module"]
pub struct PMMR_SPEC;
impl crate::RegisterSpec for PMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmmr::R](R) reader structure"]
impl crate::Readable for PMMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmr::W](W) writer structure"]
impl crate::Writable for PMMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMMR to value 0"]
impl crate::Resettable for PMMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
