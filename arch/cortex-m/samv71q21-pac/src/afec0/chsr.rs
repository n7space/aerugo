#[doc = "Register `CHSR` reader"]
pub type R = crate::R<CHSR_SPEC>;
#[doc = "Field `CH0` reader - Channel 0 Status"]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH1` reader - Channel 1 Status"]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH2` reader - Channel 2 Status"]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH3` reader - Channel 3 Status"]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH4` reader - Channel 4 Status"]
pub type CH4_R = crate::BitReader;
#[doc = "Field `CH5` reader - Channel 5 Status"]
pub type CH5_R = crate::BitReader;
#[doc = "Field `CH6` reader - Channel 6 Status"]
pub type CH6_R = crate::BitReader;
#[doc = "Field `CH7` reader - Channel 7 Status"]
pub type CH7_R = crate::BitReader;
#[doc = "Field `CH8` reader - Channel 8 Status"]
pub type CH8_R = crate::BitReader;
#[doc = "Field `CH9` reader - Channel 9 Status"]
pub type CH9_R = crate::BitReader;
#[doc = "Field `CH10` reader - Channel 10 Status"]
pub type CH10_R = crate::BitReader;
#[doc = "Field `CH11` reader - Channel 11 Status"]
pub type CH11_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Status"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Status"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Status"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Status"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Status"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Status"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Status"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Status"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Status"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Status"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Status"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Status"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "AFEC Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSR_SPEC;
impl crate::RegisterSpec for CHSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsr::R`](R) reader structure"]
impl crate::Readable for CHSR_SPEC {}
#[doc = "`reset()` method sets CHSR to value 0"]
impl crate::Resettable for CHSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
