#[doc = "Register `GE` writer"]
pub type W = crate::W<GE_SPEC>;
#[doc = "Field `EN0` writer - XDMAC Channel 0 Enable Bit"]
pub type EN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN1` writer - XDMAC Channel 1 Enable Bit"]
pub type EN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN2` writer - XDMAC Channel 2 Enable Bit"]
pub type EN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN3` writer - XDMAC Channel 3 Enable Bit"]
pub type EN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN4` writer - XDMAC Channel 4 Enable Bit"]
pub type EN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN5` writer - XDMAC Channel 5 Enable Bit"]
pub type EN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN6` writer - XDMAC Channel 6 Enable Bit"]
pub type EN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN7` writer - XDMAC Channel 7 Enable Bit"]
pub type EN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN8` writer - XDMAC Channel 8 Enable Bit"]
pub type EN8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN9` writer - XDMAC Channel 9 Enable Bit"]
pub type EN9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN10` writer - XDMAC Channel 10 Enable Bit"]
pub type EN10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN11` writer - XDMAC Channel 11 Enable Bit"]
pub type EN11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN12` writer - XDMAC Channel 12 Enable Bit"]
pub type EN12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN13` writer - XDMAC Channel 13 Enable Bit"]
pub type EN13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN14` writer - XDMAC Channel 14 Enable Bit"]
pub type EN14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN15` writer - XDMAC Channel 15 Enable Bit"]
pub type EN15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN16` writer - XDMAC Channel 16 Enable Bit"]
pub type EN16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN17` writer - XDMAC Channel 17 Enable Bit"]
pub type EN17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN18` writer - XDMAC Channel 18 Enable Bit"]
pub type EN18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN19` writer - XDMAC Channel 19 Enable Bit"]
pub type EN19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN20` writer - XDMAC Channel 20 Enable Bit"]
pub type EN20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN21` writer - XDMAC Channel 21 Enable Bit"]
pub type EN21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN22` writer - XDMAC Channel 22 Enable Bit"]
pub type EN22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN23` writer - XDMAC Channel 23 Enable Bit"]
pub type EN23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en0(&mut self) -> EN0_W<GE_SPEC, 0> {
        EN0_W::new(self)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<GE_SPEC, 1> {
        EN1_W::new(self)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<GE_SPEC, 2> {
        EN2_W::new(self)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en3(&mut self) -> EN3_W<GE_SPEC, 3> {
        EN3_W::new(self)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en4(&mut self) -> EN4_W<GE_SPEC, 4> {
        EN4_W::new(self)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en5(&mut self) -> EN5_W<GE_SPEC, 5> {
        EN5_W::new(self)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en6(&mut self) -> EN6_W<GE_SPEC, 6> {
        EN6_W::new(self)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en7(&mut self) -> EN7_W<GE_SPEC, 7> {
        EN7_W::new(self)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en8(&mut self) -> EN8_W<GE_SPEC, 8> {
        EN8_W::new(self)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en9(&mut self) -> EN9_W<GE_SPEC, 9> {
        EN9_W::new(self)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en10(&mut self) -> EN10_W<GE_SPEC, 10> {
        EN10_W::new(self)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en11(&mut self) -> EN11_W<GE_SPEC, 11> {
        EN11_W::new(self)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en12(&mut self) -> EN12_W<GE_SPEC, 12> {
        EN12_W::new(self)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en13(&mut self) -> EN13_W<GE_SPEC, 13> {
        EN13_W::new(self)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en14(&mut self) -> EN14_W<GE_SPEC, 14> {
        EN14_W::new(self)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en15(&mut self) -> EN15_W<GE_SPEC, 15> {
        EN15_W::new(self)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en16(&mut self) -> EN16_W<GE_SPEC, 16> {
        EN16_W::new(self)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en17(&mut self) -> EN17_W<GE_SPEC, 17> {
        EN17_W::new(self)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en18(&mut self) -> EN18_W<GE_SPEC, 18> {
        EN18_W::new(self)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en19(&mut self) -> EN19_W<GE_SPEC, 19> {
        EN19_W::new(self)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en20(&mut self) -> EN20_W<GE_SPEC, 20> {
        EN20_W::new(self)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en21(&mut self) -> EN21_W<GE_SPEC, 21> {
        EN21_W::new(self)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en22(&mut self) -> EN22_W<GE_SPEC, 22> {
        EN22_W::new(self)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en23(&mut self) -> EN23_W<GE_SPEC, 23> {
        EN23_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Global Channel Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ge::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GE_SPEC;
impl crate::RegisterSpec for GE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ge::W`](W) writer structure"]
impl crate::Writable for GE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GE to value 0"]
impl crate::Resettable for GE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
