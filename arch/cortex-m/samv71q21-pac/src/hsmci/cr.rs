#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCIEN` writer - Multi-Media Interface Enable"]
pub type MCIEN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `MCIDIS` writer - Multi-Media Interface Disable"]
pub type MCIDIS_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `PWSEN` writer - Power Save Mode Enable"]
pub type PWSEN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `PWSDIS` writer - Power Save Mode Disable"]
pub type PWSDIS_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Multi-Media Interface Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcien(&mut self) -> MCIEN_W<0> {
        MCIEN_W::new(self)
    }
    #[doc = "Bit 1 - Multi-Media Interface Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mcidis(&mut self) -> MCIDIS_W<1> {
        MCIDIS_W::new(self)
    }
    #[doc = "Bit 2 - Power Save Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwsen(&mut self) -> PWSEN_W<2> {
        PWSEN_W::new(self)
    }
    #[doc = "Bit 3 - Power Save Mode Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pwsdis(&mut self) -> PWSDIS_W<3> {
        PWSDIS_W::new(self)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<7> {
        SWRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
