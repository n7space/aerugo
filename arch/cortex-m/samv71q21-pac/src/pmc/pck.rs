#[doc = "Register `PCK[%s]` reader"]
pub type R = crate::R<PCK_SPEC>;
#[doc = "Register `PCK[%s]` writer"]
pub type W = crate::W<PCK_SPEC>;
#[doc = "Field `CSS` reader - Programmable Clock Source Selection"]
pub type CSS_R = crate::FieldReader<CSSSELECT_A>;
#[doc = "Programmable Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSSSELECT_A {
    #[doc = "0: SLCK is selected"]
    SLOW_CLK = 0,
    #[doc = "1: MAINCK is selected"]
    MAIN_CLK = 1,
    #[doc = "2: PLLACK is selected"]
    PLLA_CLK = 2,
    #[doc = "3: UPLLCKDIV is selected"]
    UPLL_CLK = 3,
    #[doc = "4: MCK is selected"]
    MCK = 4,
}
impl From<CSSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSSSELECT_A {
    type Ux = u8;
}
impl CSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSSSELECT_A> {
        match self.bits {
            0 => Some(CSSSELECT_A::SLOW_CLK),
            1 => Some(CSSSELECT_A::MAIN_CLK),
            2 => Some(CSSSELECT_A::PLLA_CLK),
            3 => Some(CSSSELECT_A::UPLL_CLK),
            4 => Some(CSSSELECT_A::MCK),
            _ => None,
        }
    }
    #[doc = "SLCK is selected"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == CSSSELECT_A::SLOW_CLK
    }
    #[doc = "MAINCK is selected"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == CSSSELECT_A::MAIN_CLK
    }
    #[doc = "PLLACK is selected"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == CSSSELECT_A::PLLA_CLK
    }
    #[doc = "UPLLCKDIV is selected"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        *self == CSSSELECT_A::UPLL_CLK
    }
    #[doc = "MCK is selected"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CSSSELECT_A::MCK
    }
}
#[doc = "Field `CSS` writer - Programmable Clock Source Selection"]
pub type CSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CSSSELECT_A>;
impl<'a, REG, const O: u8> CSS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SLCK is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut crate::W<REG> {
        self.variant(CSSSELECT_A::SLOW_CLK)
    }
    #[doc = "MAINCK is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(CSSSELECT_A::MAIN_CLK)
    }
    #[doc = "PLLACK is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut crate::W<REG> {
        self.variant(CSSSELECT_A::PLLA_CLK)
    }
    #[doc = "UPLLCKDIV is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(CSSSELECT_A::UPLL_CLK)
    }
    #[doc = "MCK is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(CSSSELECT_A::MCK)
    }
}
#[doc = "Field `PRES` reader - Programmable Clock Prescaler"]
pub type PRES_R = crate::FieldReader;
#[doc = "Field `PRES` writer - Programmable Clock Prescaler"]
pub type PRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:2 - Programmable Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:11 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Programmable Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CSS_W<PCK_SPEC, 0> {
        CSS_W::new(self)
    }
    #[doc = "Bits 4:11 - Programmable Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PRES_W<PCK_SPEC, 4> {
        PRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Programmable Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pck::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pck::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCK_SPEC;
impl crate::RegisterSpec for PCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pck::R`](R) reader structure"]
impl crate::Readable for PCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pck::W`](W) writer structure"]
impl crate::Writable for PCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCK[%s]
to value 0"]
impl crate::Resettable for PCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
