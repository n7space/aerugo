#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `WDUNF` reader - Watchdog Underflow (cleared on read)"]
pub type WDUNF_R = crate::BitReader;
#[doc = "Field `WDERR` reader - Watchdog Error (cleared on read)"]
pub type WDERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Watchdog Underflow (cleared on read)"]
    #[inline(always)]
    pub fn wdunf(&self) -> WDUNF_R {
        WDUNF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Error (cleared on read)"]
    #[inline(always)]
    pub fn wderr(&self) -> WDERR_R {
        WDERR_R::new(((self.bits >> 1) & 1) != 0)
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
