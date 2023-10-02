#[doc = "Register `EMR` reader"]
pub type R = crate::R<EMR_SPEC>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EMR_SPEC>;
#[doc = "Field `TRIGSRCA` reader - Trigger Source for Input A"]
pub type TRIGSRCA_R = crate::FieldReader<TRIGSRCASELECT_A>;
#[doc = "Trigger Source for Input A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSRCASELECT_A {
    #[doc = "0: The trigger/capture input A is driven by external pin TIOAx"]
    EXTERNAL_TIOAX = 0,
    #[doc = "1: The trigger/capture input A is driven internally by PWMx"]
    PWMX = 1,
}
impl From<TRIGSRCASELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRCASELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGSRCASELECT_A {
    type Ux = u8;
}
impl TRIGSRCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGSRCASELECT_A> {
        match self.bits {
            0 => Some(TRIGSRCASELECT_A::EXTERNAL_TIOAX),
            1 => Some(TRIGSRCASELECT_A::PWMX),
            _ => None,
        }
    }
    #[doc = "The trigger/capture input A is driven by external pin TIOAx"]
    #[inline(always)]
    pub fn is_external_tioax(&self) -> bool {
        *self == TRIGSRCASELECT_A::EXTERNAL_TIOAX
    }
    #[doc = "The trigger/capture input A is driven internally by PWMx"]
    #[inline(always)]
    pub fn is_pwmx(&self) -> bool {
        *self == TRIGSRCASELECT_A::PWMX
    }
}
#[doc = "Field `TRIGSRCA` writer - Trigger Source for Input A"]
pub type TRIGSRCA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, TRIGSRCASELECT_A>;
impl<'a, REG, const O: u8> TRIGSRCA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The trigger/capture input A is driven by external pin TIOAx"]
    #[inline(always)]
    pub fn external_tioax(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCASELECT_A::EXTERNAL_TIOAX)
    }
    #[doc = "The trigger/capture input A is driven internally by PWMx"]
    #[inline(always)]
    pub fn pwmx(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCASELECT_A::PWMX)
    }
}
#[doc = "Field `TRIGSRCB` reader - Trigger Source for Input B"]
pub type TRIGSRCB_R = crate::FieldReader<TRIGSRCBSELECT_A>;
#[doc = "Trigger Source for Input B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSRCBSELECT_A {
    #[doc = "0: The trigger/capture input B is driven by external pin TIOBx"]
    EXTERNAL_TIOBX = 0,
    #[doc = "1: For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    PWMX = 1,
}
impl From<TRIGSRCBSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRCBSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGSRCBSELECT_A {
    type Ux = u8;
}
impl TRIGSRCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGSRCBSELECT_A> {
        match self.bits {
            0 => Some(TRIGSRCBSELECT_A::EXTERNAL_TIOBX),
            1 => Some(TRIGSRCBSELECT_A::PWMX),
            _ => None,
        }
    }
    #[doc = "The trigger/capture input B is driven by external pin TIOBx"]
    #[inline(always)]
    pub fn is_external_tiobx(&self) -> bool {
        *self == TRIGSRCBSELECT_A::EXTERNAL_TIOBX
    }
    #[doc = "For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    #[inline(always)]
    pub fn is_pwmx(&self) -> bool {
        *self == TRIGSRCBSELECT_A::PWMX
    }
}
#[doc = "Field `TRIGSRCB` writer - Trigger Source for Input B"]
pub type TRIGSRCB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, TRIGSRCBSELECT_A>;
impl<'a, REG, const O: u8> TRIGSRCB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The trigger/capture input B is driven by external pin TIOBx"]
    #[inline(always)]
    pub fn external_tiobx(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCBSELECT_A::EXTERNAL_TIOBX)
    }
    #[doc = "For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    #[inline(always)]
    pub fn pwmx(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSRCBSELECT_A::PWMX)
    }
}
#[doc = "Field `NODIVCLK` reader - No Divided Clock"]
pub type NODIVCLK_R = crate::BitReader;
#[doc = "Field `NODIVCLK` writer - No Divided Clock"]
pub type NODIVCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline(always)]
    pub fn trigsrca(&self) -> TRIGSRCA_R {
        TRIGSRCA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline(always)]
    pub fn trigsrcb(&self) -> TRIGSRCB_R {
        TRIGSRCB_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline(always)]
    pub fn nodivclk(&self) -> NODIVCLK_R {
        NODIVCLK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline(always)]
    #[must_use]
    pub fn trigsrca(&mut self) -> TRIGSRCA_W<EMR_SPEC, 0> {
        TRIGSRCA_W::new(self)
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline(always)]
    #[must_use]
    pub fn trigsrcb(&mut self) -> TRIGSRCB_W<EMR_SPEC, 4> {
        TRIGSRCB_W::new(self)
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline(always)]
    #[must_use]
    pub fn nodivclk(&mut self) -> NODIVCLK_W<EMR_SPEC, 8> {
        NODIVCLK_W::new(self)
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
#[doc = "Extended Mode Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
