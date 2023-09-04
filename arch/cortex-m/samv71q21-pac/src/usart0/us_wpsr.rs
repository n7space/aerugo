#[doc = "Register `US_WPSR` reader"]
pub type R = crate::R<US_WPSR_SPEC>;
#[doc = "Field `WPVS` reader - Write Protection Violation Status"]
pub type WPVS_R = crate::BitReader;
#[doc = "Field `WPVSRC` reader - Write Protection Violation Source"]
pub type WPVSRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:23 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "Write Protection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`us_wpsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct US_WPSR_SPEC;
impl crate::RegisterSpec for US_WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`us_wpsr::R`](R) reader structure"]
impl crate::Readable for US_WPSR_SPEC {}
#[doc = "`reset()` method sets US_WPSR to value 0"]
impl crate::Resettable for US_WPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
