#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Enable 0"]
pub type EOC0_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Enable 1"]
pub type EOC1_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `EOC2` writer - End of Conversion Interrupt Enable 2"]
pub type EOC2_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `EOC3` writer - End of Conversion Interrupt Enable 3"]
pub type EOC3_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `EOC4` writer - End of Conversion Interrupt Enable 4"]
pub type EOC4_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `EOC5` writer - End of Conversion Interrupt Enable 5"]
pub type EOC5_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `EOC6` writer - End of Conversion Interrupt Enable 6"]
pub type EOC6_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `EOC7` writer - End of Conversion Interrupt Enable 7"]
pub type EOC7_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `EOC8` writer - End of Conversion Interrupt Enable 8"]
pub type EOC8_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `EOC9` writer - End of Conversion Interrupt Enable 9"]
pub type EOC9_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `EOC10` writer - End of Conversion Interrupt Enable 10"]
pub type EOC10_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `EOC11` writer - End of Conversion Interrupt Enable 11"]
pub type EOC11_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `DRDY` writer - Data Ready Interrupt Enable"]
pub type DRDY_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `GOVRE` writer - General Overrun Error Interrupt Enable"]
pub type GOVRE_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `COMPE` writer - Comparison Event Interrupt Enable"]
pub type COMPE_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `TEMPCHG` writer - Temperature Change Interrupt Enable"]
pub type TEMPCHG_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
impl W {
    #[doc = "Bit 0 - End of Conversion Interrupt Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn eoc0(&mut self) -> EOC0_W<0> {
        EOC0_W::new(self)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn eoc1(&mut self) -> EOC1_W<1> {
        EOC1_W::new(self)
    }
    #[doc = "Bit 2 - End of Conversion Interrupt Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn eoc2(&mut self) -> EOC2_W<2> {
        EOC2_W::new(self)
    }
    #[doc = "Bit 3 - End of Conversion Interrupt Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn eoc3(&mut self) -> EOC3_W<3> {
        EOC3_W::new(self)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn eoc4(&mut self) -> EOC4_W<4> {
        EOC4_W::new(self)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn eoc5(&mut self) -> EOC5_W<5> {
        EOC5_W::new(self)
    }
    #[doc = "Bit 6 - End of Conversion Interrupt Enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn eoc6(&mut self) -> EOC6_W<6> {
        EOC6_W::new(self)
    }
    #[doc = "Bit 7 - End of Conversion Interrupt Enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn eoc7(&mut self) -> EOC7_W<7> {
        EOC7_W::new(self)
    }
    #[doc = "Bit 8 - End of Conversion Interrupt Enable 8"]
    #[inline(always)]
    #[must_use]
    pub fn eoc8(&mut self) -> EOC8_W<8> {
        EOC8_W::new(self)
    }
    #[doc = "Bit 9 - End of Conversion Interrupt Enable 9"]
    #[inline(always)]
    #[must_use]
    pub fn eoc9(&mut self) -> EOC9_W<9> {
        EOC9_W::new(self)
    }
    #[doc = "Bit 10 - End of Conversion Interrupt Enable 10"]
    #[inline(always)]
    #[must_use]
    pub fn eoc10(&mut self) -> EOC10_W<10> {
        EOC10_W::new(self)
    }
    #[doc = "Bit 11 - End of Conversion Interrupt Enable 11"]
    #[inline(always)]
    #[must_use]
    pub fn eoc11(&mut self) -> EOC11_W<11> {
        EOC11_W::new(self)
    }
    #[doc = "Bit 24 - Data Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DRDY_W<24> {
        DRDY_W::new(self)
    }
    #[doc = "Bit 25 - General Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn govre(&mut self) -> GOVRE_W<25> {
        GOVRE_W::new(self)
    }
    #[doc = "Bit 26 - Comparison Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compe(&mut self) -> COMPE_W<26> {
        COMPE_W::new(self)
    }
    #[doc = "Bit 30 - Temperature Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tempchg(&mut self) -> TEMPCHG_W<30> {
        TEMPCHG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
