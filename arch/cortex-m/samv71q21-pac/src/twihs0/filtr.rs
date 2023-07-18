#[doc = "Register `FILTR` reader"]
pub struct R(crate::R<FILTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTR` writer"]
pub struct W(crate::W<FILTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTR_SPEC>;
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
impl From<crate::W<FILTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILT` reader - RX Digital Filter"]
pub type FILT_R = crate::BitReader;
#[doc = "Field `FILT` writer - RX Digital Filter"]
pub type FILT_W<'a, const O: u8> = crate::BitWriter<'a, FILTR_SPEC, O>;
#[doc = "Field `PADFEN` reader - PAD Filter Enable"]
pub type PADFEN_R = crate::BitReader;
#[doc = "Field `PADFEN` writer - PAD Filter Enable"]
pub type PADFEN_W<'a, const O: u8> = crate::BitWriter<'a, FILTR_SPEC, O>;
#[doc = "Field `PADFCFG` reader - PAD Filter Config"]
pub type PADFCFG_R = crate::BitReader;
#[doc = "Field `PADFCFG` writer - PAD Filter Config"]
pub type PADFCFG_W<'a, const O: u8> = crate::BitWriter<'a, FILTR_SPEC, O>;
#[doc = "Field `THRES` reader - Digital Filter Threshold"]
pub type THRES_R = crate::FieldReader;
#[doc = "Field `THRES` writer - Digital Filter Threshold"]
pub type THRES_W<'a, const O: u8> = crate::FieldWriter<'a, FILTR_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - RX Digital Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAD Filter Enable"]
    #[inline(always)]
    pub fn padfen(&self) -> PADFEN_R {
        PADFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAD Filter Config"]
    #[inline(always)]
    pub fn padfcfg(&self) -> PADFCFG_R {
        PADFCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Digital Filter Threshold"]
    #[inline(always)]
    pub fn thres(&self) -> THRES_R {
        THRES_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RX Digital Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filt(&mut self) -> FILT_W<0> {
        FILT_W::new(self)
    }
    #[doc = "Bit 1 - PAD Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn padfen(&mut self) -> PADFEN_W<1> {
        PADFEN_W::new(self)
    }
    #[doc = "Bit 2 - PAD Filter Config"]
    #[inline(always)]
    #[must_use]
    pub fn padfcfg(&mut self) -> PADFCFG_W<2> {
        PADFCFG_W::new(self)
    }
    #[doc = "Bits 8:10 - Digital Filter Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn thres(&mut self) -> THRES_W<8> {
        THRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filtr](index.html) module"]
pub struct FILTR_SPEC;
impl crate::RegisterSpec for FILTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filtr::R](R) reader structure"]
impl crate::Readable for FILTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filtr::W](W) writer structure"]
impl crate::Writable for FILTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTR to value 0"]
impl crate::Resettable for FILTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
