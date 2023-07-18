#[doc = "Register `CHER` writer"]
pub struct W(crate::W<CHER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CHER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` writer - Channel 0 Enable"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
#[doc = "Field `CH1` writer - Channel 1 Enable"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
#[doc = "Field `CH2` writer - Channel 2 Enable"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
#[doc = "Field `CH3` writer - Channel 3 Enable"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
#[doc = "Field `CH4` writer - Channel 4 Enable"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
#[doc = "Field `CH5` writer - Channel 5 Enable"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
#[doc = "Field `CH6` writer - Channel 6 Enable"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
#[doc = "Field `CH7` writer - Channel 7 Enable"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
#[doc = "Field `CH8` writer - Channel 8 Enable"]
pub type CH8_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
#[doc = "Field `CH9` writer - Channel 9 Enable"]
pub type CH9_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
#[doc = "Field `CH10` writer - Channel 10 Enable"]
pub type CH10_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
#[doc = "Field `CH11` writer - Channel 11 Enable"]
pub type CH11_W<'a, const O: u8> = crate::BitWriter<'a, CHER_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<8> {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<9> {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<10> {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<11> {
        CH11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cher](index.html) module"]
pub struct CHER_SPEC;
impl crate::RegisterSpec for CHER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cher::W](W) writer structure"]
impl crate::Writable for CHER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHER to value 0"]
impl crate::Resettable for CHER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
