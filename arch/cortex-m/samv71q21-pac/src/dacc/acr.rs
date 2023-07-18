#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBCTLCH0` reader - Analog Output Current Control"]
pub type IBCTLCH0_R = crate::FieldReader;
#[doc = "Field `IBCTLCH0` writer - Analog Output Current Control"]
pub type IBCTLCH0_W<'a, const O: u8> = crate::FieldWriter<'a, ACR_SPEC, 2, O>;
#[doc = "Field `IBCTLCH1` reader - Analog Output Current Control"]
pub type IBCTLCH1_R = crate::FieldReader;
#[doc = "Field `IBCTLCH1` writer - Analog Output Current Control"]
pub type IBCTLCH1_W<'a, const O: u8> = crate::FieldWriter<'a, ACR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch0(&self) -> IBCTLCH0_R {
        IBCTLCH0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch1(&self) -> IBCTLCH1_R {
        IBCTLCH1_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Output Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn ibctlch0(&mut self) -> IBCTLCH0_W<0> {
        IBCTLCH0_W::new(self)
    }
    #[doc = "Bits 2:3 - Analog Output Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn ibctlch1(&mut self) -> IBCTLCH1_W<2> {
        IBCTLCH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Current Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
