#[doc = "Register `CDS_MSP` reader"]
pub type R = crate::R<CDS_MSP_SPEC>;
#[doc = "Register `CDS_MSP` writer"]
pub type W = crate::W<CDS_MSP_SPEC>;
#[doc = "Field `SDS_MSP` reader - Channel x Source Data stride or Memory Set Pattern"]
pub type SDS_MSP_R = crate::FieldReader<u16>;
#[doc = "Field `SDS_MSP` writer - Channel x Source Data stride or Memory Set Pattern"]
pub type SDS_MSP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `DDS_MSP` reader - Channel x Destination Data Stride or Memory Set Pattern"]
pub type DDS_MSP_R = crate::FieldReader<u16>;
#[doc = "Field `DDS_MSP` writer - Channel x Destination Data Stride or Memory Set Pattern"]
pub type DDS_MSP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
    pub fn sds_msp(&mut self) -> SDS_MSP_W<CDS_MSP_SPEC, 0> {
        SDS_MSP_W::new(self)
    }
    #[doc = "Bits 16:31 - Channel x Destination Data Stride or Memory Set Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn dds_msp(&mut self) -> DDS_MSP_W<CDS_MSP_SPEC, 16> {
        DDS_MSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Data Stride Memory Set Pattern\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cds_msp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cds_msp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDS_MSP_SPEC;
impl crate::RegisterSpec for CDS_MSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cds_msp::R`](R) reader structure"]
impl crate::Readable for CDS_MSP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cds_msp::W`](W) writer structure"]
impl crate::Writable for CDS_MSP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDS_MSP to value 0"]
impl crate::Resettable for CDS_MSP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
