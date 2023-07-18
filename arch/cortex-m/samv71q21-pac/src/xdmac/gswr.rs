#[doc = "Register `GSWR` writer"]
pub struct W(crate::W<GSWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GSWR_SPEC>;
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
impl From<crate::W<GSWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GSWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWREQ0` writer - XDMAC Channel 0 Software Request Bit"]
pub type SWREQ0_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ1` writer - XDMAC Channel 1 Software Request Bit"]
pub type SWREQ1_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ2` writer - XDMAC Channel 2 Software Request Bit"]
pub type SWREQ2_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ3` writer - XDMAC Channel 3 Software Request Bit"]
pub type SWREQ3_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ4` writer - XDMAC Channel 4 Software Request Bit"]
pub type SWREQ4_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ5` writer - XDMAC Channel 5 Software Request Bit"]
pub type SWREQ5_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ6` writer - XDMAC Channel 6 Software Request Bit"]
pub type SWREQ6_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ7` writer - XDMAC Channel 7 Software Request Bit"]
pub type SWREQ7_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ8` writer - XDMAC Channel 8 Software Request Bit"]
pub type SWREQ8_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ9` writer - XDMAC Channel 9 Software Request Bit"]
pub type SWREQ9_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ10` writer - XDMAC Channel 10 Software Request Bit"]
pub type SWREQ10_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ11` writer - XDMAC Channel 11 Software Request Bit"]
pub type SWREQ11_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ12` writer - XDMAC Channel 12 Software Request Bit"]
pub type SWREQ12_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ13` writer - XDMAC Channel 13 Software Request Bit"]
pub type SWREQ13_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ14` writer - XDMAC Channel 14 Software Request Bit"]
pub type SWREQ14_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ15` writer - XDMAC Channel 15 Software Request Bit"]
pub type SWREQ15_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ16` writer - XDMAC Channel 16 Software Request Bit"]
pub type SWREQ16_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ17` writer - XDMAC Channel 17 Software Request Bit"]
pub type SWREQ17_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ18` writer - XDMAC Channel 18 Software Request Bit"]
pub type SWREQ18_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ19` writer - XDMAC Channel 19 Software Request Bit"]
pub type SWREQ19_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ20` writer - XDMAC Channel 20 Software Request Bit"]
pub type SWREQ20_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ21` writer - XDMAC Channel 21 Software Request Bit"]
pub type SWREQ21_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ22` writer - XDMAC Channel 22 Software Request Bit"]
pub type SWREQ22_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
#[doc = "Field `SWREQ23` writer - XDMAC Channel 23 Software Request Bit"]
pub type SWREQ23_W<'a, const O: u8> = crate::BitWriter<'a, GSWR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq0(&mut self) -> SWREQ0_W<0> {
        SWREQ0_W::new(self)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq1(&mut self) -> SWREQ1_W<1> {
        SWREQ1_W::new(self)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq2(&mut self) -> SWREQ2_W<2> {
        SWREQ2_W::new(self)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq3(&mut self) -> SWREQ3_W<3> {
        SWREQ3_W::new(self)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq4(&mut self) -> SWREQ4_W<4> {
        SWREQ4_W::new(self)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq5(&mut self) -> SWREQ5_W<5> {
        SWREQ5_W::new(self)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq6(&mut self) -> SWREQ6_W<6> {
        SWREQ6_W::new(self)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq7(&mut self) -> SWREQ7_W<7> {
        SWREQ7_W::new(self)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq8(&mut self) -> SWREQ8_W<8> {
        SWREQ8_W::new(self)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq9(&mut self) -> SWREQ9_W<9> {
        SWREQ9_W::new(self)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq10(&mut self) -> SWREQ10_W<10> {
        SWREQ10_W::new(self)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq11(&mut self) -> SWREQ11_W<11> {
        SWREQ11_W::new(self)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq12(&mut self) -> SWREQ12_W<12> {
        SWREQ12_W::new(self)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq13(&mut self) -> SWREQ13_W<13> {
        SWREQ13_W::new(self)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq14(&mut self) -> SWREQ14_W<14> {
        SWREQ14_W::new(self)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq15(&mut self) -> SWREQ15_W<15> {
        SWREQ15_W::new(self)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq16(&mut self) -> SWREQ16_W<16> {
        SWREQ16_W::new(self)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq17(&mut self) -> SWREQ17_W<17> {
        SWREQ17_W::new(self)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq18(&mut self) -> SWREQ18_W<18> {
        SWREQ18_W::new(self)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq19(&mut self) -> SWREQ19_W<19> {
        SWREQ19_W::new(self)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq20(&mut self) -> SWREQ20_W<20> {
        SWREQ20_W::new(self)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq21(&mut self) -> SWREQ21_W<21> {
        SWREQ21_W::new(self)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq22(&mut self) -> SWREQ22_W<22> {
        SWREQ22_W::new(self)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq23(&mut self) -> SWREQ23_W<23> {
        SWREQ23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Software Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gswr](index.html) module"]
pub struct GSWR_SPEC;
impl crate::RegisterSpec for GSWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gswr::W](W) writer structure"]
impl crate::Writable for GSWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GSWR to value 0"]
impl crate::Resettable for GSWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
