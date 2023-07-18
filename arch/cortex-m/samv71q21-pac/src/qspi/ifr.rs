#[doc = "Register `IFR` reader"]
pub struct R(crate::R<IFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFR` writer"]
pub struct W(crate::W<IFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFR_SPEC>;
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
impl From<crate::W<IFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIDTH` reader - Width of Instruction Code, Address, Option Code and Data"]
pub type WIDTH_R = crate::FieldReader<WIDTHSELECT_A>;
#[doc = "Width of Instruction Code, Address, Option Code and Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WIDTHSELECT_A {
    #[doc = "0: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    SINGLE_BIT_SPI = 0,
    #[doc = "1: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    DUAL_OUTPUT = 1,
    #[doc = "2: Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    QUAD_OUTPUT = 2,
    #[doc = "3: Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_IO = 3,
    #[doc = "4: Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_IO = 4,
    #[doc = "5: Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_CMD = 5,
    #[doc = "6: Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_CMD = 6,
}
impl From<WIDTHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WIDTHSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WIDTHSELECT_A {
    type Ux = u8;
}
impl WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WIDTHSELECT_A> {
        match self.bits {
            0 => Some(WIDTHSELECT_A::SINGLE_BIT_SPI),
            1 => Some(WIDTHSELECT_A::DUAL_OUTPUT),
            2 => Some(WIDTHSELECT_A::QUAD_OUTPUT),
            3 => Some(WIDTHSELECT_A::DUAL_IO),
            4 => Some(WIDTHSELECT_A::QUAD_IO),
            5 => Some(WIDTHSELECT_A::DUAL_CMD),
            6 => Some(WIDTHSELECT_A::QUAD_CMD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_BIT_SPI`"]
    #[inline(always)]
    pub fn is_single_bit_spi(&self) -> bool {
        *self == WIDTHSELECT_A::SINGLE_BIT_SPI
    }
    #[doc = "Checks if the value of the field is `DUAL_OUTPUT`"]
    #[inline(always)]
    pub fn is_dual_output(&self) -> bool {
        *self == WIDTHSELECT_A::DUAL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `QUAD_OUTPUT`"]
    #[inline(always)]
    pub fn is_quad_output(&self) -> bool {
        *self == WIDTHSELECT_A::QUAD_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        *self == WIDTHSELECT_A::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        *self == WIDTHSELECT_A::QUAD_IO
    }
    #[doc = "Checks if the value of the field is `DUAL_CMD`"]
    #[inline(always)]
    pub fn is_dual_cmd(&self) -> bool {
        *self == WIDTHSELECT_A::DUAL_CMD
    }
    #[doc = "Checks if the value of the field is `QUAD_CMD`"]
    #[inline(always)]
    pub fn is_quad_cmd(&self) -> bool {
        *self == WIDTHSELECT_A::QUAD_CMD
    }
}
#[doc = "Field `WIDTH` writer - Width of Instruction Code, Address, Option Code and Data"]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, IFR_SPEC, 3, O, WIDTHSELECT_A>;
impl<'a, const O: u8> WIDTH_W<'a, O> {
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    #[inline(always)]
    pub fn single_bit_spi(self) -> &'a mut W {
        self.variant(WIDTHSELECT_A::SINGLE_BIT_SPI)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_output(self) -> &'a mut W {
        self.variant(WIDTHSELECT_A::DUAL_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_output(self) -> &'a mut W {
        self.variant(WIDTHSELECT_A::QUAD_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(WIDTHSELECT_A::DUAL_IO)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(WIDTHSELECT_A::QUAD_IO)
    }
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_cmd(self) -> &'a mut W {
        self.variant(WIDTHSELECT_A::DUAL_CMD)
    }
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_cmd(self) -> &'a mut W {
        self.variant(WIDTHSELECT_A::QUAD_CMD)
    }
}
#[doc = "Field `INSTEN` reader - Instruction Enable"]
pub type INSTEN_R = crate::BitReader;
#[doc = "Field `INSTEN` writer - Instruction Enable"]
pub type INSTEN_W<'a, const O: u8> = crate::BitWriter<'a, IFR_SPEC, O>;
#[doc = "Field `ADDREN` reader - Address Enable"]
pub type ADDREN_R = crate::BitReader;
#[doc = "Field `ADDREN` writer - Address Enable"]
pub type ADDREN_W<'a, const O: u8> = crate::BitWriter<'a, IFR_SPEC, O>;
#[doc = "Field `OPTEN` reader - Option Enable"]
pub type OPTEN_R = crate::BitReader;
#[doc = "Field `OPTEN` writer - Option Enable"]
pub type OPTEN_W<'a, const O: u8> = crate::BitWriter<'a, IFR_SPEC, O>;
#[doc = "Field `DATAEN` reader - Data Enable"]
pub type DATAEN_R = crate::BitReader;
#[doc = "Field `DATAEN` writer - Data Enable"]
pub type DATAEN_W<'a, const O: u8> = crate::BitWriter<'a, IFR_SPEC, O>;
#[doc = "Field `OPTL` reader - Option Code Length"]
pub type OPTL_R = crate::FieldReader<OPTLSELECT_A>;
#[doc = "Option Code Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPTLSELECT_A {
    #[doc = "0: The option code is 1 bit long."]
    OPTION_1BIT = 0,
    #[doc = "1: The option code is 2 bits long."]
    OPTION_2BIT = 1,
    #[doc = "2: The option code is 4 bits long."]
    OPTION_4BIT = 2,
    #[doc = "3: The option code is 8 bits long."]
    OPTION_8BIT = 3,
}
impl From<OPTLSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: OPTLSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OPTLSELECT_A {
    type Ux = u8;
}
impl OPTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPTLSELECT_A {
        match self.bits {
            0 => OPTLSELECT_A::OPTION_1BIT,
            1 => OPTLSELECT_A::OPTION_2BIT,
            2 => OPTLSELECT_A::OPTION_4BIT,
            3 => OPTLSELECT_A::OPTION_8BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPTION_1BIT`"]
    #[inline(always)]
    pub fn is_option_1bit(&self) -> bool {
        *self == OPTLSELECT_A::OPTION_1BIT
    }
    #[doc = "Checks if the value of the field is `OPTION_2BIT`"]
    #[inline(always)]
    pub fn is_option_2bit(&self) -> bool {
        *self == OPTLSELECT_A::OPTION_2BIT
    }
    #[doc = "Checks if the value of the field is `OPTION_4BIT`"]
    #[inline(always)]
    pub fn is_option_4bit(&self) -> bool {
        *self == OPTLSELECT_A::OPTION_4BIT
    }
    #[doc = "Checks if the value of the field is `OPTION_8BIT`"]
    #[inline(always)]
    pub fn is_option_8bit(&self) -> bool {
        *self == OPTLSELECT_A::OPTION_8BIT
    }
}
#[doc = "Field `OPTL` writer - Option Code Length"]
pub type OPTL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, IFR_SPEC, 2, O, OPTLSELECT_A>;
impl<'a, const O: u8> OPTL_W<'a, O> {
    #[doc = "The option code is 1 bit long."]
    #[inline(always)]
    pub fn option_1bit(self) -> &'a mut W {
        self.variant(OPTLSELECT_A::OPTION_1BIT)
    }
    #[doc = "The option code is 2 bits long."]
    #[inline(always)]
    pub fn option_2bit(self) -> &'a mut W {
        self.variant(OPTLSELECT_A::OPTION_2BIT)
    }
    #[doc = "The option code is 4 bits long."]
    #[inline(always)]
    pub fn option_4bit(self) -> &'a mut W {
        self.variant(OPTLSELECT_A::OPTION_4BIT)
    }
    #[doc = "The option code is 8 bits long."]
    #[inline(always)]
    pub fn option_8bit(self) -> &'a mut W {
        self.variant(OPTLSELECT_A::OPTION_8BIT)
    }
}
#[doc = "Field `ADDRL` reader - Address Length"]
pub type ADDRL_R = crate::BitReader<ADDRLSELECT_A>;
#[doc = "Address Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRLSELECT_A {
    #[doc = "0: The address is 24 bits long."]
    _24_BIT = 0,
    #[doc = "1: The address is 32 bits long."]
    _32_BIT = 1,
}
impl From<ADDRLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRLSELECT_A {
        match self.bits {
            false => ADDRLSELECT_A::_24_BIT,
            true => ADDRLSELECT_A::_32_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_24_BIT`"]
    #[inline(always)]
    pub fn is_24_bit(&self) -> bool {
        *self == ADDRLSELECT_A::_24_BIT
    }
    #[doc = "Checks if the value of the field is `_32_BIT`"]
    #[inline(always)]
    pub fn is_32_bit(&self) -> bool {
        *self == ADDRLSELECT_A::_32_BIT
    }
}
#[doc = "Field `ADDRL` writer - Address Length"]
pub type ADDRL_W<'a, const O: u8> = crate::BitWriter<'a, IFR_SPEC, O, ADDRLSELECT_A>;
impl<'a, const O: u8> ADDRL_W<'a, O> {
    #[doc = "The address is 24 bits long."]
    #[inline(always)]
    pub fn _24_bit(self) -> &'a mut W {
        self.variant(ADDRLSELECT_A::_24_BIT)
    }
    #[doc = "The address is 32 bits long."]
    #[inline(always)]
    pub fn _32_bit(self) -> &'a mut W {
        self.variant(ADDRLSELECT_A::_32_BIT)
    }
}
#[doc = "Field `TFRTYP` reader - Data Transfer Type"]
pub type TFRTYP_R = crate::FieldReader<TFRTYPSELECT_A>;
#[doc = "Data Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TFRTYPSELECT_A {
    #[doc = "0: Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial Flash memory is not possible."]
    TRSFR_READ = 0,
    #[doc = "1: Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial Flash memory is possible."]
    TRSFR_READ_MEMORY = 1,
    #[doc = "2: Write transfer into the serial memory.Scrambling is not performed."]
    TRSFR_WRITE = 2,
    #[doc = "3: Write data transfer into the serial memory.If enabled, scrambling is performed."]
    TRSFR_WRITE_MEMORY = 3,
}
impl From<TFRTYPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TFRTYPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TFRTYPSELECT_A {
    type Ux = u8;
}
impl TFRTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFRTYPSELECT_A {
        match self.bits {
            0 => TFRTYPSELECT_A::TRSFR_READ,
            1 => TFRTYPSELECT_A::TRSFR_READ_MEMORY,
            2 => TFRTYPSELECT_A::TRSFR_WRITE,
            3 => TFRTYPSELECT_A::TRSFR_WRITE_MEMORY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRSFR_READ`"]
    #[inline(always)]
    pub fn is_trsfr_read(&self) -> bool {
        *self == TFRTYPSELECT_A::TRSFR_READ
    }
    #[doc = "Checks if the value of the field is `TRSFR_READ_MEMORY`"]
    #[inline(always)]
    pub fn is_trsfr_read_memory(&self) -> bool {
        *self == TFRTYPSELECT_A::TRSFR_READ_MEMORY
    }
    #[doc = "Checks if the value of the field is `TRSFR_WRITE`"]
    #[inline(always)]
    pub fn is_trsfr_write(&self) -> bool {
        *self == TFRTYPSELECT_A::TRSFR_WRITE
    }
    #[doc = "Checks if the value of the field is `TRSFR_WRITE_MEMORY`"]
    #[inline(always)]
    pub fn is_trsfr_write_memory(&self) -> bool {
        *self == TFRTYPSELECT_A::TRSFR_WRITE_MEMORY
    }
}
#[doc = "Field `TFRTYP` writer - Data Transfer Type"]
pub type TFRTYP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, IFR_SPEC, 2, O, TFRTYPSELECT_A>;
impl<'a, const O: u8> TFRTYP_W<'a, O> {
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial Flash memory is not possible."]
    #[inline(always)]
    pub fn trsfr_read(self) -> &'a mut W {
        self.variant(TFRTYPSELECT_A::TRSFR_READ)
    }
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial Flash memory is possible."]
    #[inline(always)]
    pub fn trsfr_read_memory(self) -> &'a mut W {
        self.variant(TFRTYPSELECT_A::TRSFR_READ_MEMORY)
    }
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    #[inline(always)]
    pub fn trsfr_write(self) -> &'a mut W {
        self.variant(TFRTYPSELECT_A::TRSFR_WRITE)
    }
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    #[inline(always)]
    pub fn trsfr_write_memory(self) -> &'a mut W {
        self.variant(TFRTYPSELECT_A::TRSFR_WRITE_MEMORY)
    }
}
#[doc = "Field `CRM` reader - Continuous Read Mode"]
pub type CRM_R = crate::BitReader<CRMSELECT_A>;
#[doc = "Continuous Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRMSELECT_A {
    #[doc = "0: The Continuous Read mode is disabled."]
    DISABLED = 0,
    #[doc = "1: The Continuous Read mode is enabled."]
    ENABLED = 1,
}
impl From<CRMSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CRMSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRMSELECT_A {
        match self.bits {
            false => CRMSELECT_A::DISABLED,
            true => CRMSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRMSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRMSELECT_A::ENABLED
    }
}
#[doc = "Field `CRM` writer - Continuous Read Mode"]
pub type CRM_W<'a, const O: u8> = crate::BitWriter<'a, IFR_SPEC, O, CRMSELECT_A>;
impl<'a, const O: u8> CRM_W<'a, O> {
    #[doc = "The Continuous Read mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRMSELECT_A::DISABLED)
    }
    #[doc = "The Continuous Read mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRMSELECT_A::ENABLED)
    }
}
#[doc = "Field `NBDUM` reader - Number Of Dummy Cycles"]
pub type NBDUM_R = crate::FieldReader;
#[doc = "Field `NBDUM` writer - Number Of Dummy Cycles"]
pub type NBDUM_W<'a, const O: u8> = crate::FieldWriter<'a, IFR_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:2 - Width of Instruction Code, Address, Option Code and Data"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    pub fn insten(&self) -> INSTEN_R {
        INSTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    pub fn addren(&self) -> ADDREN_R {
        ADDREN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    pub fn opten(&self) -> OPTEN_R {
        OPTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    pub fn dataen(&self) -> DATAEN_R {
        DATAEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    pub fn optl(&self) -> OPTL_R {
        OPTL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    pub fn addrl(&self) -> ADDRL_R {
        ADDRL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    pub fn tfrtyp(&self) -> TFRTYP_R {
        TFRTYP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    pub fn crm(&self) -> CRM_R {
        CRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Number Of Dummy Cycles"]
    #[inline(always)]
    pub fn nbdum(&self) -> NBDUM_R {
        NBDUM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Width of Instruction Code, Address, Option Code and Data"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<0> {
        WIDTH_W::new(self)
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    #[must_use]
    pub fn insten(&mut self) -> INSTEN_W<4> {
        INSTEN_W::new(self)
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn addren(&mut self) -> ADDREN_W<5> {
        ADDREN_W::new(self)
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opten(&mut self) -> OPTEN_W<6> {
        OPTEN_W::new(self)
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dataen(&mut self) -> DATAEN_W<7> {
        DATAEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    #[must_use]
    pub fn optl(&mut self) -> OPTL_W<8> {
        OPTL_W::new(self)
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    #[must_use]
    pub fn addrl(&mut self) -> ADDRL_W<10> {
        ADDRL_W::new(self)
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    #[must_use]
    pub fn tfrtyp(&mut self) -> TFRTYP_W<12> {
        TFRTYP_W::new(self)
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn crm(&mut self) -> CRM_W<14> {
        CRM_W::new(self)
    }
    #[doc = "Bits 16:20 - Number Of Dummy Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn nbdum(&mut self) -> NBDUM_W<16> {
        NBDUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifr](index.html) module"]
pub struct IFR_SPEC;
impl crate::RegisterSpec for IFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifr::R](R) reader structure"]
impl crate::Readable for IFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifr::W](W) writer structure"]
impl crate::Writable for IFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFR to value 0"]
impl crate::Resettable for IFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
