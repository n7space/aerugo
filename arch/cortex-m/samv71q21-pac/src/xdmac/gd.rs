#[doc = "Register `GD` writer"]
pub type W = crate::W<GD_SPEC>;
#[doc = "Field `DI0` writer - XDMAC Channel 0 Disable Bit"]
pub type DI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI1` writer - XDMAC Channel 1 Disable Bit"]
pub type DI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI2` writer - XDMAC Channel 2 Disable Bit"]
pub type DI2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI3` writer - XDMAC Channel 3 Disable Bit"]
pub type DI3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI4` writer - XDMAC Channel 4 Disable Bit"]
pub type DI4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI5` writer - XDMAC Channel 5 Disable Bit"]
pub type DI5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI6` writer - XDMAC Channel 6 Disable Bit"]
pub type DI6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI7` writer - XDMAC Channel 7 Disable Bit"]
pub type DI7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI8` writer - XDMAC Channel 8 Disable Bit"]
pub type DI8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI9` writer - XDMAC Channel 9 Disable Bit"]
pub type DI9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI10` writer - XDMAC Channel 10 Disable Bit"]
pub type DI10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI11` writer - XDMAC Channel 11 Disable Bit"]
pub type DI11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI12` writer - XDMAC Channel 12 Disable Bit"]
pub type DI12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI13` writer - XDMAC Channel 13 Disable Bit"]
pub type DI13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI14` writer - XDMAC Channel 14 Disable Bit"]
pub type DI14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI15` writer - XDMAC Channel 15 Disable Bit"]
pub type DI15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI16` writer - XDMAC Channel 16 Disable Bit"]
pub type DI16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI17` writer - XDMAC Channel 17 Disable Bit"]
pub type DI17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI18` writer - XDMAC Channel 18 Disable Bit"]
pub type DI18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI19` writer - XDMAC Channel 19 Disable Bit"]
pub type DI19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI20` writer - XDMAC Channel 20 Disable Bit"]
pub type DI20_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI21` writer - XDMAC Channel 21 Disable Bit"]
pub type DI21_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI22` writer - XDMAC Channel 22 Disable Bit"]
pub type DI22_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI23` writer - XDMAC Channel 23 Disable Bit"]
pub type DI23_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di0(&mut self) -> DI0_W<GD_SPEC, 0> {
        DI0_W::new(self)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di1(&mut self) -> DI1_W<GD_SPEC, 1> {
        DI1_W::new(self)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di2(&mut self) -> DI2_W<GD_SPEC, 2> {
        DI2_W::new(self)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di3(&mut self) -> DI3_W<GD_SPEC, 3> {
        DI3_W::new(self)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di4(&mut self) -> DI4_W<GD_SPEC, 4> {
        DI4_W::new(self)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di5(&mut self) -> DI5_W<GD_SPEC, 5> {
        DI5_W::new(self)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di6(&mut self) -> DI6_W<GD_SPEC, 6> {
        DI6_W::new(self)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di7(&mut self) -> DI7_W<GD_SPEC, 7> {
        DI7_W::new(self)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di8(&mut self) -> DI8_W<GD_SPEC, 8> {
        DI8_W::new(self)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di9(&mut self) -> DI9_W<GD_SPEC, 9> {
        DI9_W::new(self)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di10(&mut self) -> DI10_W<GD_SPEC, 10> {
        DI10_W::new(self)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di11(&mut self) -> DI11_W<GD_SPEC, 11> {
        DI11_W::new(self)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di12(&mut self) -> DI12_W<GD_SPEC, 12> {
        DI12_W::new(self)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di13(&mut self) -> DI13_W<GD_SPEC, 13> {
        DI13_W::new(self)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di14(&mut self) -> DI14_W<GD_SPEC, 14> {
        DI14_W::new(self)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di15(&mut self) -> DI15_W<GD_SPEC, 15> {
        DI15_W::new(self)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di16(&mut self) -> DI16_W<GD_SPEC, 16> {
        DI16_W::new(self)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di17(&mut self) -> DI17_W<GD_SPEC, 17> {
        DI17_W::new(self)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di18(&mut self) -> DI18_W<GD_SPEC, 18> {
        DI18_W::new(self)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di19(&mut self) -> DI19_W<GD_SPEC, 19> {
        DI19_W::new(self)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di20(&mut self) -> DI20_W<GD_SPEC, 20> {
        DI20_W::new(self)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di21(&mut self) -> DI21_W<GD_SPEC, 21> {
        DI21_W::new(self)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di22(&mut self) -> DI22_W<GD_SPEC, 22> {
        DI22_W::new(self)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Disable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn di23(&mut self) -> DI23_W<GD_SPEC, 23> {
        DI23_W::new(self)
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
#[doc = "Global Channel Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GD_SPEC;
impl crate::RegisterSpec for GD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gd::W`](W) writer structure"]
impl crate::Writable for GD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GD to value 0"]
impl crate::Resettable for GD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
