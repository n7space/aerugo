#[doc = "Register `CCNT` reader"]
pub type R = crate::R<CCNT_SPEC>;
#[doc = "Field `CNT` reader - Channel Counter Register"]
pub type CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PWM Channel Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCNT_SPEC;
impl crate::RegisterSpec for CCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccnt::R`](R) reader structure"]
impl crate::Readable for CCNT_SPEC {}
#[doc = "`reset()` method sets CCNT to value 0"]
impl crate::Resettable for CCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
