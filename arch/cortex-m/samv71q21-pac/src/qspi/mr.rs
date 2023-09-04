#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `SMM` reader - Serial Memory Mode"]
pub type SMM_R = crate::BitReader<SMMSELECT_A>;
#[doc = "Serial Memory Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMMSELECT_A {
    #[doc = "0: The QSPI is in SPI mode."]
    SPI = 0,
    #[doc = "1: The QSPI is in Serial Memory mode."]
    MEMORY = 1,
}
impl From<SMMSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SMMSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMMSELECT_A {
        match self.bits {
            false => SMMSELECT_A::SPI,
            true => SMMSELECT_A::MEMORY,
        }
    }
    #[doc = "The QSPI is in SPI mode."]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == SMMSELECT_A::SPI
    }
    #[doc = "The QSPI is in Serial Memory mode."]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        *self == SMMSELECT_A::MEMORY
    }
}
#[doc = "Field `SMM` writer - Serial Memory Mode"]
pub type SMM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SMMSELECT_A>;
impl<'a, REG, const O: u8> SMM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The QSPI is in SPI mode."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(SMMSELECT_A::SPI)
    }
    #[doc = "The QSPI is in Serial Memory mode."]
    #[inline(always)]
    pub fn memory(self) -> &'a mut crate::W<REG> {
        self.variant(SMMSELECT_A::MEMORY)
    }
}
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub type LLB_R = crate::BitReader<LLBSELECT_A>;
#[doc = "Local Loopback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LLBSELECT_A {
    #[doc = "0: Local loopback path disabled."]
    DISABLED = 0,
    #[doc = "1: Local loopback path enabled."]
    ENABLED = 1,
}
impl From<LLBSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LLBSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LLBSELECT_A {
        match self.bits {
            false => LLBSELECT_A::DISABLED,
            true => LLBSELECT_A::ENABLED,
        }
    }
    #[doc = "Local loopback path disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LLBSELECT_A::DISABLED
    }
    #[doc = "Local loopback path enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LLBSELECT_A::ENABLED
    }
}
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub type LLB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LLBSELECT_A>;
impl<'a, REG, const O: u8> LLB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Local loopback path disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LLBSELECT_A::DISABLED)
    }
    #[doc = "Local loopback path enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LLBSELECT_A::ENABLED)
    }
}
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub type WDRBT_R = crate::BitReader<WDRBTSELECT_A>;
#[doc = "Wait Data Read Before Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDRBTSELECT_A {
    #[doc = "0: No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    DISABLED = 0,
    #[doc = "1: In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    ENABLED = 1,
}
impl From<WDRBTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WDRBTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WDRBT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDRBTSELECT_A {
        match self.bits {
            false => WDRBTSELECT_A::DISABLED,
            true => WDRBTSELECT_A::ENABLED,
        }
    }
    #[doc = "No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDRBTSELECT_A::DISABLED
    }
    #[doc = "In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDRBTSELECT_A::ENABLED
    }
}
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub type WDRBT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDRBTSELECT_A>;
impl<'a, REG, const O: u8> WDRBT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDRBTSELECT_A::DISABLED)
    }
    #[doc = "In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDRBTSELECT_A::ENABLED)
    }
}
#[doc = "Field `CSMODE` reader - Chip Select Mode"]
pub type CSMODE_R = crate::FieldReader<CSMODESELECT_A>;
#[doc = "Chip Select Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSMODESELECT_A {
    #[doc = "0: The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    NOT_RELOADED = 0,
    #[doc = "1: The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    LASTXFER = 1,
    #[doc = "2: The chip select is deasserted systematically after each transfer."]
    SYSTEMATICALLY = 2,
}
impl From<CSMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSMODESELECT_A {
    type Ux = u8;
}
impl CSMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSMODESELECT_A> {
        match self.bits {
            0 => Some(CSMODESELECT_A::NOT_RELOADED),
            1 => Some(CSMODESELECT_A::LASTXFER),
            2 => Some(CSMODESELECT_A::SYSTEMATICALLY),
            _ => None,
        }
    }
    #[doc = "The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    #[inline(always)]
    pub fn is_not_reloaded(&self) -> bool {
        *self == CSMODESELECT_A::NOT_RELOADED
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    #[inline(always)]
    pub fn is_lastxfer(&self) -> bool {
        *self == CSMODESELECT_A::LASTXFER
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline(always)]
    pub fn is_systematically(&self) -> bool {
        *self == CSMODESELECT_A::SYSTEMATICALLY
    }
}
#[doc = "Field `CSMODE` writer - Chip Select Mode"]
pub type CSMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CSMODESELECT_A>;
impl<'a, REG, const O: u8> CSMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    #[inline(always)]
    pub fn not_reloaded(self) -> &'a mut crate::W<REG> {
        self.variant(CSMODESELECT_A::NOT_RELOADED)
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    #[inline(always)]
    pub fn lastxfer(self) -> &'a mut crate::W<REG> {
        self.variant(CSMODESELECT_A::LASTXFER)
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline(always)]
    pub fn systematically(self) -> &'a mut crate::W<REG> {
        self.variant(CSMODESELECT_A::SYSTEMATICALLY)
    }
}
#[doc = "Field `NBBITS` reader - Number Of Bits Per Transfer"]
pub type NBBITS_R = crate::FieldReader<NBBITSSELECT_A>;
#[doc = "Number Of Bits Per Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NBBITSSELECT_A {
    #[doc = "0: 8 bits for transfer"]
    _8_BIT = 0,
    #[doc = "8: 16 bits for transfer"]
    _16_BIT = 8,
}
impl From<NBBITSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NBBITSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NBBITSSELECT_A {
    type Ux = u8;
}
impl NBBITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NBBITSSELECT_A> {
        match self.bits {
            0 => Some(NBBITSSELECT_A::_8_BIT),
            8 => Some(NBBITSSELECT_A::_16_BIT),
            _ => None,
        }
    }
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == NBBITSSELECT_A::_8_BIT
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == NBBITSSELECT_A::_16_BIT
    }
}
#[doc = "Field `NBBITS` writer - Number Of Bits Per Transfer"]
pub type NBBITS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, NBBITSSELECT_A>;
impl<'a, REG, const O: u8> NBBITS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(NBBITSSELECT_A::_8_BIT)
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(NBBITSSELECT_A::_16_BIT)
    }
}
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub type DLYBCT_R = crate::FieldReader;
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub type DLYBCT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DLYCS` reader - Minimum Inactive QCS Delay"]
pub type DLYCS_R = crate::FieldReader;
#[doc = "Field `DLYCS` writer - Minimum Inactive QCS Delay"]
pub type DLYCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn smm(&self) -> SMM_R {
        SMM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LLB_R {
        LLB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WDRBT_R {
        WDRBT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&self) -> CSMODE_R {
        CSMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Number Of Bits Per Transfer"]
    #[inline(always)]
    pub fn nbbits(&self) -> NBBITS_R {
        NBBITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DLYBCT_R {
        DLYBCT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Minimum Inactive QCS Delay"]
    #[inline(always)]
    pub fn dlycs(&self) -> DLYCS_R {
        DLYCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    #[must_use]
    pub fn smm(&mut self) -> SMM_W<MR_SPEC, 0> {
        SMM_W::new(self)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn llb(&mut self) -> LLB_W<MR_SPEC, 1> {
        LLB_W::new(self)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn wdrbt(&mut self) -> WDRBT_W<MR_SPEC, 2> {
        WDRBT_W::new(self)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    #[must_use]
    pub fn csmode(&mut self) -> CSMODE_W<MR_SPEC, 4> {
        CSMODE_W::new(self)
    }
    #[doc = "Bits 8:11 - Number Of Bits Per Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn nbbits(&mut self) -> NBBITS_W<MR_SPEC, 8> {
        NBBITS_W::new(self)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dlybct(&mut self) -> DLYBCT_W<MR_SPEC, 16> {
        DLYBCT_W::new(self)
    }
    #[doc = "Bits 24:31 - Minimum Inactive QCS Delay"]
    #[inline(always)]
    #[must_use]
    pub fn dlycs(&mut self) -> DLYCS_W<MR_SPEC, 24> {
        DLYCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
