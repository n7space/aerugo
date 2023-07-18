#[doc = "Register `CCCR` reader"]
pub struct R(crate::R<CCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCCR` writer"]
pub struct W(crate::W<CCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCCR_SPEC>;
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
impl From<crate::W<CCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` reader - Initialization (read/write)"]
pub type INIT_R = crate::BitReader<INITSELECT_A>;
#[doc = "Initialization (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITSELECT_A {
    #[doc = "0: Normal operation."]
    DISABLED = 0,
    #[doc = "1: Initialization is started."]
    ENABLED = 1,
}
impl From<INITSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: INITSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITSELECT_A {
        match self.bits {
            false => INITSELECT_A::DISABLED,
            true => INITSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INITSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INITSELECT_A::ENABLED
    }
}
#[doc = "Field `INIT` writer - Initialization (read/write)"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O, INITSELECT_A>;
impl<'a, const O: u8> INIT_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INITSELECT_A::DISABLED)
    }
    #[doc = "Initialization is started."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INITSELECT_A::ENABLED)
    }
}
#[doc = "Field `CCE` reader - Configuration Change Enable (read/write, write protection)"]
pub type CCE_R = crate::BitReader<CCESELECT_A>;
#[doc = "Configuration Change Enable (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCESELECT_A {
    #[doc = "0: The processor has no write access to the protected configuration registers."]
    PROTECTED = 0,
    #[doc = "1: The processor has write access to the protected configuration registers (while MCAN_CCCR.INIT = '1')."]
    CONFIGURABLE = 1,
}
impl From<CCESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CCESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCESELECT_A {
        match self.bits {
            false => CCESELECT_A::PROTECTED,
            true => CCESELECT_A::CONFIGURABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == CCESELECT_A::PROTECTED
    }
    #[doc = "Checks if the value of the field is `CONFIGURABLE`"]
    #[inline(always)]
    pub fn is_configurable(&self) -> bool {
        *self == CCESELECT_A::CONFIGURABLE
    }
}
#[doc = "Field `CCE` writer - Configuration Change Enable (read/write, write protection)"]
pub type CCE_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O, CCESELECT_A>;
impl<'a, const O: u8> CCE_W<'a, O> {
    #[doc = "The processor has no write access to the protected configuration registers."]
    #[inline(always)]
    pub fn protected(self) -> &'a mut W {
        self.variant(CCESELECT_A::PROTECTED)
    }
    #[doc = "The processor has write access to the protected configuration registers (while MCAN_CCCR.INIT = '1')."]
    #[inline(always)]
    pub fn configurable(self) -> &'a mut W {
        self.variant(CCESELECT_A::CONFIGURABLE)
    }
}
#[doc = "Field `ASM` reader - Restricted Operation Mode (read/write, write protection against '1')"]
pub type ASM_R = crate::BitReader<ASMSELECT_A>;
#[doc = "Restricted Operation Mode (read/write, write protection against '1')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASMSELECT_A {
    #[doc = "0: Normal CAN operation."]
    NORMAL = 0,
    #[doc = "1: Restricted Operation mode active."]
    RESTRICTED = 1,
}
impl From<ASMSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ASMSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ASM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASMSELECT_A {
        match self.bits {
            false => ASMSELECT_A::NORMAL,
            true => ASMSELECT_A::RESTRICTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ASMSELECT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESTRICTED`"]
    #[inline(always)]
    pub fn is_restricted(&self) -> bool {
        *self == ASMSELECT_A::RESTRICTED
    }
}
#[doc = "Field `ASM` writer - Restricted Operation Mode (read/write, write protection against '1')"]
pub type ASM_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O, ASMSELECT_A>;
impl<'a, const O: u8> ASM_W<'a, O> {
    #[doc = "Normal CAN operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ASMSELECT_A::NORMAL)
    }
    #[doc = "Restricted Operation mode active."]
    #[inline(always)]
    pub fn restricted(self) -> &'a mut W {
        self.variant(ASMSELECT_A::RESTRICTED)
    }
}
#[doc = "Field `CSA` reader - Clock Stop Acknowledge (read-only)"]
pub type CSA_R = crate::BitReader;
#[doc = "Field `CSA` writer - Clock Stop Acknowledge (read-only)"]
pub type CSA_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O>;
#[doc = "Field `CSR` reader - Clock Stop Request (read/write)"]
pub type CSR_R = crate::BitReader<CSRSELECT_A>;
#[doc = "Clock Stop Request (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSRSELECT_A {
    #[doc = "0: No clock stop is requested."]
    NO_CLOCK_STOP = 0,
    #[doc = "1: Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pend-ing transfer requests have been completed and the CAN bus reached idle."]
    CLOCK_STOP = 1,
}
impl From<CSRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CSRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSRSELECT_A {
        match self.bits {
            false => CSRSELECT_A::NO_CLOCK_STOP,
            true => CSRSELECT_A::CLOCK_STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK_STOP`"]
    #[inline(always)]
    pub fn is_no_clock_stop(&self) -> bool {
        *self == CSRSELECT_A::NO_CLOCK_STOP
    }
    #[doc = "Checks if the value of the field is `CLOCK_STOP`"]
    #[inline(always)]
    pub fn is_clock_stop(&self) -> bool {
        *self == CSRSELECT_A::CLOCK_STOP
    }
}
#[doc = "Field `CSR` writer - Clock Stop Request (read/write)"]
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O, CSRSELECT_A>;
impl<'a, const O: u8> CSR_W<'a, O> {
    #[doc = "No clock stop is requested."]
    #[inline(always)]
    pub fn no_clock_stop(self) -> &'a mut W {
        self.variant(CSRSELECT_A::NO_CLOCK_STOP)
    }
    #[doc = "Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pend-ing transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn clock_stop(self) -> &'a mut W {
        self.variant(CSRSELECT_A::CLOCK_STOP)
    }
}
#[doc = "Field `MON` reader - Bus Monitoring Mode (read/write, write protection against '1')"]
pub type MON_R = crate::BitReader<MONSELECT_A>;
#[doc = "Bus Monitoring Mode (read/write, write protection against '1')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONSELECT_A {
    #[doc = "0: Bus Monitoring mode is disabled."]
    DISABLED = 0,
    #[doc = "1: Bus Monitoring mode is enabled."]
    ENABLED = 1,
}
impl From<MONSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MONSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONSELECT_A {
        match self.bits {
            false => MONSELECT_A::DISABLED,
            true => MONSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONSELECT_A::ENABLED
    }
}
#[doc = "Field `MON` writer - Bus Monitoring Mode (read/write, write protection against '1')"]
pub type MON_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O, MONSELECT_A>;
impl<'a, const O: u8> MON_W<'a, O> {
    #[doc = "Bus Monitoring mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONSELECT_A::DISABLED)
    }
    #[doc = "Bus Monitoring mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONSELECT_A::ENABLED)
    }
}
#[doc = "Field `DAR` reader - Disable Automatic Retransmission (read/write, write protection)"]
pub type DAR_R = crate::BitReader<DARSELECT_A>;
#[doc = "Disable Automatic Retransmission (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DARSELECT_A {
    #[doc = "0: Automatic retransmission of messages not transmitted successfully enabled."]
    AUTO_RETX = 0,
    #[doc = "1: Automatic retransmission disabled."]
    NO_AUTO_RETX = 1,
}
impl From<DARSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DARSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DARSELECT_A {
        match self.bits {
            false => DARSELECT_A::AUTO_RETX,
            true => DARSELECT_A::NO_AUTO_RETX,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_RETX`"]
    #[inline(always)]
    pub fn is_auto_retx(&self) -> bool {
        *self == DARSELECT_A::AUTO_RETX
    }
    #[doc = "Checks if the value of the field is `NO_AUTO_RETX`"]
    #[inline(always)]
    pub fn is_no_auto_retx(&self) -> bool {
        *self == DARSELECT_A::NO_AUTO_RETX
    }
}
#[doc = "Field `DAR` writer - Disable Automatic Retransmission (read/write, write protection)"]
pub type DAR_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O, DARSELECT_A>;
impl<'a, const O: u8> DAR_W<'a, O> {
    #[doc = "Automatic retransmission of messages not transmitted successfully enabled."]
    #[inline(always)]
    pub fn auto_retx(self) -> &'a mut W {
        self.variant(DARSELECT_A::AUTO_RETX)
    }
    #[doc = "Automatic retransmission disabled."]
    #[inline(always)]
    pub fn no_auto_retx(self) -> &'a mut W {
        self.variant(DARSELECT_A::NO_AUTO_RETX)
    }
}
#[doc = "Field `TEST` reader - Test Mode Enable (read/write, write protection against '1')"]
pub type TEST_R = crate::BitReader<TESTSELECT_A>;
#[doc = "Test Mode Enable (read/write, write protection against '1')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TESTSELECT_A {
    #[doc = "0: Normal operation, MCAN_TEST register holds reset values."]
    DISABLED = 0,
    #[doc = "1: Test mode, write access to MCAN_TEST register enabled."]
    ENABLED = 1,
}
impl From<TESTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TESTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TESTSELECT_A {
        match self.bits {
            false => TESTSELECT_A::DISABLED,
            true => TESTSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TESTSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TESTSELECT_A::ENABLED
    }
}
#[doc = "Field `TEST` writer - Test Mode Enable (read/write, write protection against '1')"]
pub type TEST_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O, TESTSELECT_A>;
impl<'a, const O: u8> TEST_W<'a, O> {
    #[doc = "Normal operation, MCAN_TEST register holds reset values."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TESTSELECT_A::DISABLED)
    }
    #[doc = "Test mode, write access to MCAN_TEST register enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TESTSELECT_A::ENABLED)
    }
}
#[doc = "Field `FDOE` reader - CAN FD Operation Enable (read/write, write protection)"]
pub type FDOE_R = crate::BitReader<FDOESELECT_A>;
#[doc = "CAN FD Operation Enable (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDOESELECT_A {
    #[doc = "0: FD operation disabled."]
    DISABLED = 0,
    #[doc = "1: FD operation enabled."]
    ENABLED = 1,
}
impl From<FDOESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FDOESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FDOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDOESELECT_A {
        match self.bits {
            false => FDOESELECT_A::DISABLED,
            true => FDOESELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FDOESELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FDOESELECT_A::ENABLED
    }
}
#[doc = "Field `FDOE` writer - CAN FD Operation Enable (read/write, write protection)"]
pub type FDOE_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O, FDOESELECT_A>;
impl<'a, const O: u8> FDOE_W<'a, O> {
    #[doc = "FD operation disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FDOESELECT_A::DISABLED)
    }
    #[doc = "FD operation enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FDOESELECT_A::ENABLED)
    }
}
#[doc = "Field `BRSE` reader - Bit Rate Switching Enable (read/write, write protection)"]
pub type BRSE_R = crate::BitReader<BRSESELECT_A>;
#[doc = "Bit Rate Switching Enable (read/write, write protection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRSESELECT_A {
    #[doc = "0: Bit rate switching for transmissions disabled."]
    DISABLED = 0,
    #[doc = "1: Bit rate switching for transmissions enabled."]
    ENABLED = 1,
}
impl From<BRSESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BRSESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BRSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRSESELECT_A {
        match self.bits {
            false => BRSESELECT_A::DISABLED,
            true => BRSESELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRSESELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRSESELECT_A::ENABLED
    }
}
#[doc = "Field `BRSE` writer - Bit Rate Switching Enable (read/write, write protection)"]
pub type BRSE_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O, BRSESELECT_A>;
impl<'a, const O: u8> BRSE_W<'a, O> {
    #[doc = "Bit rate switching for transmissions disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BRSESELECT_A::DISABLED)
    }
    #[doc = "Bit rate switching for transmissions enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BRSESELECT_A::ENABLED)
    }
}
#[doc = "Field `PXHD` reader - Protocol Exception Event Handling (read/write, write protection)"]
pub type PXHD_R = crate::BitReader;
#[doc = "Field `PXHD` writer - Protocol Exception Event Handling (read/write, write protection)"]
pub type PXHD_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O>;
#[doc = "Field `EFBI` reader - Edge Filtering during Bus Integration (read/write, write protection)"]
pub type EFBI_R = crate::BitReader;
#[doc = "Field `EFBI` writer - Edge Filtering during Bus Integration (read/write, write protection)"]
pub type EFBI_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O>;
#[doc = "Field `TXP` reader - Transmit Pause (read/write, write protection)"]
pub type TXP_R = crate::BitReader;
#[doc = "Field `TXP` writer - Transmit Pause (read/write, write protection)"]
pub type TXP_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O>;
#[doc = "Field `NISO` reader - Non-ISO Operation"]
pub type NISO_R = crate::BitReader;
#[doc = "Field `NISO` writer - Non-ISO Operation"]
pub type NISO_W<'a, const O: u8> = crate::BitWriter<'a, CCCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Initialization (read/write)"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Restricted Operation Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge (read-only)"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Stop Request (read/write)"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission (read/write, write protection)"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable (read/write, write protection against '1')"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CAN FD Operation Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit Rate Switching Enable (read/write, write protection)"]
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol Exception Event Handling (read/write, write protection)"]
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration (read/write, write protection)"]
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Pause (read/write, write protection)"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non-ISO Operation"]
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<0> {
        INIT_W::new(self)
    }
    #[doc = "Bit 1 - Configuration Change Enable (read/write, write protection)"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<1> {
        CCE_W::new(self)
    }
    #[doc = "Bit 2 - Restricted Operation Mode (read/write, write protection against '1')"]
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> ASM_W<2> {
        ASM_W::new(self)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge (read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn csa(&mut self) -> CSA_W<3> {
        CSA_W::new(self)
    }
    #[doc = "Bit 4 - Clock Stop Request (read/write)"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<4> {
        CSR_W::new(self)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode (read/write, write protection against '1')"]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<5> {
        MON_W::new(self)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission (read/write, write protection)"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<6> {
        DAR_W::new(self)
    }
    #[doc = "Bit 7 - Test Mode Enable (read/write, write protection against '1')"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TEST_W<7> {
        TEST_W::new(self)
    }
    #[doc = "Bit 8 - CAN FD Operation Enable (read/write, write protection)"]
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FDOE_W<8> {
        FDOE_W::new(self)
    }
    #[doc = "Bit 9 - Bit Rate Switching Enable (read/write, write protection)"]
    #[inline(always)]
    #[must_use]
    pub fn brse(&mut self) -> BRSE_W<9> {
        BRSE_W::new(self)
    }
    #[doc = "Bit 12 - Protocol Exception Event Handling (read/write, write protection)"]
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PXHD_W<12> {
        PXHD_W::new(self)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration (read/write, write protection)"]
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EFBI_W<13> {
        EFBI_W::new(self)
    }
    #[doc = "Bit 14 - Transmit Pause (read/write, write protection)"]
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TXP_W<14> {
        TXP_W::new(self)
    }
    #[doc = "Bit 15 - Non-ISO Operation"]
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NISO_W<15> {
        NISO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccr](index.html) module"]
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cccr::R](R) reader structure"]
impl crate::Readable for CCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cccr::W](W) writer structure"]
impl crate::Writable for CCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCCR to value 0"]
impl crate::Resettable for CCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
