#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `CHID0` reader - Channel ID"]
pub type CHID0_R = crate::BitReader;
#[doc = "Field `CHID1` reader - Channel ID"]
pub type CHID1_R = crate::BitReader;
#[doc = "Field `CHID2` reader - Channel ID"]
pub type CHID2_R = crate::BitReader;
#[doc = "Field `CHID3` reader - Channel ID"]
pub type CHID3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel ID"]
    #[inline(always)]
    pub fn chid0(&self) -> CHID0_R {
        CHID0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel ID"]
    #[inline(always)]
    pub fn chid1(&self) -> CHID1_R {
        CHID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel ID"]
    #[inline(always)]
    pub fn chid2(&self) -> CHID2_R {
        CHID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel ID"]
    #[inline(always)]
    pub fn chid3(&self) -> CHID3_R {
        CHID3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "PWM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
