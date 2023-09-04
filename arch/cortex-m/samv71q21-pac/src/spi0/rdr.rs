#[doc = "Register `RDR` reader"]
pub type R = crate::R<RDR_SPEC>;
#[doc = "Field `RD` reader - Receive Data"]
pub type RD_R = crate::FieldReader<u16>;
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub type PCS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Receive Data"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RDR_SPEC {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
