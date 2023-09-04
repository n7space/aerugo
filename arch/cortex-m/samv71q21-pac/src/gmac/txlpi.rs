#[doc = "Register `TXLPI` reader"]
pub type R = crate::R<TXLPI_SPEC>;
#[doc = "Field `COUNT` reader - Count of LPI transitions (cleared on read)"]
pub type COUNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Count of LPI transitions (cleared on read)"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Transmit LPI Transitions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlpi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXLPI_SPEC;
impl crate::RegisterSpec for TXLPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlpi::R`](R) reader structure"]
impl crate::Readable for TXLPI_SPEC {}
#[doc = "`reset()` method sets TXLPI to value 0"]
impl crate::Resettable for TXLPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
