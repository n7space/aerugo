#[doc = "Register `TXLPI` reader"]
pub struct R(crate::R<TXLPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXLPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXLPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXLPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Count of LPI transitions (cleared on read)"]
pub type COUNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Count of LPI transitions (cleared on read)"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Transmit LPI Transitions\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txlpi](index.html) module"]
pub struct TXLPI_SPEC;
impl crate::RegisterSpec for TXLPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txlpi::R](R) reader structure"]
impl crate::Readable for TXLPI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXLPI to value 0"]
impl crate::Resettable for TXLPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
