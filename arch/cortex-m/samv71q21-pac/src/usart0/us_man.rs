#[doc = "Register `US_MAN` reader"]
pub struct R(crate::R<US_MAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_MAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_MAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_MAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_MAN` writer"]
pub struct W(crate::W<US_MAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_MAN_SPEC>;
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
impl From<crate::W<US_MAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_MAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PL` reader - Transmitter Preamble Length"]
pub type TX_PL_R = crate::FieldReader;
#[doc = "Field `TX_PL` writer - Transmitter Preamble Length"]
pub type TX_PL_W<'a, const O: u8> = crate::FieldWriter<'a, US_MAN_SPEC, 4, O>;
#[doc = "Field `TX_PP` reader - Transmitter Preamble Pattern"]
pub type TX_PP_R = crate::FieldReader<TX_PPSELECT_A>;
#[doc = "Transmitter Preamble Pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_PPSELECT_A {
    #[doc = "0: The preamble is composed of '1's"]
    ALL_ONE = 0,
    #[doc = "1: The preamble is composed of '0's"]
    ALL_ZERO = 1,
    #[doc = "2: The preamble is composed of '01's"]
    ZERO_ONE = 2,
    #[doc = "3: The preamble is composed of '10's"]
    ONE_ZERO = 3,
}
impl From<TX_PPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_PPSELECT_A {
    type Ux = u8;
}
impl TX_PP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_PPSELECT_A {
        match self.bits {
            0 => TX_PPSELECT_A::ALL_ONE,
            1 => TX_PPSELECT_A::ALL_ZERO,
            2 => TX_PPSELECT_A::ZERO_ONE,
            3 => TX_PPSELECT_A::ONE_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALL_ONE`"]
    #[inline(always)]
    pub fn is_all_one(&self) -> bool {
        *self == TX_PPSELECT_A::ALL_ONE
    }
    #[doc = "Checks if the value of the field is `ALL_ZERO`"]
    #[inline(always)]
    pub fn is_all_zero(&self) -> bool {
        *self == TX_PPSELECT_A::ALL_ZERO
    }
    #[doc = "Checks if the value of the field is `ZERO_ONE`"]
    #[inline(always)]
    pub fn is_zero_one(&self) -> bool {
        *self == TX_PPSELECT_A::ZERO_ONE
    }
    #[doc = "Checks if the value of the field is `ONE_ZERO`"]
    #[inline(always)]
    pub fn is_one_zero(&self) -> bool {
        *self == TX_PPSELECT_A::ONE_ZERO
    }
}
#[doc = "Field `TX_PP` writer - Transmitter Preamble Pattern"]
pub type TX_PP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, US_MAN_SPEC, 2, O, TX_PPSELECT_A>;
impl<'a, const O: u8> TX_PP_W<'a, O> {
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn all_one(self) -> &'a mut W {
        self.variant(TX_PPSELECT_A::ALL_ONE)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn all_zero(self) -> &'a mut W {
        self.variant(TX_PPSELECT_A::ALL_ZERO)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn zero_one(self) -> &'a mut W {
        self.variant(TX_PPSELECT_A::ZERO_ONE)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn one_zero(self) -> &'a mut W {
        self.variant(TX_PPSELECT_A::ONE_ZERO)
    }
}
#[doc = "Field `TX_MPOL` reader - Transmitter Manchester Polarity"]
pub type TX_MPOL_R = crate::BitReader;
#[doc = "Field `TX_MPOL` writer - Transmitter Manchester Polarity"]
pub type TX_MPOL_W<'a, const O: u8> = crate::BitWriter<'a, US_MAN_SPEC, O>;
#[doc = "Field `RX_PL` reader - Receiver Preamble Length"]
pub type RX_PL_R = crate::FieldReader;
#[doc = "Field `RX_PL` writer - Receiver Preamble Length"]
pub type RX_PL_W<'a, const O: u8> = crate::FieldWriter<'a, US_MAN_SPEC, 4, O>;
#[doc = "Field `RX_PP` reader - Receiver Preamble Pattern detected"]
pub type RX_PP_R = crate::FieldReader<RX_PPSELECT_A>;
#[doc = "Receiver Preamble Pattern detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_PPSELECT_A {
    #[doc = "0: The preamble is composed of '1's"]
    ALL_ONE = 0,
    #[doc = "1: The preamble is composed of '0's"]
    ALL_ZERO = 1,
    #[doc = "2: The preamble is composed of '01's"]
    ZERO_ONE = 2,
    #[doc = "3: The preamble is composed of '10's"]
    ONE_ZERO = 3,
}
impl From<RX_PPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RX_PPSELECT_A {
    type Ux = u8;
}
impl RX_PP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_PPSELECT_A {
        match self.bits {
            0 => RX_PPSELECT_A::ALL_ONE,
            1 => RX_PPSELECT_A::ALL_ZERO,
            2 => RX_PPSELECT_A::ZERO_ONE,
            3 => RX_PPSELECT_A::ONE_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALL_ONE`"]
    #[inline(always)]
    pub fn is_all_one(&self) -> bool {
        *self == RX_PPSELECT_A::ALL_ONE
    }
    #[doc = "Checks if the value of the field is `ALL_ZERO`"]
    #[inline(always)]
    pub fn is_all_zero(&self) -> bool {
        *self == RX_PPSELECT_A::ALL_ZERO
    }
    #[doc = "Checks if the value of the field is `ZERO_ONE`"]
    #[inline(always)]
    pub fn is_zero_one(&self) -> bool {
        *self == RX_PPSELECT_A::ZERO_ONE
    }
    #[doc = "Checks if the value of the field is `ONE_ZERO`"]
    #[inline(always)]
    pub fn is_one_zero(&self) -> bool {
        *self == RX_PPSELECT_A::ONE_ZERO
    }
}
#[doc = "Field `RX_PP` writer - Receiver Preamble Pattern detected"]
pub type RX_PP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, US_MAN_SPEC, 2, O, RX_PPSELECT_A>;
impl<'a, const O: u8> RX_PP_W<'a, O> {
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn all_one(self) -> &'a mut W {
        self.variant(RX_PPSELECT_A::ALL_ONE)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn all_zero(self) -> &'a mut W {
        self.variant(RX_PPSELECT_A::ALL_ZERO)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn zero_one(self) -> &'a mut W {
        self.variant(RX_PPSELECT_A::ZERO_ONE)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn one_zero(self) -> &'a mut W {
        self.variant(RX_PPSELECT_A::ONE_ZERO)
    }
}
#[doc = "Field `RX_MPOL` reader - Receiver Manchester Polarity"]
pub type RX_MPOL_R = crate::BitReader;
#[doc = "Field `RX_MPOL` writer - Receiver Manchester Polarity"]
pub type RX_MPOL_W<'a, const O: u8> = crate::BitWriter<'a, US_MAN_SPEC, O>;
#[doc = "Field `ONE` reader - Must Be Set to 1"]
pub type ONE_R = crate::BitReader;
#[doc = "Field `ONE` writer - Must Be Set to 1"]
pub type ONE_W<'a, const O: u8> = crate::BitWriter<'a, US_MAN_SPEC, O>;
#[doc = "Field `DRIFT` reader - Drift Compensation"]
pub type DRIFT_R = crate::BitReader;
#[doc = "Field `DRIFT` writer - Drift Compensation"]
pub type DRIFT_W<'a, const O: u8> = crate::BitWriter<'a, US_MAN_SPEC, O>;
#[doc = "Field `RXIDLEV` reader - Receiver Idle Value"]
pub type RXIDLEV_R = crate::BitReader;
#[doc = "Field `RXIDLEV` writer - Receiver Idle Value"]
pub type RXIDLEV_W<'a, const O: u8> = crate::BitWriter<'a, US_MAN_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    pub fn tx_pl(&self) -> TX_PL_R {
        TX_PL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    pub fn tx_pp(&self) -> TX_PP_R {
        TX_PP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    pub fn tx_mpol(&self) -> TX_MPOL_R {
        TX_MPOL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    pub fn rx_pl(&self) -> RX_PL_R {
        RX_PL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    pub fn rx_pp(&self) -> RX_PP_R {
        RX_PP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    pub fn rx_mpol(&self) -> RX_MPOL_R {
        RX_MPOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline(always)]
    pub fn drift(&self) -> DRIFT_R {
        DRIFT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Receiver Idle Value"]
    #[inline(always)]
    pub fn rxidlev(&self) -> RXIDLEV_R {
        RXIDLEV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pl(&mut self) -> TX_PL_W<0> {
        TX_PL_W::new(self)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pp(&mut self) -> TX_PP_W<8> {
        TX_PP_W::new(self)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn tx_mpol(&mut self) -> TX_MPOL_W<12> {
        TX_MPOL_W::new(self)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pl(&mut self) -> RX_PL_W<16> {
        RX_PL_W::new(self)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pp(&mut self) -> RX_PP_W<24> {
        RX_PP_W::new(self)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn rx_mpol(&mut self) -> RX_MPOL_W<28> {
        RX_MPOL_W::new(self)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn one(&mut self) -> ONE_W<29> {
        ONE_W::new(self)
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline(always)]
    #[must_use]
    pub fn drift(&mut self) -> DRIFT_W<30> {
        DRIFT_W::new(self)
    }
    #[doc = "Bit 31 - Receiver Idle Value"]
    #[inline(always)]
    #[must_use]
    pub fn rxidlev(&mut self) -> RXIDLEV_W<31> {
        RXIDLEV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manchester Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_man](index.html) module"]
pub struct US_MAN_SPEC;
impl crate::RegisterSpec for US_MAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_man::R](R) reader structure"]
impl crate::Readable for US_MAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_man::W](W) writer structure"]
impl crate::Writable for US_MAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_MAN to value 0"]
impl crate::Resettable for US_MAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
