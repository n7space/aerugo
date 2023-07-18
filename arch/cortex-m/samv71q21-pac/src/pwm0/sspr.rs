#[doc = "Register `SSPR` reader"]
pub struct R(crate::R<SSPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSPR` writer"]
pub struct W(crate::W<SSPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPR_SPEC>;
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
impl From<crate::W<SSPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPRD` reader - Spread Spectrum Limit Value"]
pub type SPRD_R = crate::FieldReader<u32>;
#[doc = "Field `SPRD` writer - Spread Spectrum Limit Value"]
pub type SPRD_W<'a, const O: u8> = crate::FieldWriter<'a, SSPR_SPEC, 24, O, u32>;
#[doc = "Field `SPRDM` reader - Spread Spectrum Counter Mode"]
pub type SPRDM_R = crate::BitReader;
#[doc = "Field `SPRDM` writer - Spread Spectrum Counter Mode"]
pub type SPRDM_W<'a, const O: u8> = crate::BitWriter<'a, SSPR_SPEC, O>;
impl R {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value"]
    #[inline(always)]
    pub fn sprd(&self) -> SPRD_R {
        SPRD_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Spread Spectrum Counter Mode"]
    #[inline(always)]
    pub fn sprdm(&self) -> SPRDM_R {
        SPRDM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value"]
    #[inline(always)]
    #[must_use]
    pub fn sprd(&mut self) -> SPRD_W<0> {
        SPRD_W::new(self)
    }
    #[doc = "Bit 24 - Spread Spectrum Counter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sprdm(&mut self) -> SPRDM_W<24> {
        SPRDM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Spread Spectrum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspr](index.html) module"]
pub struct SSPR_SPEC;
impl crate::RegisterSpec for SSPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspr::R](R) reader structure"]
impl crate::Readable for SSPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sspr::W](W) writer structure"]
impl crate::Writable for SSPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSPR to value 0"]
impl crate::Resettable for SSPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
