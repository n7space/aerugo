#[doc = "Register `SCUC` reader"]
pub struct R(crate::R<SCUC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCUC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCUC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCUC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCUC` writer"]
pub struct W(crate::W<SCUC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCUC_SPEC>;
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
impl From<crate::W<SCUC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCUC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDULOCK` reader - Synchronous Channels Update Unlock"]
pub type UPDULOCK_R = crate::BitReader;
#[doc = "Field `UPDULOCK` writer - Synchronous Channels Update Unlock"]
pub type UPDULOCK_W<'a, const O: u8> = crate::BitWriter<'a, SCUC_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Synchronous Channels Update Unlock"]
    #[inline(always)]
    pub fn updulock(&self) -> UPDULOCK_R {
        UPDULOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Channels Update Unlock"]
    #[inline(always)]
    #[must_use]
    pub fn updulock(&mut self) -> UPDULOCK_W<0> {
        UPDULOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Sync Channels Update Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scuc](index.html) module"]
pub struct SCUC_SPEC;
impl crate::RegisterSpec for SCUC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scuc::R](R) reader structure"]
impl crate::Readable for SCUC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scuc::W](W) writer structure"]
impl crate::Writable for SCUC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCUC to value 0"]
impl crate::Resettable for SCUC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
