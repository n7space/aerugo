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
#[doc = "Field `PGA0EN` reader - PGA0 Enable"]
pub type PGA0EN_R = crate::BitReader;
#[doc = "Field `PGA0EN` writer - PGA0 Enable"]
pub type PGA0EN_W<'a, const O: u8> = crate::BitWriter<'a, ACR_SPEC, O>;
#[doc = "Field `PGA1EN` reader - PGA1 Enable"]
pub type PGA1EN_R = crate::BitReader;
#[doc = "Field `PGA1EN` writer - PGA1 Enable"]
pub type PGA1EN_W<'a, const O: u8> = crate::BitWriter<'a, ACR_SPEC, O>;
#[doc = "Field `IBCTL` reader - AFE Bias Current Control"]
pub type IBCTL_R = crate::FieldReader;
#[doc = "Field `IBCTL` writer - AFE Bias Current Control"]
pub type IBCTL_W<'a, const O: u8> = crate::FieldWriter<'a, ACR_SPEC, 2, O>;
impl R {
    #[doc = "Bit 2 - PGA0 Enable"]
    #[inline(always)]
    pub fn pga0en(&self) -> PGA0EN_R {
        PGA0EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PGA1 Enable"]
    #[inline(always)]
    pub fn pga1en(&self) -> PGA1EN_R {
        PGA1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - AFE Bias Current Control"]
    #[inline(always)]
    pub fn ibctl(&self) -> IBCTL_R {
        IBCTL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - PGA0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pga0en(&mut self) -> PGA0EN_W<2> {
        PGA0EN_W::new(self)
    }
    #[doc = "Bit 3 - PGA1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pga1en(&mut self) -> PGA1EN_W<3> {
        PGA1EN_W::new(self)
    }
    #[doc = "Bits 8:9 - AFE Bias Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn ibctl(&mut self) -> IBCTL_W<8> {
        IBCTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
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
