#[doc = "Register `QIDR` writer"]
pub struct W(crate::W<QIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QIDR_SPEC>;
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
impl From<crate::W<QIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDX` writer - Index"]
pub type IDX_W<'a, const O: u8> = crate::BitWriter<'a, QIDR_SPEC, O>;
#[doc = "Field `DIRCHG` writer - Direction Change"]
pub type DIRCHG_W<'a, const O: u8> = crate::BitWriter<'a, QIDR_SPEC, O>;
#[doc = "Field `QERR` writer - Quadrature Error"]
pub type QERR_W<'a, const O: u8> = crate::BitWriter<'a, QIDR_SPEC, O>;
#[doc = "Field `MPE` writer - Consecutive Missing Pulse Error"]
pub type MPE_W<'a, const O: u8> = crate::BitWriter<'a, QIDR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Index"]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IDX_W<0> {
        IDX_W::new(self)
    }
    #[doc = "Bit 1 - Direction Change"]
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DIRCHG_W<1> {
        DIRCHG_W::new(self)
    }
    #[doc = "Bit 2 - Quadrature Error"]
    #[inline(always)]
    #[must_use]
    pub fn qerr(&mut self) -> QERR_W<2> {
        QERR_W::new(self)
    }
    #[doc = "Bit 3 - Consecutive Missing Pulse Error"]
    #[inline(always)]
    #[must_use]
    pub fn mpe(&mut self) -> MPE_W<3> {
        MPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QDEC Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qidr](index.html) module"]
pub struct QIDR_SPEC;
impl crate::RegisterSpec for QIDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [qidr::W](W) writer structure"]
impl crate::Writable for QIDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QIDR to value 0"]
impl crate::Resettable for QIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
