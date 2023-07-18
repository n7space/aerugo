#[doc = "Register `SCUP` reader"]
pub struct R(crate::R<SCUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCUP` writer"]
pub struct W(crate::W<SCUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCUP_SPEC>;
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
impl From<crate::W<SCUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPR` reader - Update Period"]
pub type UPR_R = crate::FieldReader;
#[doc = "Field `UPR` writer - Update Period"]
pub type UPR_W<'a, const O: u8> = crate::FieldWriter<'a, SCUP_SPEC, 4, O>;
#[doc = "Field `UPRCNT` reader - Update Period Counter"]
pub type UPRCNT_R = crate::FieldReader;
#[doc = "Field `UPRCNT` writer - Update Period Counter"]
pub type UPRCNT_W<'a, const O: u8> = crate::FieldWriter<'a, SCUP_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Update Period"]
    #[inline(always)]
    pub fn upr(&self) -> UPR_R {
        UPR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Update Period Counter"]
    #[inline(always)]
    pub fn uprcnt(&self) -> UPRCNT_R {
        UPRCNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update Period"]
    #[inline(always)]
    #[must_use]
    pub fn upr(&mut self) -> UPR_W<0> {
        UPR_W::new(self)
    }
    #[doc = "Bits 4:7 - Update Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn uprcnt(&mut self) -> UPRCNT_W<4> {
        UPRCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Sync Channels Update Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scup](index.html) module"]
pub struct SCUP_SPEC;
impl crate::RegisterSpec for SCUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scup::R](R) reader structure"]
impl crate::Readable for SCUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scup::W](W) writer structure"]
impl crate::Writable for SCUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCUP to value 0"]
impl crate::Resettable for SCUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
