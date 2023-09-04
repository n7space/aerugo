#[doc = "Register `IDR` writer"]
pub type W = crate::W<IDR_SPEC>;
#[doc = "Field `MOSCXTS` writer - Main Crystal Oscillator Status Interrupt Disable"]
pub type MOSCXTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCKA` writer - PLLA Lock Interrupt Disable"]
pub type LOCKA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MCKRDY` writer - Master Clock Ready Interrupt Disable"]
pub type MCKRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCKU` writer - UTMI PLL Lock Interrupt Disable"]
pub type LOCKU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCKRDY0` writer - Programmable Clock Ready 0 Interrupt Disable"]
pub type PCKRDY0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCKRDY1` writer - Programmable Clock Ready 1 Interrupt Disable"]
pub type PCKRDY1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCKRDY2` writer - Programmable Clock Ready 2 Interrupt Disable"]
pub type PCKRDY2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCKRDY3` writer - Programmable Clock Ready 3 Interrupt Disable"]
pub type PCKRDY3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCKRDY4` writer - Programmable Clock Ready 4 Interrupt Disable"]
pub type PCKRDY4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCKRDY5` writer - Programmable Clock Ready 5 Interrupt Disable"]
pub type PCKRDY5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCKRDY6` writer - Programmable Clock Ready 6 Interrupt Disable"]
pub type PCKRDY6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCKRDY7` writer - Programmable Clock Ready 7 Interrupt Disable"]
pub type PCKRDY7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MOSCSELS` writer - Main Clock Source Oscillator Selection Status Interrupt Disable"]
pub type MOSCSELS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MOSCRCS` writer - Main RC Status Interrupt Disable"]
pub type MOSCRCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFDEV` writer - Clock Failure Detector Event Interrupt Disable"]
pub type CFDEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XT32KERR` writer - 32.768 kHz Crystal Oscillator Error Interrupt Disable"]
pub type XT32KERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn moscxts(&mut self) -> MOSCXTS_W<IDR_SPEC, 0> {
        MOSCXTS_W::new(self)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn locka(&mut self) -> LOCKA_W<IDR_SPEC, 1> {
        LOCKA_W::new(self)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mckrdy(&mut self) -> MCKRDY_W<IDR_SPEC, 3> {
        MCKRDY_W::new(self)
    }
    #[doc = "Bit 6 - UTMI PLL Lock Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn locku(&mut self) -> LOCKU_W<IDR_SPEC, 6> {
        LOCKU_W::new(self)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy0(&mut self) -> PCKRDY0_W<IDR_SPEC, 8> {
        PCKRDY0_W::new(self)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy1(&mut self) -> PCKRDY1_W<IDR_SPEC, 9> {
        PCKRDY1_W::new(self)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy2(&mut self) -> PCKRDY2_W<IDR_SPEC, 10> {
        PCKRDY2_W::new(self)
    }
    #[doc = "Bit 11 - Programmable Clock Ready 3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy3(&mut self) -> PCKRDY3_W<IDR_SPEC, 11> {
        PCKRDY3_W::new(self)
    }
    #[doc = "Bit 12 - Programmable Clock Ready 4 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy4(&mut self) -> PCKRDY4_W<IDR_SPEC, 12> {
        PCKRDY4_W::new(self)
    }
    #[doc = "Bit 13 - Programmable Clock Ready 5 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy5(&mut self) -> PCKRDY5_W<IDR_SPEC, 13> {
        PCKRDY5_W::new(self)
    }
    #[doc = "Bit 14 - Programmable Clock Ready 6 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy6(&mut self) -> PCKRDY6_W<IDR_SPEC, 14> {
        PCKRDY6_W::new(self)
    }
    #[doc = "Bit 15 - Programmable Clock Ready 7 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pckrdy7(&mut self) -> PCKRDY7_W<IDR_SPEC, 15> {
        PCKRDY7_W::new(self)
    }
    #[doc = "Bit 16 - Main Clock Source Oscillator Selection Status Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn moscsels(&mut self) -> MOSCSELS_W<IDR_SPEC, 16> {
        MOSCSELS_W::new(self)
    }
    #[doc = "Bit 17 - Main RC Status Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcs(&mut self) -> MOSCRCS_W<IDR_SPEC, 17> {
        MOSCRCS_W::new(self)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cfdev(&mut self) -> CFDEV_W<IDR_SPEC, 18> {
        CFDEV_W::new(self)
    }
    #[doc = "Bit 21 - 32.768 kHz Crystal Oscillator Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn xt32kerr(&mut self) -> XT32KERR_W<IDR_SPEC, 21> {
        XT32KERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
