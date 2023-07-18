#[doc = "Register `US_MR_SPI_MODE` reader"]
pub struct R(crate::R<US_MR_SPI_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_MR_SPI_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_MR_SPI_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_MR_SPI_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_MR_SPI_MODE` writer"]
pub struct W(crate::W<US_MR_SPI_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_MR_SPI_MODE_SPEC>;
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
impl From<crate::W<US_MR_SPI_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_MR_SPI_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART_MODE` reader - USART Mode of Operation"]
pub type USART_MODE_R = crate::FieldReader<USART_MODESELECT_A>;
#[doc = "USART Mode of Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART_MODESELECT_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: RS485"]
    RS485 = 1,
    #[doc = "2: Hardware handshaking"]
    HW_HANDSHAKING = 2,
    #[doc = "3: Modem"]
    MODEM = 3,
    #[doc = "4: IS07816 Protocol: T = 0"]
    IS07816_T_0 = 4,
    #[doc = "6: IS07816 Protocol: T = 1"]
    IS07816_T_1 = 6,
    #[doc = "8: IrDA"]
    IRDA = 8,
    #[doc = "9: LON"]
    LON = 9,
    #[doc = "10: LIN Master mode"]
    LIN_MASTER = 10,
    #[doc = "11: LIN Slave mode"]
    LIN_SLAVE = 11,
    #[doc = "14: SPI Master mode (CLKO must be written to 1 and USCLKS = 0, 1 or 2)"]
    SPI_MASTER = 14,
    #[doc = "15: SPI Slave mode"]
    SPI_SLAVE = 15,
}
impl From<USART_MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: USART_MODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART_MODESELECT_A {
    type Ux = u8;
}
impl USART_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USART_MODESELECT_A> {
        match self.bits {
            0 => Some(USART_MODESELECT_A::NORMAL),
            1 => Some(USART_MODESELECT_A::RS485),
            2 => Some(USART_MODESELECT_A::HW_HANDSHAKING),
            3 => Some(USART_MODESELECT_A::MODEM),
            4 => Some(USART_MODESELECT_A::IS07816_T_0),
            6 => Some(USART_MODESELECT_A::IS07816_T_1),
            8 => Some(USART_MODESELECT_A::IRDA),
            9 => Some(USART_MODESELECT_A::LON),
            10 => Some(USART_MODESELECT_A::LIN_MASTER),
            11 => Some(USART_MODESELECT_A::LIN_SLAVE),
            14 => Some(USART_MODESELECT_A::SPI_MASTER),
            15 => Some(USART_MODESELECT_A::SPI_SLAVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == USART_MODESELECT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RS485`"]
    #[inline(always)]
    pub fn is_rs485(&self) -> bool {
        *self == USART_MODESELECT_A::RS485
    }
    #[doc = "Checks if the value of the field is `HW_HANDSHAKING`"]
    #[inline(always)]
    pub fn is_hw_handshaking(&self) -> bool {
        *self == USART_MODESELECT_A::HW_HANDSHAKING
    }
    #[doc = "Checks if the value of the field is `MODEM`"]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == USART_MODESELECT_A::MODEM
    }
    #[doc = "Checks if the value of the field is `IS07816_T_0`"]
    #[inline(always)]
    pub fn is_is07816_t_0(&self) -> bool {
        *self == USART_MODESELECT_A::IS07816_T_0
    }
    #[doc = "Checks if the value of the field is `IS07816_T_1`"]
    #[inline(always)]
    pub fn is_is07816_t_1(&self) -> bool {
        *self == USART_MODESELECT_A::IS07816_T_1
    }
    #[doc = "Checks if the value of the field is `IRDA`"]
    #[inline(always)]
    pub fn is_irda(&self) -> bool {
        *self == USART_MODESELECT_A::IRDA
    }
    #[doc = "Checks if the value of the field is `LON`"]
    #[inline(always)]
    pub fn is_lon(&self) -> bool {
        *self == USART_MODESELECT_A::LON
    }
    #[doc = "Checks if the value of the field is `LIN_MASTER`"]
    #[inline(always)]
    pub fn is_lin_master(&self) -> bool {
        *self == USART_MODESELECT_A::LIN_MASTER
    }
    #[doc = "Checks if the value of the field is `LIN_SLAVE`"]
    #[inline(always)]
    pub fn is_lin_slave(&self) -> bool {
        *self == USART_MODESELECT_A::LIN_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == USART_MODESELECT_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == USART_MODESELECT_A::SPI_SLAVE
    }
}
#[doc = "Field `USART_MODE` writer - USART Mode of Operation"]
pub type USART_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, US_MR_SPI_MODE_SPEC, 4, O, USART_MODESELECT_A>;
impl<'a, const O: u8> USART_MODE_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(USART_MODESELECT_A::NORMAL)
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn rs485(self) -> &'a mut W {
        self.variant(USART_MODESELECT_A::RS485)
    }
    #[doc = "Hardware handshaking"]
    #[inline(always)]
    pub fn hw_handshaking(self) -> &'a mut W {
        self.variant(USART_MODESELECT_A::HW_HANDSHAKING)
    }
    #[doc = "Modem"]
    #[inline(always)]
    pub fn modem(self) -> &'a mut W {
        self.variant(USART_MODESELECT_A::MODEM)
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline(always)]
    pub fn is07816_t_0(self) -> &'a mut W {
        self.variant(USART_MODESELECT_A::IS07816_T_0)
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline(always)]
    pub fn is07816_t_1(self) -> &'a mut W {
        self.variant(USART_MODESELECT_A::IS07816_T_1)
    }
    #[doc = "IrDA"]
    #[inline(always)]
    pub fn irda(self) -> &'a mut W {
        self.variant(USART_MODESELECT_A::IRDA)
    }
    #[doc = "LON"]
    #[inline(always)]
    pub fn lon(self) -> &'a mut W {
        self.variant(USART_MODESELECT_A::LON)
    }
    #[doc = "LIN Master mode"]
    #[inline(always)]
    pub fn lin_master(self) -> &'a mut W {
        self.variant(USART_MODESELECT_A::LIN_MASTER)
    }
    #[doc = "LIN Slave mode"]
    #[inline(always)]
    pub fn lin_slave(self) -> &'a mut W {
        self.variant(USART_MODESELECT_A::LIN_SLAVE)
    }
    #[doc = "SPI Master mode (CLKO must be written to 1 and USCLKS = 0, 1 or 2)"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(USART_MODESELECT_A::SPI_MASTER)
    }
    #[doc = "SPI Slave mode"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(USART_MODESELECT_A::SPI_SLAVE)
    }
}
#[doc = "Field `USCLKS` reader - Clock Selection"]
pub type USCLKS_R = crate::FieldReader<USCLKSSELECT_A>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USCLKSSELECT_A {
    #[doc = "0: Peripheral clock is selected"]
    MCK = 0,
    #[doc = "1: Peripheral clock divided (DIV = 8) is selected"]
    DIV = 1,
    #[doc = "2: PMC programmable clock (PCK) is selected. If the SCK pin is driven (CLKO = 1), the CD field must be greater than 1."]
    PCK = 2,
    #[doc = "3: Serial clock (SCK) is selected"]
    SCK = 3,
}
impl From<USCLKSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: USCLKSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USCLKSSELECT_A {
    type Ux = u8;
}
impl USCLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCLKSSELECT_A {
        match self.bits {
            0 => USCLKSSELECT_A::MCK,
            1 => USCLKSSELECT_A::DIV,
            2 => USCLKSSELECT_A::PCK,
            3 => USCLKSSELECT_A::SCK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == USCLKSSELECT_A::MCK
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == USCLKSSELECT_A::DIV
    }
    #[doc = "Checks if the value of the field is `PCK`"]
    #[inline(always)]
    pub fn is_pck(&self) -> bool {
        *self == USCLKSSELECT_A::PCK
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == USCLKSSELECT_A::SCK
    }
}
#[doc = "Field `USCLKS` writer - Clock Selection"]
pub type USCLKS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, US_MR_SPI_MODE_SPEC, 2, O, USCLKSSELECT_A>;
impl<'a, const O: u8> USCLKS_W<'a, O> {
    #[doc = "Peripheral clock is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(USCLKSSELECT_A::MCK)
    }
    #[doc = "Peripheral clock divided (DIV = 8) is selected"]
    #[inline(always)]
    pub fn div(self) -> &'a mut W {
        self.variant(USCLKSSELECT_A::DIV)
    }
    #[doc = "PMC programmable clock (PCK) is selected. If the SCK pin is driven (CLKO = 1), the CD field must be greater than 1."]
    #[inline(always)]
    pub fn pck(self) -> &'a mut W {
        self.variant(USCLKSSELECT_A::PCK)
    }
    #[doc = "Serial clock (SCK) is selected"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut W {
        self.variant(USCLKSSELECT_A::SCK)
    }
}
#[doc = "Field `CHRL` reader - Character Length"]
pub type CHRL_R = crate::FieldReader<CHRLSELECT_A>;
#[doc = "Character Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHRLSELECT_A {
    #[doc = "0: Character length is 5 bits"]
    _5_BIT = 0,
    #[doc = "1: Character length is 6 bits"]
    _6_BIT = 1,
    #[doc = "2: Character length is 7 bits"]
    _7_BIT = 2,
    #[doc = "3: Character length is 8 bits"]
    _8_BIT = 3,
}
impl From<CHRLSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CHRLSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHRLSELECT_A {
    type Ux = u8;
}
impl CHRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHRLSELECT_A {
        match self.bits {
            0 => CHRLSELECT_A::_5_BIT,
            1 => CHRLSELECT_A::_6_BIT,
            2 => CHRLSELECT_A::_7_BIT,
            3 => CHRLSELECT_A::_8_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == CHRLSELECT_A::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == CHRLSELECT_A::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == CHRLSELECT_A::_7_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == CHRLSELECT_A::_8_BIT
    }
}
#[doc = "Field `CHRL` writer - Character Length"]
pub type CHRL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, US_MR_SPI_MODE_SPEC, 2, O, CHRLSELECT_A>;
impl<'a, const O: u8> CHRL_W<'a, O> {
    #[doc = "Character length is 5 bits"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(CHRLSELECT_A::_5_BIT)
    }
    #[doc = "Character length is 6 bits"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(CHRLSELECT_A::_6_BIT)
    }
    #[doc = "Character length is 7 bits"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(CHRLSELECT_A::_7_BIT)
    }
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(CHRLSELECT_A::_8_BIT)
    }
}
#[doc = "Field `CPHA` reader - SPI Clock Phase"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - SPI Clock Phase"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, US_MR_SPI_MODE_SPEC, O>;
#[doc = "Field `CPOL` reader - SPI Clock Polarity"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - SPI Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, US_MR_SPI_MODE_SPEC, O>;
#[doc = "Field `CLKO` reader - Clock Output Select"]
pub type CLKO_R = crate::BitReader;
#[doc = "Field `CLKO` writer - Clock Output Select"]
pub type CLKO_W<'a, const O: u8> = crate::BitWriter<'a, US_MR_SPI_MODE_SPEC, O>;
#[doc = "Field `WRDBT` reader - Wait Read Data Before Transfer"]
pub type WRDBT_R = crate::BitReader;
#[doc = "Field `WRDBT` writer - Wait Read Data Before Transfer"]
pub type WRDBT_W<'a, const O: u8> = crate::BitWriter<'a, US_MR_SPI_MODE_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&self) -> USART_MODE_R {
        USART_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&self) -> USCLKS_R {
        USCLKS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    pub fn chrl(&self) -> CHRL_R {
        CHRL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&self) -> CLKO_R {
        CLKO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    pub fn wrdbt(&self) -> WRDBT_R {
        WRDBT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    #[must_use]
    pub fn usart_mode(&mut self) -> USART_MODE_W<0> {
        USART_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn usclks(&mut self) -> USCLKS_W<4> {
        USCLKS_W::new(self)
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    #[must_use]
    pub fn chrl(&mut self) -> CHRL_W<6> {
        CHRL_W::new(self)
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<8> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<16> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn clko(&mut self) -> CLKO_W<18> {
        CLKO_W::new(self)
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn wrdbt(&mut self) -> WRDBT_W<20> {
        WRDBT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_mr_spi_mode](index.html) module"]
pub struct US_MR_SPI_MODE_SPEC;
impl crate::RegisterSpec for US_MR_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_mr_spi_mode::R](R) reader structure"]
impl crate::Readable for US_MR_SPI_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_mr_spi_mode::W](W) writer structure"]
impl crate::Writable for US_MR_SPI_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_MR_SPI_MODE to value 0"]
impl crate::Resettable for US_MR_SPI_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
