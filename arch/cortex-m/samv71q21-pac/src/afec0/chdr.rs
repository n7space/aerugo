#[doc = "Register `CHDR` writer"]
pub type W = crate::W<CHDR_SPEC>;
#[doc = "Field `CH0` writer - Channel 0 Disable"]
pub type CH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1` writer - Channel 1 Disable"]
pub type CH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH2` writer - Channel 2 Disable"]
pub type CH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH3` writer - Channel 3 Disable"]
pub type CH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH4` writer - Channel 4 Disable"]
pub type CH4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH5` writer - Channel 5 Disable"]
pub type CH5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH6` writer - Channel 6 Disable"]
pub type CH6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH7` writer - Channel 7 Disable"]
pub type CH7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH8` writer - Channel 8 Disable"]
pub type CH8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH9` writer - Channel 9 Disable"]
pub type CH9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH10` writer - Channel 10 Disable"]
pub type CH10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH11` writer - Channel 11 Disable"]
pub type CH11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<CHDR_SPEC, 0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<CHDR_SPEC, 1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<CHDR_SPEC, 2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<CHDR_SPEC, 3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<CHDR_SPEC, 4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<CHDR_SPEC, 5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<CHDR_SPEC, 6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<CHDR_SPEC, 7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<CHDR_SPEC, 8> {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<CHDR_SPEC, 9> {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<CHDR_SPEC, 10> {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<CHDR_SPEC, 11> {
        CH11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AFEC Channel Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHDR_SPEC;
impl crate::RegisterSpec for CHDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chdr::W`](W) writer structure"]
impl crate::Writable for CHDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHDR to value 0"]
impl crate::Resettable for CHDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
