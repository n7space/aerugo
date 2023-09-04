#[doc = "Register `GRWS` writer"]
pub type W = crate::W<GRWS_SPEC>;
#[doc = "Field `RWS0` writer - XDMAC Channel 0 Read Write Suspend Bit"]
pub type RWS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS1` writer - XDMAC Channel 1 Read Write Suspend Bit"]
pub type RWS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS2` writer - XDMAC Channel 2 Read Write Suspend Bit"]
pub type RWS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS3` writer - XDMAC Channel 3 Read Write Suspend Bit"]
pub type RWS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS4` writer - XDMAC Channel 4 Read Write Suspend Bit"]
pub type RWS4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS5` writer - XDMAC Channel 5 Read Write Suspend Bit"]
pub type RWS5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS6` writer - XDMAC Channel 6 Read Write Suspend Bit"]
pub type RWS6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS7` writer - XDMAC Channel 7 Read Write Suspend Bit"]
pub type RWS7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS8` writer - XDMAC Channel 8 Read Write Suspend Bit"]
pub type RWS8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS9` writer - XDMAC Channel 9 Read Write Suspend Bit"]
pub type RWS9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS10` writer - XDMAC Channel 10 Read Write Suspend Bit"]
pub type RWS10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS11` writer - XDMAC Channel 11 Read Write Suspend Bit"]
pub type RWS11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS12` writer - XDMAC Channel 12 Read Write Suspend Bit"]
pub type RWS12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS13` writer - XDMAC Channel 13 Read Write Suspend Bit"]
pub type RWS13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS14` writer - XDMAC Channel 14 Read Write Suspend Bit"]
pub type RWS14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS15` writer - XDMAC Channel 15 Read Write Suspend Bit"]
pub type RWS15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS16` writer - XDMAC Channel 16 Read Write Suspend Bit"]
pub type RWS16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS17` writer - XDMAC Channel 17 Read Write Suspend Bit"]
pub type RWS17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS18` writer - XDMAC Channel 18 Read Write Suspend Bit"]
pub type RWS18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS19` writer - XDMAC Channel 19 Read Write Suspend Bit"]
pub type RWS19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS20` writer - XDMAC Channel 20 Read Write Suspend Bit"]
pub type RWS20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS21` writer - XDMAC Channel 21 Read Write Suspend Bit"]
pub type RWS21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS22` writer - XDMAC Channel 22 Read Write Suspend Bit"]
pub type RWS22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWS23` writer - XDMAC Channel 23 Read Write Suspend Bit"]
pub type RWS23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws0(&mut self) -> RWS0_W<GRWS_SPEC, 0> {
        RWS0_W::new(self)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws1(&mut self) -> RWS1_W<GRWS_SPEC, 1> {
        RWS1_W::new(self)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws2(&mut self) -> RWS2_W<GRWS_SPEC, 2> {
        RWS2_W::new(self)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws3(&mut self) -> RWS3_W<GRWS_SPEC, 3> {
        RWS3_W::new(self)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws4(&mut self) -> RWS4_W<GRWS_SPEC, 4> {
        RWS4_W::new(self)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws5(&mut self) -> RWS5_W<GRWS_SPEC, 5> {
        RWS5_W::new(self)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws6(&mut self) -> RWS6_W<GRWS_SPEC, 6> {
        RWS6_W::new(self)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws7(&mut self) -> RWS7_W<GRWS_SPEC, 7> {
        RWS7_W::new(self)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws8(&mut self) -> RWS8_W<GRWS_SPEC, 8> {
        RWS8_W::new(self)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws9(&mut self) -> RWS9_W<GRWS_SPEC, 9> {
        RWS9_W::new(self)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws10(&mut self) -> RWS10_W<GRWS_SPEC, 10> {
        RWS10_W::new(self)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws11(&mut self) -> RWS11_W<GRWS_SPEC, 11> {
        RWS11_W::new(self)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws12(&mut self) -> RWS12_W<GRWS_SPEC, 12> {
        RWS12_W::new(self)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws13(&mut self) -> RWS13_W<GRWS_SPEC, 13> {
        RWS13_W::new(self)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws14(&mut self) -> RWS14_W<GRWS_SPEC, 14> {
        RWS14_W::new(self)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws15(&mut self) -> RWS15_W<GRWS_SPEC, 15> {
        RWS15_W::new(self)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws16(&mut self) -> RWS16_W<GRWS_SPEC, 16> {
        RWS16_W::new(self)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws17(&mut self) -> RWS17_W<GRWS_SPEC, 17> {
        RWS17_W::new(self)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws18(&mut self) -> RWS18_W<GRWS_SPEC, 18> {
        RWS18_W::new(self)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws19(&mut self) -> RWS19_W<GRWS_SPEC, 19> {
        RWS19_W::new(self)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws20(&mut self) -> RWS20_W<GRWS_SPEC, 20> {
        RWS20_W::new(self)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws21(&mut self) -> RWS21_W<GRWS_SPEC, 21> {
        RWS21_W::new(self)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws22(&mut self) -> RWS22_W<GRWS_SPEC, 22> {
        RWS22_W::new(self)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Write Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rws23(&mut self) -> RWS23_W<GRWS_SPEC, 23> {
        RWS23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Global Channel Read Write Suspend Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grws::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRWS_SPEC;
impl crate::RegisterSpec for GRWS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`grws::W`](W) writer structure"]
impl crate::Writable for GRWS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GRWS to value 0"]
impl crate::Resettable for GRWS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
