#[doc = "Register `PCR` reader"]
pub type R = crate::R<PCR_SPEC>;
#[doc = "Register `PCR` writer"]
pub type W = crate::W<PCR_SPEC>;
#[doc = "Field `PID` reader - Peripheral ID"]
pub type PID_R = crate::FieldReader;
#[doc = "Field `PID` writer - Peripheral ID"]
pub type PID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `GCLKCSS` reader - Generic Clock Source Selection"]
pub type GCLKCSS_R = crate::FieldReader<GCLKCSSSELECT_A>;
#[doc = "Generic Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GCLKCSSSELECT_A {
    #[doc = "0: Slow clock is selected"]
    SLOW_CLK = 0,
    #[doc = "1: Main clock is selected"]
    MAIN_CLK = 1,
    #[doc = "2: PLLACK is selected"]
    PLLA_CLK = 2,
    #[doc = "3: UPLL Clock is selected"]
    UPLL_CLK = 3,
    #[doc = "4: Master Clock is selected"]
    MCK_CLK = 4,
}
impl From<GCLKCSSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GCLKCSSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GCLKCSSSELECT_A {
    type Ux = u8;
}
impl GCLKCSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GCLKCSSSELECT_A> {
        match self.bits {
            0 => Some(GCLKCSSSELECT_A::SLOW_CLK),
            1 => Some(GCLKCSSSELECT_A::MAIN_CLK),
            2 => Some(GCLKCSSSELECT_A::PLLA_CLK),
            3 => Some(GCLKCSSSELECT_A::UPLL_CLK),
            4 => Some(GCLKCSSSELECT_A::MCK_CLK),
            _ => None,
        }
    }
    #[doc = "Slow clock is selected"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == GCLKCSSSELECT_A::SLOW_CLK
    }
    #[doc = "Main clock is selected"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == GCLKCSSSELECT_A::MAIN_CLK
    }
    #[doc = "PLLACK is selected"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == GCLKCSSSELECT_A::PLLA_CLK
    }
    #[doc = "UPLL Clock is selected"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        *self == GCLKCSSSELECT_A::UPLL_CLK
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn is_mck_clk(&self) -> bool {
        *self == GCLKCSSSELECT_A::MCK_CLK
    }
}
#[doc = "Field `GCLKCSS` writer - Generic Clock Source Selection"]
pub type GCLKCSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, GCLKCSSSELECT_A>;
impl<'a, REG, const O: u8> GCLKCSS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slow clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut crate::W<REG> {
        self.variant(GCLKCSSSELECT_A::SLOW_CLK)
    }
    #[doc = "Main clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(GCLKCSSSELECT_A::MAIN_CLK)
    }
    #[doc = "PLLACK is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut crate::W<REG> {
        self.variant(GCLKCSSSELECT_A::PLLA_CLK)
    }
    #[doc = "UPLL Clock is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(GCLKCSSSELECT_A::UPLL_CLK)
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn mck_clk(self) -> &'a mut crate::W<REG> {
        self.variant(GCLKCSSSELECT_A::MCK_CLK)
    }
}
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::BitReader;
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GCLKDIV` reader - Generic Clock Division Ratio"]
pub type GCLKDIV_R = crate::FieldReader;
#[doc = "Field `GCLKDIV` writer - Generic Clock Division Ratio"]
pub type GCLKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GCLKEN` reader - Generic Clock Enable"]
pub type GCLKEN_R = crate::BitReader;
#[doc = "Field `GCLKEN` writer - Generic Clock Enable"]
pub type GCLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Generic Clock Source Selection"]
    #[inline(always)]
    pub fn gclkcss(&self) -> GCLKCSS_R {
        GCLKCSS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 20:27 - Generic Clock Division Ratio"]
    #[inline(always)]
    pub fn gclkdiv(&self) -> GCLKDIV_R {
        GCLKDIV_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Generic Clock Enable"]
    #[inline(always)]
    pub fn gclken(&self) -> GCLKEN_R {
        GCLKEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Peripheral ID"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<PCR_SPEC, 0> {
        PID_W::new(self)
    }
    #[doc = "Bits 8:10 - Generic Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn gclkcss(&mut self) -> GCLKCSS_W<PCR_SPEC, 8> {
        GCLKCSS_W::new(self)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<PCR_SPEC, 12> {
        CMD_W::new(self)
    }
    #[doc = "Bits 20:27 - Generic Clock Division Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn gclkdiv(&mut self) -> GCLKDIV_W<PCR_SPEC, 20> {
        GCLKDIV_W::new(self)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<PCR_SPEC, 28> {
        EN_W::new(self)
    }
    #[doc = "Bit 29 - Generic Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gclken(&mut self) -> GCLKEN_W<PCR_SPEC, 29> {
        GCLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCR to value 0"]
impl crate::Resettable for PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
