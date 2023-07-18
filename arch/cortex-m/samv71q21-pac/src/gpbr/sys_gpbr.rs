#[doc = "Register `SYS_GPBR[%s]` reader"]
pub struct R(crate::R<SYS_GPBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_GPBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_GPBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_GPBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_GPBR[%s]` writer"]
pub struct W(crate::W<SYS_GPBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_GPBR_SPEC>;
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
impl From<crate::W<SYS_GPBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_GPBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPBR_VALUE` reader - Value of GPBR x"]
pub type GPBR_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `GPBR_VALUE` writer - Value of GPBR x"]
pub type GPBR_VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, SYS_GPBR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of GPBR x"]
    #[inline(always)]
    pub fn gpbr_value(&self) -> GPBR_VALUE_R {
        GPBR_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of GPBR x"]
    #[inline(always)]
    #[must_use]
    pub fn gpbr_value(&mut self) -> GPBR_VALUE_W<0> {
        GPBR_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Backup Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpbr](index.html) module"]
pub struct SYS_GPBR_SPEC;
impl crate::RegisterSpec for SYS_GPBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_gpbr::R](R) reader structure"]
impl crate::Readable for SYS_GPBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_gpbr::W](W) writer structure"]
impl crate::Writable for SYS_GPBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYS_GPBR[%s]
to value 0"]
impl crate::Resettable for SYS_GPBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
