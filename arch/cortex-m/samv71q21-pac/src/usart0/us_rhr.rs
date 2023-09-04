#[doc = "Register `US_RHR` reader"]
pub type R = crate::R<US_RHR_SPEC>;
#[doc = "Field `RXCHR` reader - Received Character"]
pub type RXCHR_R = crate::FieldReader<u16>;
#[doc = "Field `RXSYNH` reader - Received Sync"]
pub type RXSYNH_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Received Character"]
    #[inline(always)]
    pub fn rxchr(&self) -> RXCHR_R {
        RXCHR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Received Sync"]
    #[inline(always)]
    pub fn rxsynh(&self) -> RXSYNH_R {
        RXSYNH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Receive Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_rhr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_RHR_SPEC;
impl crate::RegisterSpec for US_RHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_rhr::R`](R) reader structure"]
impl crate::Readable for US_RHR_SPEC {}
#[doc = "`reset()` method sets US_RHR to value 0"]
impl crate::Resettable for US_RHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
