#[doc = "Register `CDS_MSP` reader"]
pub struct R(crate::R<CDS_MSP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDS_MSP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDS_MSP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDS_MSP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDS_MSP` writer"]
pub struct W(crate::W<CDS_MSP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDS_MSP_SPEC>;
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
impl From<crate::W<CDS_MSP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDS_MSP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDS_MSP` reader - Channel x Source Data stride or Memory Set Pattern"]
pub type SDS_MSP_R = crate::FieldReader<u16>;
#[doc = "Field `SDS_MSP` writer - Channel x Source Data stride or Memory Set Pattern"]
pub type SDS_MSP_W<'a, const O: u8> = crate::FieldWriter<'a, CDS_MSP_SPEC, 16, O, u16>;
#[doc = "Field `DDS_MSP` reader - Channel x Destination Data Stride or Memory Set Pattern"]
pub type DDS_MSP_R = crate::FieldReader<u16>;
#[doc = "Field `DDS_MSP` writer - Channel x Destination Data Stride or Memory Set Pattern"]
pub type DDS_MSP_W<'a, const O: u8> = crate::FieldWriter<'a, CDS_MSP_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel x Source Data stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn sds_msp(&self) -> SDS_MSP_R {
        SDS_MSP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Channel x Destination Data Stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn dds_msp(&self) -> DDS_MSP_R {
        DDS_MSP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel x Source Data stride or Memory Set Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn sds_msp(&mut self) -> SDS_MSP_W<0> {
        SDS_MSP_W::new(self)
    }
    #[doc = "Bits 16:31 - Channel x Destination Data Stride or Memory Set Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn dds_msp(&mut self) -> DDS_MSP_W<16> {
        DDS_MSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Data Stride Memory Set Pattern\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp](index.html) module"]
pub struct CDS_MSP_SPEC;
impl crate::RegisterSpec for CDS_MSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cds_msp::R](R) reader structure"]
impl crate::Readable for CDS_MSP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cds_msp::W](W) writer structure"]
impl crate::Writable for CDS_MSP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDS_MSP to value 0"]
impl crate::Resettable for CDS_MSP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
