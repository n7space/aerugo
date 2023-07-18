#[doc = "Register `GRWR` writer"]
pub struct W(crate::W<GRWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRWR_SPEC>;
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
impl From<crate::W<GRWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWR0` writer - XDMAC Channel 0 Read Write Resume Bit"]
pub type RWR0_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR1` writer - XDMAC Channel 1 Read Write Resume Bit"]
pub type RWR1_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR2` writer - XDMAC Channel 2 Read Write Resume Bit"]
pub type RWR2_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR3` writer - XDMAC Channel 3 Read Write Resume Bit"]
pub type RWR3_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR4` writer - XDMAC Channel 4 Read Write Resume Bit"]
pub type RWR4_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR5` writer - XDMAC Channel 5 Read Write Resume Bit"]
pub type RWR5_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR6` writer - XDMAC Channel 6 Read Write Resume Bit"]
pub type RWR6_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR7` writer - XDMAC Channel 7 Read Write Resume Bit"]
pub type RWR7_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR8` writer - XDMAC Channel 8 Read Write Resume Bit"]
pub type RWR8_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR9` writer - XDMAC Channel 9 Read Write Resume Bit"]
pub type RWR9_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR10` writer - XDMAC Channel 10 Read Write Resume Bit"]
pub type RWR10_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR11` writer - XDMAC Channel 11 Read Write Resume Bit"]
pub type RWR11_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR12` writer - XDMAC Channel 12 Read Write Resume Bit"]
pub type RWR12_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR13` writer - XDMAC Channel 13 Read Write Resume Bit"]
pub type RWR13_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR14` writer - XDMAC Channel 14 Read Write Resume Bit"]
pub type RWR14_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR15` writer - XDMAC Channel 15 Read Write Resume Bit"]
pub type RWR15_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR16` writer - XDMAC Channel 16 Read Write Resume Bit"]
pub type RWR16_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR17` writer - XDMAC Channel 17 Read Write Resume Bit"]
pub type RWR17_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR18` writer - XDMAC Channel 18 Read Write Resume Bit"]
pub type RWR18_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR19` writer - XDMAC Channel 19 Read Write Resume Bit"]
pub type RWR19_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR20` writer - XDMAC Channel 20 Read Write Resume Bit"]
pub type RWR20_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR21` writer - XDMAC Channel 21 Read Write Resume Bit"]
pub type RWR21_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR22` writer - XDMAC Channel 22 Read Write Resume Bit"]
pub type RWR22_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
#[doc = "Field `RWR23` writer - XDMAC Channel 23 Read Write Resume Bit"]
pub type RWR23_W<'a, const O: u8> = crate::BitWriter<'a, GRWR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr0(&mut self) -> RWR0_W<0> {
        RWR0_W::new(self)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr1(&mut self) -> RWR1_W<1> {
        RWR1_W::new(self)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr2(&mut self) -> RWR2_W<2> {
        RWR2_W::new(self)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr3(&mut self) -> RWR3_W<3> {
        RWR3_W::new(self)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr4(&mut self) -> RWR4_W<4> {
        RWR4_W::new(self)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr5(&mut self) -> RWR5_W<5> {
        RWR5_W::new(self)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr6(&mut self) -> RWR6_W<6> {
        RWR6_W::new(self)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr7(&mut self) -> RWR7_W<7> {
        RWR7_W::new(self)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr8(&mut self) -> RWR8_W<8> {
        RWR8_W::new(self)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr9(&mut self) -> RWR9_W<9> {
        RWR9_W::new(self)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr10(&mut self) -> RWR10_W<10> {
        RWR10_W::new(self)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr11(&mut self) -> RWR11_W<11> {
        RWR11_W::new(self)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr12(&mut self) -> RWR12_W<12> {
        RWR12_W::new(self)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr13(&mut self) -> RWR13_W<13> {
        RWR13_W::new(self)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr14(&mut self) -> RWR14_W<14> {
        RWR14_W::new(self)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr15(&mut self) -> RWR15_W<15> {
        RWR15_W::new(self)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr16(&mut self) -> RWR16_W<16> {
        RWR16_W::new(self)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr17(&mut self) -> RWR17_W<17> {
        RWR17_W::new(self)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr18(&mut self) -> RWR18_W<18> {
        RWR18_W::new(self)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr19(&mut self) -> RWR19_W<19> {
        RWR19_W::new(self)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr20(&mut self) -> RWR20_W<20> {
        RWR20_W::new(self)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr21(&mut self) -> RWR21_W<21> {
        RWR21_W::new(self)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr22(&mut self) -> RWR22_W<22> {
        RWR22_W::new(self)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Write Resume Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwr23(&mut self) -> RWR23_W<23> {
        RWR23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Read Write Resume Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grwr](index.html) module"]
pub struct GRWR_SPEC;
impl crate::RegisterSpec for GRWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [grwr::W](W) writer structure"]
impl crate::Writable for GRWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GRWR to value 0"]
impl crate::Resettable for GRWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
