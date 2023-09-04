#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `ALMS` reader - Real-time Alarm Status (cleared on read)"]
pub type ALMS_R = crate::BitReader;
#[doc = "Field `RTTINC` reader - Prescaler Roll-over Status (cleared on read)"]
pub type RTTINC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Real-time Alarm Status (cleared on read)"]
    #[inline(always)]
    pub fn alms(&self) -> ALMS_R {
        ALMS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prescaler Roll-over Status (cleared on read)"]
    #[inline(always)]
    pub fn rttinc(&self) -> RTTINC_R {
        RTTINC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
