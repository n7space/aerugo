#[doc = "Register `MCTL` reader"]
pub struct R(crate::R<MCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTL` writer"]
pub struct W(crate::W<MCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTL_SPEC>;
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
impl From<crate::W<MCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XCMP` reader - Transfer Complete (Write 0 to Clear)"]
pub type XCMP_R = crate::BitReader;
#[doc = "Field `XCMP` writer - Transfer Complete (Write 0 to Clear)"]
pub type XCMP_W<'a, const O: u8> = crate::BitWriter<'a, MCTL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Transfer Complete (Write 0 to Clear)"]
    #[inline(always)]
    pub fn xcmp(&self) -> XCMP_R {
        XCMP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete (Write 0 to Clear)"]
    #[inline(always)]
    #[must_use]
    pub fn xcmp(&mut self) -> XCMP_W<0> {
        XCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIF Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctl](index.html) module"]
pub struct MCTL_SPEC;
impl crate::RegisterSpec for MCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctl::R](R) reader structure"]
impl crate::Readable for MCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctl::W](W) writer structure"]
impl crate::Writable for MCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTL to value 0"]
impl crate::Resettable for MCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
