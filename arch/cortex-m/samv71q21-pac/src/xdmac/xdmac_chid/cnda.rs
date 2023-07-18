#[doc = "Register `CNDA` reader"]
pub struct R(crate::R<CNDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNDA` writer"]
pub struct W(crate::W<CNDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDA_SPEC>;
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
impl From<crate::W<CNDA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDAIF` reader - Channel x Next Descriptor Interface"]
pub type NDAIF_R = crate::BitReader;
#[doc = "Field `NDAIF` writer - Channel x Next Descriptor Interface"]
pub type NDAIF_W<'a, const O: u8> = crate::BitWriter<'a, CNDA_SPEC, O>;
#[doc = "Field `NDA` reader - Channel x Next Descriptor Address"]
pub type NDA_R = crate::FieldReader<u32>;
#[doc = "Field `NDA` writer - Channel x Next Descriptor Address"]
pub type NDA_W<'a, const O: u8> = crate::FieldWriter<'a, CNDA_SPEC, 30, O, u32>;
impl R {
    #[doc = "Bit 0 - Channel x Next Descriptor Interface"]
    #[inline(always)]
    pub fn ndaif(&self) -> NDAIF_R {
        NDAIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:31 - Channel x Next Descriptor Address"]
    #[inline(always)]
    pub fn nda(&self) -> NDA_R {
        NDA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Channel x Next Descriptor Interface"]
    #[inline(always)]
    #[must_use]
    pub fn ndaif(&mut self) -> NDAIF_W<0> {
        NDAIF_W::new(self)
    }
    #[doc = "Bits 2:31 - Channel x Next Descriptor Address"]
    #[inline(always)]
    #[must_use]
    pub fn nda(&mut self) -> NDA_W<2> {
        NDA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Next Descriptor Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnda](index.html) module"]
pub struct CNDA_SPEC;
impl crate::RegisterSpec for CNDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnda::R](R) reader structure"]
impl crate::Readable for CNDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnda::W](W) writer structure"]
impl crate::Writable for CNDA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNDA to value 0"]
impl crate::Resettable for CNDA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
