#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Voltage Regulator Off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VROFFSELECT_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: If KEY is correct, VROFF asserts the vddcore_nreset and stops the voltage regulator."]
    STOP_VREG = 1,
}
impl From<VROFFSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: VROFFSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VROFF` writer - Voltage Regulator Off"]
pub type VROFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VROFFSELECT_AW>;
impl<'a, REG, const O: u8> VROFF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(VROFFSELECT_AW::NO_EFFECT)
    }
    #[doc = "If KEY is correct, VROFF asserts the vddcore_nreset and stops the voltage regulator."]
    #[inline(always)]
    pub fn stop_vreg(self) -> &'a mut crate::W<REG> {
        self.variant(VROFFSELECT_AW::STOP_VREG)
    }
}
#[doc = "Crystal Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XTALSELSELECT_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: If KEY is correct, XTALSEL switches the slow clock on the crystal oscillator output."]
    CRYSTAL_SEL = 1,
}
impl From<XTALSELSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: XTALSELSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALSEL` writer - Crystal Oscillator Select"]
pub type XTALSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, XTALSELSELECT_AW>;
impl<'a, REG, const O: u8> XTALSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(XTALSELSELECT_AW::NO_EFFECT)
    }
    #[doc = "If KEY is correct, XTALSEL switches the slow clock on the crystal oscillator output."]
    #[inline(always)]
    pub fn crystal_sel(self) -> &'a mut crate::W<REG> {
        self.variant(XTALSELSELECT_AW::CRYSTAL_SEL)
    }
}
#[doc = "Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEYSELECT_AW {
    #[doc = "165: Writing any other value in this field aborts the write operation."]
    PASSWD = 165,
}
impl From<KEYSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEYSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEYSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `KEY` writer - Password"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, KEYSELECT_AW>;
impl<'a, REG, const O: u8> KEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSELECT_AW::PASSWD)
    }
}
impl W {
    #[doc = "Bit 2 - Voltage Regulator Off"]
    #[inline(always)]
    #[must_use]
    pub fn vroff(&mut self) -> VROFF_W<CR_SPEC, 2> {
        VROFF_W::new(self)
    }
    #[doc = "Bit 3 - Crystal Oscillator Select"]
    #[inline(always)]
    #[must_use]
    pub fn xtalsel(&mut self) -> XTALSEL_W<CR_SPEC, 3> {
        XTALSEL_W::new(self)
    }
    #[doc = "Bits 24:31 - Password"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CR_SPEC, 24> {
        KEY_W::new(self)
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
#[doc = "Supply Controller Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
