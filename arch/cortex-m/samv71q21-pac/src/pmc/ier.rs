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
#[doc = "Field `MOSCXTS` writer - Main Crystal Oscillator Status Interrupt Enable"]
pub type MOSCXTS_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `LOCKA` writer - PLLA Lock Interrupt Enable"]
pub type LOCKA_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `MCKRDY` writer - Master Clock Ready Interrupt Enable"]
pub type MCKRDY_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `LOCKU` writer - UTMI PLL Lock Interrupt Enable"]
pub type LOCKU_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `PCKRDY0` writer - Programmable Clock Ready 0 Interrupt Enable"]
pub type PCKRDY0_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `PCKRDY1` writer - Programmable Clock Ready 1 Interrupt Enable"]
pub type PCKRDY1_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `PCKRDY2` writer - Programmable Clock Ready 2 Interrupt Enable"]
pub type PCKRDY2_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `PCKRDY3` writer - Programmable Clock Ready 3 Interrupt Enable"]
pub type PCKRDY3_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `PCKRDY4` writer - Programmable Clock Ready 4 Interrupt Enable"]
pub type PCKRDY4_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `PCKRDY5` writer - Programmable Clock Ready 5 Interrupt Enable"]
pub type PCKRDY5_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `PCKRDY6` writer - Programmable Clock Ready 6 Interrupt Enable"]
pub type PCKRDY6_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `PCKRDY7` writer - Programmable Clock Ready 7 Interrupt Enable"]
pub type PCKRDY7_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `MOSCSELS` writer - Main Clock Source Oscillator Selection Status Interrupt Enable"]
pub type MOSCSELS_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `MOSCRCS` writer - Main RC Oscillator Status Interrupt Enable"]
pub type MOSCRCS_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `CFDEV` writer - Clock Failure Detector Event Interrupt Enable"]
pub type CFDEV_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
#[doc = "Field `XT32KERR` writer - 32.768 kHz Crystal Oscillator Error Interrupt Enable"]
pub type XT32KERR_W<'a, const O: u8> = crate::BitWriter<'a, IER_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscxts(&mut self) -> MOSCXTS_W<0> {
        MOSCXTS_W::new(self)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn locka(&mut self) -> LOCKA_W<1> {
        LOCKA_W::new(self)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mckrdy(&mut self) -> MCKRDY_W<3> {
        MCKRDY_W::new(self)
    }
    #[doc = "Bit 6 - UTMI PLL Lock Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn locku(&mut self) -> LOCKU_W<6> {
        LOCKU_W::new(self)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy0(&mut self) -> PCKRDY0_W<8> {
        PCKRDY0_W::new(self)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy1(&mut self) -> PCKRDY1_W<9> {
        PCKRDY1_W::new(self)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy2(&mut self) -> PCKRDY2_W<10> {
        PCKRDY2_W::new(self)
    }
    #[doc = "Bit 11 - Programmable Clock Ready 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy3(&mut self) -> PCKRDY3_W<11> {
        PCKRDY3_W::new(self)
    }
    #[doc = "Bit 12 - Programmable Clock Ready 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy4(&mut self) -> PCKRDY4_W<12> {
        PCKRDY4_W::new(self)
    }
    #[doc = "Bit 13 - Programmable Clock Ready 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy5(&mut self) -> PCKRDY5_W<13> {
        PCKRDY5_W::new(self)
    }
    #[doc = "Bit 14 - Programmable Clock Ready 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy6(&mut self) -> PCKRDY6_W<14> {
        PCKRDY6_W::new(self)
    }
    #[doc = "Bit 15 - Programmable Clock Ready 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy7(&mut self) -> PCKRDY7_W<15> {
        PCKRDY7_W::new(self)
    }
    #[doc = "Bit 16 - Main Clock Source Oscillator Selection Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscsels(&mut self) -> MOSCSELS_W<16> {
        MOSCSELS_W::new(self)
    }
    #[doc = "Bit 17 - Main RC Oscillator Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcs(&mut self) -> MOSCRCS_W<17> {
        MOSCRCS_W::new(self)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfdev(&mut self) -> CFDEV_W<18> {
        CFDEV_W::new(self)
    }
    #[doc = "Bit 21 - 32.768 kHz Crystal Oscillator Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xt32kerr(&mut self) -> XT32KERR_W<21> {
        XT32KERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
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
