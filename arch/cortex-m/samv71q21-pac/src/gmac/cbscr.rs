#[doc = "Register `CBSCR` reader"]
pub struct R(crate::R<CBSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBSCR` writer"]
pub struct W(crate::W<CBSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBSCR_SPEC>;
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
impl From<crate::W<CBSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QBE` reader - Queue B CBS Enable"]
pub type QBE_R = crate::BitReader;
#[doc = "Field `QBE` writer - Queue B CBS Enable"]
pub type QBE_W<'a, const O: u8> = crate::BitWriter<'a, CBSCR_SPEC, O>;
#[doc = "Field `QAE` reader - Queue A CBS Enable"]
pub type QAE_R = crate::BitReader;
#[doc = "Field `QAE` writer - Queue A CBS Enable"]
pub type QAE_W<'a, const O: u8> = crate::BitWriter<'a, CBSCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Queue B CBS Enable"]
    #[inline(always)]
    pub fn qbe(&self) -> QBE_R {
        QBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Queue A CBS Enable"]
    #[inline(always)]
    pub fn qae(&self) -> QAE_R {
        QAE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Queue B CBS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qbe(&mut self) -> QBE_W<0> {
        QBE_W::new(self)
    }
    #[doc = "Bit 1 - Queue A CBS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qae(&mut self) -> QAE_W<1> {
        QAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Credit-Based Shaping Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbscr](index.html) module"]
pub struct CBSCR_SPEC;
impl crate::RegisterSpec for CBSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cbscr::R](R) reader structure"]
impl crate::Readable for CBSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbscr::W](W) writer structure"]
impl crate::Writable for CBSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBSCR to value 0"]
impl crate::Resettable for CBSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
