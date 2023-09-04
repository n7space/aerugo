#[doc = "Register `HCER[%s]` reader"]
pub type R = crate::R<HCER_SPEC>;
#[doc = "Field `CERR` reader - Bitwise Channel Error Bit \\[31:0\\]"]
pub type CERR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Error Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn cerr(&self) -> CERR_R {
        CERR_R::new(self.bits)
    }
}
#[doc = "HBI Channel Error 0 Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcer::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCER_SPEC;
impl crate::RegisterSpec for HCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcer::R`](R) reader structure"]
impl crate::Readable for HCER_SPEC {}
#[doc = "`reset()` method sets HCER[%s]
to value 0"]
impl crate::Resettable for HCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
