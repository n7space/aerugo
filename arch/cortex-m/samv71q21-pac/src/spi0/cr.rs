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
#[doc = "Field `SPIEN` writer - SPI Enable"]
pub type SPIEN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `SPIDIS` writer - SPI Disable"]
pub type SPIDIS_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `SWRST` writer - SPI Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `REQCLR` writer - Request to Clear the Comparison Trigger"]
pub type REQCLR_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LASTXFER_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - SPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<0> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 1 - SPI Disable"]
    #[inline(always)]
    #[must_use]
    pub fn spidis(&mut self) -> SPIDIS_W<1> {
        SPIDIS_W::new(self)
    }
    #[doc = "Bit 7 - SPI Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<7> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 12 - Request to Clear the Comparison Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn reqclr(&mut self) -> REQCLR_W<12> {
        REQCLR_W::new(self)
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn lastxfer(&mut self) -> LASTXFER_W<24> {
        LASTXFER_W::new(self)
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
