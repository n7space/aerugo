#[doc = "Register `HCBR[%s]` reader"]
pub type R = crate::R<HCBR_SPEC>;
#[doc = "Field `CHB` reader - Bitwise Channel Busy Bit \\[31:0\\]"]
pub type CHB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Busy Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn chb(&self) -> CHB_R {
        CHB_R::new(self.bits)
    }
}
#[doc = "HBI Channel Busy 0 Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcbr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCBR_SPEC;
impl crate::RegisterSpec for HCBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcbr::R`](R) reader structure"]
impl crate::Readable for HCBR_SPEC {}
#[doc = "`reset()` method sets HCBR[%s]
to value 0"]
impl crate::Resettable for HCBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
