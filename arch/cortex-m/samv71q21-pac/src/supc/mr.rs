#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `BODRSTEN` reader - Brownout Detector Reset Enable"]
pub type BODRSTEN_R = crate::BitReader<BODRSTENSELECT_A>;
#[doc = "Brownout Detector Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BODRSTENSELECT_A {
    #[doc = "0: The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    NOT_ENABLE = 0,
    #[doc = "1: The core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    ENABLE = 1,
}
impl From<BODRSTENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BODRSTENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BODRSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTENSELECT_A {
        match self.bits {
            false => BODRSTENSELECT_A::NOT_ENABLE,
            true => BODRSTENSELECT_A::ENABLE,
        }
    }
    #[doc = "The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == BODRSTENSELECT_A::NOT_ENABLE
    }
    #[doc = "The core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODRSTENSELECT_A::ENABLE
    }
}
#[doc = "Field `BODRSTEN` writer - Brownout Detector Reset Enable"]
pub type BODRSTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BODRSTENSELECT_A>;
impl<'a, REG, const O: u8> BODRSTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(BODRSTENSELECT_A::NOT_ENABLE)
    }
    #[doc = "The core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BODRSTENSELECT_A::ENABLE)
    }
}
#[doc = "Field `BODDIS` reader - Brownout Detector Disable"]
pub type BODDIS_R = crate::BitReader<BODDISSELECT_A>;
#[doc = "Brownout Detector Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BODDISSELECT_A {
    #[doc = "0: The core brownout detector is enabled."]
    ENABLE = 0,
    #[doc = "1: The core brownout detector is disabled."]
    DISABLE = 1,
}
impl From<BODDISSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BODDISSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BODDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODDISSELECT_A {
        match self.bits {
            false => BODDISSELECT_A::ENABLE,
            true => BODDISSELECT_A::DISABLE,
        }
    }
    #[doc = "The core brownout detector is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODDISSELECT_A::ENABLE
    }
    #[doc = "The core brownout detector is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODDISSELECT_A::DISABLE
    }
}
#[doc = "Field `BODDIS` writer - Brownout Detector Disable"]
pub type BODDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BODDISSELECT_A>;
impl<'a, REG, const O: u8> BODDIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The core brownout detector is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BODDISSELECT_A::ENABLE)
    }
    #[doc = "The core brownout detector is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BODDISSELECT_A::DISABLE)
    }
}
#[doc = "Field `ONREG` reader - Voltage Regulator Enable"]
pub type ONREG_R = crate::BitReader<ONREGSELECT_A>;
#[doc = "Voltage Regulator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONREGSELECT_A {
    #[doc = "0: Internal voltage regulator is not used (external power supply is used)."]
    ONREG_UNUSED = 0,
    #[doc = "1: Internal voltage regulator is used."]
    ONREG_USED = 1,
}
impl From<ONREGSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ONREGSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ONREG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONREGSELECT_A {
        match self.bits {
            false => ONREGSELECT_A::ONREG_UNUSED,
            true => ONREGSELECT_A::ONREG_USED,
        }
    }
    #[doc = "Internal voltage regulator is not used (external power supply is used)."]
    #[inline(always)]
    pub fn is_onreg_unused(&self) -> bool {
        *self == ONREGSELECT_A::ONREG_UNUSED
    }
    #[doc = "Internal voltage regulator is used."]
    #[inline(always)]
    pub fn is_onreg_used(&self) -> bool {
        *self == ONREGSELECT_A::ONREG_USED
    }
}
#[doc = "Field `ONREG` writer - Voltage Regulator Enable"]
pub type ONREG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ONREGSELECT_A>;
impl<'a, REG, const O: u8> ONREG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal voltage regulator is not used (external power supply is used)."]
    #[inline(always)]
    pub fn onreg_unused(self) -> &'a mut crate::W<REG> {
        self.variant(ONREGSELECT_A::ONREG_UNUSED)
    }
    #[doc = "Internal voltage regulator is used."]
    #[inline(always)]
    pub fn onreg_used(self) -> &'a mut crate::W<REG> {
        self.variant(ONREGSELECT_A::ONREG_USED)
    }
}
#[doc = "Field `BKUPRETON` reader - SRAM On In Backup Mode"]
pub type BKUPRETON_R = crate::BitReader;
#[doc = "Field `BKUPRETON` writer - SRAM On In Backup Mode"]
pub type BKUPRETON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSCBYPASS` reader - Oscillator Bypass"]
pub type OSCBYPASS_R = crate::BitReader<OSCBYPASSSELECT_A>;
#[doc = "Oscillator Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCBYPASSSELECT_A {
    #[doc = "0: No effect. Clock selection depends on the value of XTALSEL (SUPC_CR)."]
    NO_EFFECT = 0,
    #[doc = "1: The 32 kHz crystal oscillator is bypassed if XTALSEL (SUPC_CR) is set. OSCBYPASS must be set prior to setting XTALSEL."]
    BYPASS = 1,
}
impl From<OSCBYPASSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: OSCBYPASSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCBYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCBYPASSSELECT_A {
        match self.bits {
            false => OSCBYPASSSELECT_A::NO_EFFECT,
            true => OSCBYPASSSELECT_A::BYPASS,
        }
    }
    #[doc = "No effect. Clock selection depends on the value of XTALSEL (SUPC_CR)."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OSCBYPASSSELECT_A::NO_EFFECT
    }
    #[doc = "The 32 kHz crystal oscillator is bypassed if XTALSEL (SUPC_CR) is set. OSCBYPASS must be set prior to setting XTALSEL."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == OSCBYPASSSELECT_A::BYPASS
    }
}
#[doc = "Field `OSCBYPASS` writer - Oscillator Bypass"]
pub type OSCBYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSCBYPASSSELECT_A>;
impl<'a, REG, const O: u8> OSCBYPASS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. Clock selection depends on the value of XTALSEL (SUPC_CR)."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(OSCBYPASSSELECT_A::NO_EFFECT)
    }
    #[doc = "The 32 kHz crystal oscillator is bypassed if XTALSEL (SUPC_CR) is set. OSCBYPASS must be set prior to setting XTALSEL."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(OSCBYPASSSELECT_A::BYPASS)
    }
}
#[doc = "Field `KEY` reader - Password Key"]
pub type KEY_R = crate::FieldReader<KEYSELECT_A>;
#[doc = "Password Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEYSELECT_A {
    #[doc = "165: Writing any other value in this field aborts the write operation."]
    PASSWD = 165,
}
impl From<KEYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEYSELECT_A {
    type Ux = u8;
}
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEYSELECT_A> {
        match self.bits {
            165 => Some(KEYSELECT_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == KEYSELECT_A::PASSWD
    }
}
#[doc = "Field `KEY` writer - Password Key"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, KEYSELECT_A>;
impl<'a, REG, const O: u8> KEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSELECT_A::PASSWD)
    }
}
impl R {
    #[doc = "Bit 12 - Brownout Detector Reset Enable"]
    #[inline(always)]
    pub fn bodrsten(&self) -> BODRSTEN_R {
        BODRSTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Brownout Detector Disable"]
    #[inline(always)]
    pub fn boddis(&self) -> BODDIS_R {
        BODDIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Voltage Regulator Enable"]
    #[inline(always)]
    pub fn onreg(&self) -> ONREG_R {
        ONREG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM On In Backup Mode"]
    #[inline(always)]
    pub fn bkupreton(&self) -> BKUPRETON_R {
        BKUPRETON_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbypass(&self) -> OSCBYPASS_R {
        OSCBYPASS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Brownout Detector Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bodrsten(&mut self) -> BODRSTEN_W<MR_SPEC, 12> {
        BODRSTEN_W::new(self)
    }
    #[doc = "Bit 13 - Brownout Detector Disable"]
    #[inline(always)]
    #[must_use]
    pub fn boddis(&mut self) -> BODDIS_W<MR_SPEC, 13> {
        BODDIS_W::new(self)
    }
    #[doc = "Bit 14 - Voltage Regulator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn onreg(&mut self) -> ONREG_W<MR_SPEC, 14> {
        ONREG_W::new(self)
    }
    #[doc = "Bit 17 - SRAM On In Backup Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bkupreton(&mut self) -> BKUPRETON_W<MR_SPEC, 17> {
        BKUPRETON_W::new(self)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn oscbypass(&mut self) -> OSCBYPASS_W<MR_SPEC, 20> {
        OSCBYPASS_W::new(self)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<MR_SPEC, 24> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Supply Controller Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
