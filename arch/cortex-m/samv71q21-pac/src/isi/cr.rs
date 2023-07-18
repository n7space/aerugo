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
#[doc = "Field `ISI_EN` writer - ISI Module Enable Request"]
pub type ISI_EN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `ISI_DIS` writer - ISI Module Disable Request"]
pub type ISI_DIS_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `ISI_SRST` writer - ISI Software Reset Request"]
pub type ISI_SRST_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
#[doc = "Field `ISI_CDC` writer - ISI Codec Request"]
pub type ISI_CDC_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - ISI Module Enable Request"]
    #[inline(always)]
    #[must_use]
    pub fn isi_en(&mut self) -> ISI_EN_W<0> {
        ISI_EN_W::new(self)
    }
    #[doc = "Bit 1 - ISI Module Disable Request"]
    #[inline(always)]
    #[must_use]
    pub fn isi_dis(&mut self) -> ISI_DIS_W<1> {
        ISI_DIS_W::new(self)
    }
    #[doc = "Bit 2 - ISI Software Reset Request"]
    #[inline(always)]
    #[must_use]
    pub fn isi_srst(&mut self) -> ISI_SRST_W<2> {
        ISI_SRST_W::new(self)
    }
    #[doc = "Bit 8 - ISI Codec Request"]
    #[inline(always)]
    #[must_use]
    pub fn isi_cdc(&mut self) -> ISI_CDC_W<8> {
        ISI_CDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
