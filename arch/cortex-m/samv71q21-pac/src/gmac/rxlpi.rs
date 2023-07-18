#[doc = "Register `RXLPI` reader"]
pub struct R(crate::R<RXLPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXLPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXLPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXLPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Count of RX LPI transitions (cleared on read)"]
pub type COUNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Count of RX LPI transitions (cleared on read)"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Received LPI Transitions\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxlpi](index.html) module"]
pub struct RXLPI_SPEC;
impl crate::RegisterSpec for RXLPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxlpi::R](R) reader structure"]
impl crate::Readable for RXLPI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXLPI to value 0"]
impl crate::Resettable for RXLPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
