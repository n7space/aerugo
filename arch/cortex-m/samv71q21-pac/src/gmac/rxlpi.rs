#[doc = "Register `RXLPI` reader"]
pub type R = crate::R<RXLPI_SPEC>;
#[doc = "Field `COUNT` reader - Count of RX LPI transitions (cleared on read)"]
pub type COUNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Count of RX LPI transitions (cleared on read)"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Received LPI Transitions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlpi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXLPI_SPEC;
impl crate::RegisterSpec for RXLPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlpi::R`](R) reader structure"]
impl crate::Readable for RXLPI_SPEC {}
#[doc = "`reset()` method sets RXLPI to value 0"]
impl crate::Resettable for RXLPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
