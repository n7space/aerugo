#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Field `ACK` reader - Acknowledge Update Interrupt Mask"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ALR` reader - Alarm Interrupt Mask"]
pub type ALR_R = crate::BitReader;
#[doc = "Field `SEC` reader - Second Event Interrupt Mask"]
pub type SEC_R = crate::BitReader;
#[doc = "Field `TIM` reader - Time Event Interrupt Mask"]
pub type TIM_R = crate::BitReader;
#[doc = "Field `CAL` reader - Calendar Event Interrupt Mask"]
pub type CAL_R = crate::BitReader;
#[doc = "Field `TDERR` reader - Time and/or Date Error Mask"]
pub type TDERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Acknowledge Update Interrupt Mask"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn alr(&self) -> ALR_R {
        ALR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Second Event Interrupt Mask"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time Event Interrupt Mask"]
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Calendar Event Interrupt Mask"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time and/or Date Error Mask"]
    #[inline(always)]
    pub fn tderr(&self) -> TDERR_R {
        TDERR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
