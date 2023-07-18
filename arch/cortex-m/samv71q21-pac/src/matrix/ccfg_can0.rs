#[doc = "Register `CCFG_CAN0` reader"]
pub struct R(crate::R<CCFG_CAN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_CAN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_CAN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_CAN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_CAN0` writer"]
pub struct W(crate::W<CCFG_CAN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_CAN0_SPEC>;
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
impl From<crate::W<CCFG_CAN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_CAN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN0DMABA` reader - CAN0 DMA Base Address"]
pub type CAN0DMABA_R = crate::FieldReader<u16>;
#[doc = "Field `CAN0DMABA` writer - CAN0 DMA Base Address"]
pub type CAN0DMABA_W<'a, const O: u8> = crate::FieldWriter<'a, CCFG_CAN0_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 16:31 - CAN0 DMA Base Address"]
    #[inline(always)]
    pub fn can0dmaba(&self) -> CAN0DMABA_R {
        CAN0DMABA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - CAN0 DMA Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn can0dmaba(&mut self) -> CAN0DMABA_W<16> {
        CAN0DMABA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_can0](index.html) module"]
pub struct CCFG_CAN0_SPEC;
impl crate::RegisterSpec for CCFG_CAN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_can0::R](R) reader structure"]
impl crate::Readable for CCFG_CAN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_can0::W](W) writer structure"]
impl crate::Writable for CCFG_CAN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_CAN0 to value 0"]
impl crate::Resettable for CCFG_CAN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
