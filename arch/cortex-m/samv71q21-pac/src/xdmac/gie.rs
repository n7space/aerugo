#[doc = "Register `GIE` writer"]
pub type W = crate::W<GIE_SPEC>;
#[doc = "Field `IE0` writer - XDMAC Channel 0 Interrupt Enable Bit"]
pub type IE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE1` writer - XDMAC Channel 1 Interrupt Enable Bit"]
pub type IE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE2` writer - XDMAC Channel 2 Interrupt Enable Bit"]
pub type IE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE3` writer - XDMAC Channel 3 Interrupt Enable Bit"]
pub type IE3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE4` writer - XDMAC Channel 4 Interrupt Enable Bit"]
pub type IE4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE5` writer - XDMAC Channel 5 Interrupt Enable Bit"]
pub type IE5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE6` writer - XDMAC Channel 6 Interrupt Enable Bit"]
pub type IE6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE7` writer - XDMAC Channel 7 Interrupt Enable Bit"]
pub type IE7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE8` writer - XDMAC Channel 8 Interrupt Enable Bit"]
pub type IE8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE9` writer - XDMAC Channel 9 Interrupt Enable Bit"]
pub type IE9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE10` writer - XDMAC Channel 10 Interrupt Enable Bit"]
pub type IE10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE11` writer - XDMAC Channel 11 Interrupt Enable Bit"]
pub type IE11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE12` writer - XDMAC Channel 12 Interrupt Enable Bit"]
pub type IE12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE13` writer - XDMAC Channel 13 Interrupt Enable Bit"]
pub type IE13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE14` writer - XDMAC Channel 14 Interrupt Enable Bit"]
pub type IE14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE15` writer - XDMAC Channel 15 Interrupt Enable Bit"]
pub type IE15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE16` writer - XDMAC Channel 16 Interrupt Enable Bit"]
pub type IE16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE17` writer - XDMAC Channel 17 Interrupt Enable Bit"]
pub type IE17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE18` writer - XDMAC Channel 18 Interrupt Enable Bit"]
pub type IE18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE19` writer - XDMAC Channel 19 Interrupt Enable Bit"]
pub type IE19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE20` writer - XDMAC Channel 20 Interrupt Enable Bit"]
pub type IE20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE21` writer - XDMAC Channel 21 Interrupt Enable Bit"]
pub type IE21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE22` writer - XDMAC Channel 22 Interrupt Enable Bit"]
pub type IE22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IE23` writer - XDMAC Channel 23 Interrupt Enable Bit"]
pub type IE23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie0(&mut self) -> IE0_W<GIE_SPEC, 0> {
        IE0_W::new(self)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie1(&mut self) -> IE1_W<GIE_SPEC, 1> {
        IE1_W::new(self)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie2(&mut self) -> IE2_W<GIE_SPEC, 2> {
        IE2_W::new(self)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie3(&mut self) -> IE3_W<GIE_SPEC, 3> {
        IE3_W::new(self)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie4(&mut self) -> IE4_W<GIE_SPEC, 4> {
        IE4_W::new(self)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie5(&mut self) -> IE5_W<GIE_SPEC, 5> {
        IE5_W::new(self)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie6(&mut self) -> IE6_W<GIE_SPEC, 6> {
        IE6_W::new(self)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie7(&mut self) -> IE7_W<GIE_SPEC, 7> {
        IE7_W::new(self)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie8(&mut self) -> IE8_W<GIE_SPEC, 8> {
        IE8_W::new(self)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie9(&mut self) -> IE9_W<GIE_SPEC, 9> {
        IE9_W::new(self)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie10(&mut self) -> IE10_W<GIE_SPEC, 10> {
        IE10_W::new(self)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie11(&mut self) -> IE11_W<GIE_SPEC, 11> {
        IE11_W::new(self)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie12(&mut self) -> IE12_W<GIE_SPEC, 12> {
        IE12_W::new(self)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie13(&mut self) -> IE13_W<GIE_SPEC, 13> {
        IE13_W::new(self)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie14(&mut self) -> IE14_W<GIE_SPEC, 14> {
        IE14_W::new(self)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie15(&mut self) -> IE15_W<GIE_SPEC, 15> {
        IE15_W::new(self)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie16(&mut self) -> IE16_W<GIE_SPEC, 16> {
        IE16_W::new(self)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie17(&mut self) -> IE17_W<GIE_SPEC, 17> {
        IE17_W::new(self)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie18(&mut self) -> IE18_W<GIE_SPEC, 18> {
        IE18_W::new(self)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie19(&mut self) -> IE19_W<GIE_SPEC, 19> {
        IE19_W::new(self)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie20(&mut self) -> IE20_W<GIE_SPEC, 20> {
        IE20_W::new(self)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie21(&mut self) -> IE21_W<GIE_SPEC, 21> {
        IE21_W::new(self)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie22(&mut self) -> IE22_W<GIE_SPEC, 22> {
        IE22_W::new(self)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ie23(&mut self) -> IE23_W<GIE_SPEC, 23> {
        IE23_W::new(self)
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
#[doc = "Global Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gie::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GIE_SPEC;
impl crate::RegisterSpec for GIE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gie::W`](W) writer structure"]
impl crate::Writable for GIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GIE to value 0"]
impl crate::Resettable for GIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
