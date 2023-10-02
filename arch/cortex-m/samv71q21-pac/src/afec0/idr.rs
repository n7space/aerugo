#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Disable 0"]
pub type EOC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Disable 1"]
pub type EOC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC2` writer - End of Conversion Interrupt Disable 2"]
pub type EOC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC3` writer - End of Conversion Interrupt Disable 3"]
pub type EOC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC4` writer - End of Conversion Interrupt Disable 4"]
pub type EOC4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC5` writer - End of Conversion Interrupt Disable 5"]
pub type EOC5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC6` writer - End of Conversion Interrupt Disable 6"]
pub type EOC6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC7` writer - End of Conversion Interrupt Disable 7"]
pub type EOC7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC8` writer - End of Conversion Interrupt Disable 8"]
pub type EOC8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC9` writer - End of Conversion Interrupt Disable 9"]
pub type EOC9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC10` writer - End of Conversion Interrupt Disable 10"]
pub type EOC10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOC11` writer - End of Conversion Interrupt Disable 11"]
pub type EOC11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRDY` writer - Data Ready Interrupt Disable"]
pub type DRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GOVRE` writer - General Overrun Error Interrupt Disable"]
pub type GOVRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMPE` writer - Comparison Event Interrupt Disable"]
pub type COMPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEMPCHG` writer - Temperature Change Interrupt Disable"]
pub type TEMPCHG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - End of Conversion Interrupt Disable 0"]
    #[inline(always)]
    #[must_use]
    pub fn eoc0(&mut self) -> EOC0_W<IDR_SPEC, 0> {
        EOC0_W::new(self)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Disable 1"]
    #[inline(always)]
    #[must_use]
    pub fn eoc1(&mut self) -> EOC1_W<IDR_SPEC, 1> {
        EOC1_W::new(self)
    }
    #[doc = "Bit 2 - End of Conversion Interrupt Disable 2"]
    #[inline(always)]
    #[must_use]
    pub fn eoc2(&mut self) -> EOC2_W<IDR_SPEC, 2> {
        EOC2_W::new(self)
    }
    #[doc = "Bit 3 - End of Conversion Interrupt Disable 3"]
    #[inline(always)]
    #[must_use]
    pub fn eoc3(&mut self) -> EOC3_W<IDR_SPEC, 3> {
        EOC3_W::new(self)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Disable 4"]
    #[inline(always)]
    #[must_use]
    pub fn eoc4(&mut self) -> EOC4_W<IDR_SPEC, 4> {
        EOC4_W::new(self)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Disable 5"]
    #[inline(always)]
    #[must_use]
    pub fn eoc5(&mut self) -> EOC5_W<IDR_SPEC, 5> {
        EOC5_W::new(self)
    }
    #[doc = "Bit 6 - End of Conversion Interrupt Disable 6"]
    #[inline(always)]
    #[must_use]
    pub fn eoc6(&mut self) -> EOC6_W<IDR_SPEC, 6> {
        EOC6_W::new(self)
    }
    #[doc = "Bit 7 - End of Conversion Interrupt Disable 7"]
    #[inline(always)]
    #[must_use]
    pub fn eoc7(&mut self) -> EOC7_W<IDR_SPEC, 7> {
        EOC7_W::new(self)
    }
    #[doc = "Bit 8 - End of Conversion Interrupt Disable 8"]
    #[inline(always)]
    #[must_use]
    pub fn eoc8(&mut self) -> EOC8_W<IDR_SPEC, 8> {
        EOC8_W::new(self)
    }
    #[doc = "Bit 9 - End of Conversion Interrupt Disable 9"]
    #[inline(always)]
    #[must_use]
    pub fn eoc9(&mut self) -> EOC9_W<IDR_SPEC, 9> {
        EOC9_W::new(self)
    }
    #[doc = "Bit 10 - End of Conversion Interrupt Disable 10"]
    #[inline(always)]
    #[must_use]
    pub fn eoc10(&mut self) -> EOC10_W<IDR_SPEC, 10> {
        EOC10_W::new(self)
    }
    #[doc = "Bit 11 - End of Conversion Interrupt Disable 11"]
    #[inline(always)]
    #[must_use]
    pub fn eoc11(&mut self) -> EOC11_W<IDR_SPEC, 11> {
        EOC11_W::new(self)
    }
    #[doc = "Bit 24 - Data Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DRDY_W<IDR_SPEC, 24> {
        DRDY_W::new(self)
    }
    #[doc = "Bit 25 - General Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn govre(&mut self) -> GOVRE_W<IDR_SPEC, 25> {
        GOVRE_W::new(self)
    }
    #[doc = "Bit 26 - Comparison Event Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn compe(&mut self) -> COMPE_W<IDR_SPEC, 26> {
        COMPE_W::new(self)
    }
    #[doc = "Bit 30 - Temperature Change Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tempchg(&mut self) -> TEMPCHG_W<IDR_SPEC, 30> {
        TEMPCHG_W::new(self)
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
#[doc = "AFEC Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
