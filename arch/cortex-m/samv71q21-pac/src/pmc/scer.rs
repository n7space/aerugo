#[doc = "Register `SCER` writer"]
pub type W = crate::W<SCER_SPEC>;
#[doc = "Field `USBCLK` writer - Enable USB FS Clock"]
pub type USBCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCK0` writer - Programmable Clock 0 Output Enable"]
pub type PCK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCK1` writer - Programmable Clock 1 Output Enable"]
pub type PCK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCK2` writer - Programmable Clock 2 Output Enable"]
pub type PCK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCK3` writer - Programmable Clock 3 Output Enable"]
pub type PCK3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCK4` writer - Programmable Clock 4 Output Enable"]
pub type PCK4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCK5` writer - Programmable Clock 5 Output Enable"]
pub type PCK5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCK6` writer - Programmable Clock 6 Output Enable"]
pub type PCK6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCK7` writer - Programmable Clock 7 Output Enable"]
pub type PCK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 5 - Enable USB FS Clock"]
    #[inline(always)]
    #[must_use]
    pub fn usbclk(&mut self) -> USBCLK_W<SCER_SPEC, 5> {
        USBCLK_W::new(self)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pck0(&mut self) -> PCK0_W<SCER_SPEC, 8> {
        PCK0_W::new(self)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pck1(&mut self) -> PCK1_W<SCER_SPEC, 9> {
        PCK1_W::new(self)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pck2(&mut self) -> PCK2_W<SCER_SPEC, 10> {
        PCK2_W::new(self)
    }
    #[doc = "Bit 11 - Programmable Clock 3 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pck3(&mut self) -> PCK3_W<SCER_SPEC, 11> {
        PCK3_W::new(self)
    }
    #[doc = "Bit 12 - Programmable Clock 4 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pck4(&mut self) -> PCK4_W<SCER_SPEC, 12> {
        PCK4_W::new(self)
    }
    #[doc = "Bit 13 - Programmable Clock 5 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pck5(&mut self) -> PCK5_W<SCER_SPEC, 13> {
        PCK5_W::new(self)
    }
    #[doc = "Bit 14 - Programmable Clock 6 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pck6(&mut self) -> PCK6_W<SCER_SPEC, 14> {
        PCK6_W::new(self)
    }
    #[doc = "Bit 15 - Programmable Clock 7 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pck7(&mut self) -> PCK7_W<SCER_SPEC, 15> {
        PCK7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "System Clock Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scer::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCER_SPEC;
impl crate::RegisterSpec for SCER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scer::W`](W) writer structure"]
impl crate::Writable for SCER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCER to value 0"]
impl crate::Resettable for SCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
