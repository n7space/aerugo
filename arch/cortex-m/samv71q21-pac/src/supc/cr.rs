#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
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
pub type VROFF_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, VROFFSELECT_AW>;
impl<'a, const O: u8> VROFF_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(VROFFSELECT_AW::NO_EFFECT)
    }
    #[doc = "If KEY is correct, VROFF asserts the vddcore_nreset and stops the voltage regulator."]
    #[inline(always)]
    pub fn stop_vreg(self) -> &'a mut W {
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
pub type XTALSEL_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, XTALSELSELECT_AW>;
impl<'a, const O: u8> XTALSEL_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(XTALSELSELECT_AW::NO_EFFECT)
    }
    #[doc = "If KEY is correct, XTALSEL switches the slow clock on the crystal oscillator output."]
    #[inline(always)]
    pub fn crystal_sel(self) -> &'a mut W {
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
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, CR_SPEC, 8, O, KEYSELECT_AW>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEYSELECT_AW::PASSWD)
    }
}
impl W {
    #[doc = "Bit 2 - Voltage Regulator Off"]
    #[inline(always)]
    #[must_use]
    pub fn vroff(&mut self) -> VROFF_W<2> {
        VROFF_W::new(self)
    }
    #[doc = "Bit 3 - Crystal Oscillator Select"]
    #[inline(always)]
    #[must_use]
    pub fn xtalsel(&mut self) -> XTALSEL_W<3> {
        XTALSEL_W::new(self)
    }
    #[doc = "Bits 24:31 - Password"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<24> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
