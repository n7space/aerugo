#[doc = "Register `GSWR` writer"]
pub type W = crate::W<GSWR_SPEC>;
#[doc = "Field `SWREQ0` writer - XDMAC Channel 0 Software Request Bit"]
pub type SWREQ0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ1` writer - XDMAC Channel 1 Software Request Bit"]
pub type SWREQ1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ2` writer - XDMAC Channel 2 Software Request Bit"]
pub type SWREQ2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ3` writer - XDMAC Channel 3 Software Request Bit"]
pub type SWREQ3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ4` writer - XDMAC Channel 4 Software Request Bit"]
pub type SWREQ4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ5` writer - XDMAC Channel 5 Software Request Bit"]
pub type SWREQ5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ6` writer - XDMAC Channel 6 Software Request Bit"]
pub type SWREQ6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ7` writer - XDMAC Channel 7 Software Request Bit"]
pub type SWREQ7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ8` writer - XDMAC Channel 8 Software Request Bit"]
pub type SWREQ8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ9` writer - XDMAC Channel 9 Software Request Bit"]
pub type SWREQ9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ10` writer - XDMAC Channel 10 Software Request Bit"]
pub type SWREQ10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ11` writer - XDMAC Channel 11 Software Request Bit"]
pub type SWREQ11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ12` writer - XDMAC Channel 12 Software Request Bit"]
pub type SWREQ12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ13` writer - XDMAC Channel 13 Software Request Bit"]
pub type SWREQ13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ14` writer - XDMAC Channel 14 Software Request Bit"]
pub type SWREQ14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ15` writer - XDMAC Channel 15 Software Request Bit"]
pub type SWREQ15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ16` writer - XDMAC Channel 16 Software Request Bit"]
pub type SWREQ16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ17` writer - XDMAC Channel 17 Software Request Bit"]
pub type SWREQ17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ18` writer - XDMAC Channel 18 Software Request Bit"]
pub type SWREQ18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ19` writer - XDMAC Channel 19 Software Request Bit"]
pub type SWREQ19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ20` writer - XDMAC Channel 20 Software Request Bit"]
pub type SWREQ20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ21` writer - XDMAC Channel 21 Software Request Bit"]
pub type SWREQ21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ22` writer - XDMAC Channel 22 Software Request Bit"]
pub type SWREQ22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWREQ23` writer - XDMAC Channel 23 Software Request Bit"]
pub type SWREQ23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq0(&mut self) -> SWREQ0_W<GSWR_SPEC, 0> {
        SWREQ0_W::new(self)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq1(&mut self) -> SWREQ1_W<GSWR_SPEC, 1> {
        SWREQ1_W::new(self)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq2(&mut self) -> SWREQ2_W<GSWR_SPEC, 2> {
        SWREQ2_W::new(self)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq3(&mut self) -> SWREQ3_W<GSWR_SPEC, 3> {
        SWREQ3_W::new(self)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq4(&mut self) -> SWREQ4_W<GSWR_SPEC, 4> {
        SWREQ4_W::new(self)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq5(&mut self) -> SWREQ5_W<GSWR_SPEC, 5> {
        SWREQ5_W::new(self)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq6(&mut self) -> SWREQ6_W<GSWR_SPEC, 6> {
        SWREQ6_W::new(self)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq7(&mut self) -> SWREQ7_W<GSWR_SPEC, 7> {
        SWREQ7_W::new(self)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq8(&mut self) -> SWREQ8_W<GSWR_SPEC, 8> {
        SWREQ8_W::new(self)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq9(&mut self) -> SWREQ9_W<GSWR_SPEC, 9> {
        SWREQ9_W::new(self)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq10(&mut self) -> SWREQ10_W<GSWR_SPEC, 10> {
        SWREQ10_W::new(self)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq11(&mut self) -> SWREQ11_W<GSWR_SPEC, 11> {
        SWREQ11_W::new(self)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq12(&mut self) -> SWREQ12_W<GSWR_SPEC, 12> {
        SWREQ12_W::new(self)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq13(&mut self) -> SWREQ13_W<GSWR_SPEC, 13> {
        SWREQ13_W::new(self)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq14(&mut self) -> SWREQ14_W<GSWR_SPEC, 14> {
        SWREQ14_W::new(self)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq15(&mut self) -> SWREQ15_W<GSWR_SPEC, 15> {
        SWREQ15_W::new(self)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq16(&mut self) -> SWREQ16_W<GSWR_SPEC, 16> {
        SWREQ16_W::new(self)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq17(&mut self) -> SWREQ17_W<GSWR_SPEC, 17> {
        SWREQ17_W::new(self)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq18(&mut self) -> SWREQ18_W<GSWR_SPEC, 18> {
        SWREQ18_W::new(self)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq19(&mut self) -> SWREQ19_W<GSWR_SPEC, 19> {
        SWREQ19_W::new(self)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq20(&mut self) -> SWREQ20_W<GSWR_SPEC, 20> {
        SWREQ20_W::new(self)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq21(&mut self) -> SWREQ21_W<GSWR_SPEC, 21> {
        SWREQ21_W::new(self)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq22(&mut self) -> SWREQ22_W<GSWR_SPEC, 22> {
        SWREQ22_W::new(self)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Software Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn swreq23(&mut self) -> SWREQ23_W<GSWR_SPEC, 23> {
        SWREQ23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Global Channel Software Request Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gswr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GSWR_SPEC;
impl crate::RegisterSpec for GSWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gswr::W`](W) writer structure"]
impl crate::Writable for GSWR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GSWR to value 0"]
impl crate::Resettable for GSWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
