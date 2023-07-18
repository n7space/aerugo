#[doc = "Register `CMR` reader"]
pub struct R(crate::R<CMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMR` writer"]
pub struct W(crate::W<CMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMR_SPEC>;
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
impl From<crate::W<CMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPRE` reader - Channel Pre-scaler"]
pub type CPRE_R = crate::FieldReader<CPRESELECT_A>;
#[doc = "Channel Pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPRESELECT_A {
    #[doc = "0: Peripheral clock"]
    MCK = 0,
    #[doc = "1: Peripheral clock/2"]
    MCK_DIV_2 = 1,
    #[doc = "2: Peripheral clock/4"]
    MCK_DIV_4 = 2,
    #[doc = "3: Peripheral clock/8"]
    MCK_DIV_8 = 3,
    #[doc = "4: Peripheral clock/16"]
    MCK_DIV_16 = 4,
    #[doc = "5: Peripheral clock/32"]
    MCK_DIV_32 = 5,
    #[doc = "6: Peripheral clock/64"]
    MCK_DIV_64 = 6,
    #[doc = "7: Peripheral clock/128"]
    MCK_DIV_128 = 7,
    #[doc = "8: Peripheral clock/256"]
    MCK_DIV_256 = 8,
    #[doc = "9: Peripheral clock/512"]
    MCK_DIV_512 = 9,
    #[doc = "10: Peripheral clock/1024"]
    MCK_DIV_1024 = 10,
    #[doc = "11: Clock A"]
    CLKA = 11,
    #[doc = "12: Clock B"]
    CLKB = 12,
}
impl From<CPRESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CPRESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CPRESELECT_A {
    type Ux = u8;
}
impl CPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPRESELECT_A> {
        match self.bits {
            0 => Some(CPRESELECT_A::MCK),
            1 => Some(CPRESELECT_A::MCK_DIV_2),
            2 => Some(CPRESELECT_A::MCK_DIV_4),
            3 => Some(CPRESELECT_A::MCK_DIV_8),
            4 => Some(CPRESELECT_A::MCK_DIV_16),
            5 => Some(CPRESELECT_A::MCK_DIV_32),
            6 => Some(CPRESELECT_A::MCK_DIV_64),
            7 => Some(CPRESELECT_A::MCK_DIV_128),
            8 => Some(CPRESELECT_A::MCK_DIV_256),
            9 => Some(CPRESELECT_A::MCK_DIV_512),
            10 => Some(CPRESELECT_A::MCK_DIV_1024),
            11 => Some(CPRESELECT_A::CLKA),
            12 => Some(CPRESELECT_A::CLKB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CPRESELECT_A::MCK
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_2`"]
    #[inline(always)]
    pub fn is_mck_div_2(&self) -> bool {
        *self == CPRESELECT_A::MCK_DIV_2
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_4`"]
    #[inline(always)]
    pub fn is_mck_div_4(&self) -> bool {
        *self == CPRESELECT_A::MCK_DIV_4
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_8`"]
    #[inline(always)]
    pub fn is_mck_div_8(&self) -> bool {
        *self == CPRESELECT_A::MCK_DIV_8
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_16`"]
    #[inline(always)]
    pub fn is_mck_div_16(&self) -> bool {
        *self == CPRESELECT_A::MCK_DIV_16
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_32`"]
    #[inline(always)]
    pub fn is_mck_div_32(&self) -> bool {
        *self == CPRESELECT_A::MCK_DIV_32
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_64`"]
    #[inline(always)]
    pub fn is_mck_div_64(&self) -> bool {
        *self == CPRESELECT_A::MCK_DIV_64
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_128`"]
    #[inline(always)]
    pub fn is_mck_div_128(&self) -> bool {
        *self == CPRESELECT_A::MCK_DIV_128
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_256`"]
    #[inline(always)]
    pub fn is_mck_div_256(&self) -> bool {
        *self == CPRESELECT_A::MCK_DIV_256
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_512`"]
    #[inline(always)]
    pub fn is_mck_div_512(&self) -> bool {
        *self == CPRESELECT_A::MCK_DIV_512
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_1024`"]
    #[inline(always)]
    pub fn is_mck_div_1024(&self) -> bool {
        *self == CPRESELECT_A::MCK_DIV_1024
    }
    #[doc = "Checks if the value of the field is `CLKA`"]
    #[inline(always)]
    pub fn is_clka(&self) -> bool {
        *self == CPRESELECT_A::CLKA
    }
    #[doc = "Checks if the value of the field is `CLKB`"]
    #[inline(always)]
    pub fn is_clkb(&self) -> bool {
        *self == CPRESELECT_A::CLKB
    }
}
#[doc = "Field `CPRE` writer - Channel Pre-scaler"]
pub type CPRE_W<'a, const O: u8> = crate::FieldWriter<'a, CMR_SPEC, 4, O, CPRESELECT_A>;
impl<'a, const O: u8> CPRE_W<'a, O> {
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CPRESELECT_A::MCK)
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn mck_div_2(self) -> &'a mut W {
        self.variant(CPRESELECT_A::MCK_DIV_2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn mck_div_4(self) -> &'a mut W {
        self.variant(CPRESELECT_A::MCK_DIV_4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn mck_div_8(self) -> &'a mut W {
        self.variant(CPRESELECT_A::MCK_DIV_8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn mck_div_16(self) -> &'a mut W {
        self.variant(CPRESELECT_A::MCK_DIV_16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn mck_div_32(self) -> &'a mut W {
        self.variant(CPRESELECT_A::MCK_DIV_32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn mck_div_64(self) -> &'a mut W {
        self.variant(CPRESELECT_A::MCK_DIV_64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn mck_div_128(self) -> &'a mut W {
        self.variant(CPRESELECT_A::MCK_DIV_128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn mck_div_256(self) -> &'a mut W {
        self.variant(CPRESELECT_A::MCK_DIV_256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn mck_div_512(self) -> &'a mut W {
        self.variant(CPRESELECT_A::MCK_DIV_512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn mck_div_1024(self) -> &'a mut W {
        self.variant(CPRESELECT_A::MCK_DIV_1024)
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn clka(self) -> &'a mut W {
        self.variant(CPRESELECT_A::CLKA)
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn clkb(self) -> &'a mut W {
        self.variant(CPRESELECT_A::CLKB)
    }
}
#[doc = "Field `CALG` reader - Channel Alignment"]
pub type CALG_R = crate::BitReader<CALGSELECT_A>;
#[doc = "Channel Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALGSELECT_A {
    #[doc = "0: Left aligned"]
    LEFT_ALIGNED = 0,
    #[doc = "1: Center aligned"]
    CENTER_ALIGNED = 1,
}
impl From<CALGSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CALGSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CALG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALGSELECT_A {
        match self.bits {
            false => CALGSELECT_A::LEFT_ALIGNED,
            true => CALGSELECT_A::CENTER_ALIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT_ALIGNED`"]
    #[inline(always)]
    pub fn is_left_aligned(&self) -> bool {
        *self == CALGSELECT_A::LEFT_ALIGNED
    }
    #[doc = "Checks if the value of the field is `CENTER_ALIGNED`"]
    #[inline(always)]
    pub fn is_center_aligned(&self) -> bool {
        *self == CALGSELECT_A::CENTER_ALIGNED
    }
}
#[doc = "Field `CALG` writer - Channel Alignment"]
pub type CALG_W<'a, const O: u8> = crate::BitWriter<'a, CMR_SPEC, O, CALGSELECT_A>;
impl<'a, const O: u8> CALG_W<'a, O> {
    #[doc = "Left aligned"]
    #[inline(always)]
    pub fn left_aligned(self) -> &'a mut W {
        self.variant(CALGSELECT_A::LEFT_ALIGNED)
    }
    #[doc = "Center aligned"]
    #[inline(always)]
    pub fn center_aligned(self) -> &'a mut W {
        self.variant(CALGSELECT_A::CENTER_ALIGNED)
    }
}
#[doc = "Field `CPOL` reader - Channel Polarity"]
pub type CPOL_R = crate::BitReader<CPOLSELECT_A>;
#[doc = "Channel Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOLSELECT_A {
    #[doc = "0: Waveform starts at low level"]
    LOW_POLARITY = 0,
    #[doc = "1: Waveform starts at high level"]
    HIGH_POLARITY = 1,
}
impl From<CPOLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPOLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOLSELECT_A {
        match self.bits {
            false => CPOLSELECT_A::LOW_POLARITY,
            true => CPOLSELECT_A::HIGH_POLARITY,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_POLARITY`"]
    #[inline(always)]
    pub fn is_low_polarity(&self) -> bool {
        *self == CPOLSELECT_A::LOW_POLARITY
    }
    #[doc = "Checks if the value of the field is `HIGH_POLARITY`"]
    #[inline(always)]
    pub fn is_high_polarity(&self) -> bool {
        *self == CPOLSELECT_A::HIGH_POLARITY
    }
}
#[doc = "Field `CPOL` writer - Channel Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, CMR_SPEC, O, CPOLSELECT_A>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "Waveform starts at low level"]
    #[inline(always)]
    pub fn low_polarity(self) -> &'a mut W {
        self.variant(CPOLSELECT_A::LOW_POLARITY)
    }
    #[doc = "Waveform starts at high level"]
    #[inline(always)]
    pub fn high_polarity(self) -> &'a mut W {
        self.variant(CPOLSELECT_A::HIGH_POLARITY)
    }
}
#[doc = "Field `CES` reader - Counter Event Selection"]
pub type CES_R = crate::BitReader<CESSELECT_A>;
#[doc = "Counter Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CESSELECT_A {
    #[doc = "0: At the end of PWM period"]
    SINGLE_EVENT = 0,
    #[doc = "1: At half of PWM period AND at the end of PWM period"]
    DOUBLE_EVENT = 1,
}
impl From<CESSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CESSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CESSELECT_A {
        match self.bits {
            false => CESSELECT_A::SINGLE_EVENT,
            true => CESSELECT_A::DOUBLE_EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EVENT`"]
    #[inline(always)]
    pub fn is_single_event(&self) -> bool {
        *self == CESSELECT_A::SINGLE_EVENT
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EVENT`"]
    #[inline(always)]
    pub fn is_double_event(&self) -> bool {
        *self == CESSELECT_A::DOUBLE_EVENT
    }
}
#[doc = "Field `CES` writer - Counter Event Selection"]
pub type CES_W<'a, const O: u8> = crate::BitWriter<'a, CMR_SPEC, O, CESSELECT_A>;
impl<'a, const O: u8> CES_W<'a, O> {
    #[doc = "At the end of PWM period"]
    #[inline(always)]
    pub fn single_event(self) -> &'a mut W {
        self.variant(CESSELECT_A::SINGLE_EVENT)
    }
    #[doc = "At half of PWM period AND at the end of PWM period"]
    #[inline(always)]
    pub fn double_event(self) -> &'a mut W {
        self.variant(CESSELECT_A::DOUBLE_EVENT)
    }
}
#[doc = "Field `UPDS` reader - Update Selection"]
pub type UPDS_R = crate::BitReader<UPDSSELECT_A>;
#[doc = "Update Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDSSELECT_A {
    #[doc = "0: At the next end of PWM period"]
    UPDATE_AT_PERIOD = 0,
    #[doc = "1: At the next end of Half PWM period"]
    UPDATE_AT_HALF_PERIOD = 1,
}
impl From<UPDSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: UPDSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDSSELECT_A {
        match self.bits {
            false => UPDSSELECT_A::UPDATE_AT_PERIOD,
            true => UPDSSELECT_A::UPDATE_AT_HALF_PERIOD,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATE_AT_PERIOD`"]
    #[inline(always)]
    pub fn is_update_at_period(&self) -> bool {
        *self == UPDSSELECT_A::UPDATE_AT_PERIOD
    }
    #[doc = "Checks if the value of the field is `UPDATE_AT_HALF_PERIOD`"]
    #[inline(always)]
    pub fn is_update_at_half_period(&self) -> bool {
        *self == UPDSSELECT_A::UPDATE_AT_HALF_PERIOD
    }
}
#[doc = "Field `UPDS` writer - Update Selection"]
pub type UPDS_W<'a, const O: u8> = crate::BitWriter<'a, CMR_SPEC, O, UPDSSELECT_A>;
impl<'a, const O: u8> UPDS_W<'a, O> {
    #[doc = "At the next end of PWM period"]
    #[inline(always)]
    pub fn update_at_period(self) -> &'a mut W {
        self.variant(UPDSSELECT_A::UPDATE_AT_PERIOD)
    }
    #[doc = "At the next end of Half PWM period"]
    #[inline(always)]
    pub fn update_at_half_period(self) -> &'a mut W {
        self.variant(UPDSSELECT_A::UPDATE_AT_HALF_PERIOD)
    }
}
#[doc = "Field `DPOLI` reader - Disabled Polarity Inverted"]
pub type DPOLI_R = crate::BitReader;
#[doc = "Field `DPOLI` writer - Disabled Polarity Inverted"]
pub type DPOLI_W<'a, const O: u8> = crate::BitWriter<'a, CMR_SPEC, O>;
#[doc = "Field `TCTS` reader - Timer Counter Trigger Selection"]
pub type TCTS_R = crate::BitReader;
#[doc = "Field `TCTS` writer - Timer Counter Trigger Selection"]
pub type TCTS_W<'a, const O: u8> = crate::BitWriter<'a, CMR_SPEC, O>;
#[doc = "Field `DTE` reader - Dead-Time Generator Enable"]
pub type DTE_R = crate::BitReader;
#[doc = "Field `DTE` writer - Dead-Time Generator Enable"]
pub type DTE_W<'a, const O: u8> = crate::BitWriter<'a, CMR_SPEC, O>;
#[doc = "Field `DTHI` reader - Dead-Time PWMHx Output Inverted"]
pub type DTHI_R = crate::BitReader;
#[doc = "Field `DTHI` writer - Dead-Time PWMHx Output Inverted"]
pub type DTHI_W<'a, const O: u8> = crate::BitWriter<'a, CMR_SPEC, O>;
#[doc = "Field `DTLI` reader - Dead-Time PWMLx Output Inverted"]
pub type DTLI_R = crate::BitReader;
#[doc = "Field `DTLI` writer - Dead-Time PWMLx Output Inverted"]
pub type DTLI_W<'a, const O: u8> = crate::BitWriter<'a, CMR_SPEC, O>;
#[doc = "Field `PPM` reader - Push-Pull Mode"]
pub type PPM_R = crate::BitReader;
#[doc = "Field `PPM` writer - Push-Pull Mode"]
pub type PPM_W<'a, const O: u8> = crate::BitWriter<'a, CMR_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&self) -> CPRE_R {
        CPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&self) -> CALG_R {
        CALG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Update Selection"]
    #[inline(always)]
    pub fn upds(&self) -> UPDS_R {
        UPDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Disabled Polarity Inverted"]
    #[inline(always)]
    pub fn dpoli(&self) -> DPOLI_R {
        DPOLI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer Counter Trigger Selection"]
    #[inline(always)]
    pub fn tcts(&self) -> TCTS_R {
        TCTS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&self) -> DTE_R {
        DTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&self) -> DTHI_R {
        DTHI_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&self) -> DTLI_R {
        DTLI_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Push-Pull Mode"]
    #[inline(always)]
    pub fn ppm(&self) -> PPM_R {
        PPM_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    #[must_use]
    pub fn cpre(&mut self) -> CPRE_W<0> {
        CPRE_W::new(self)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    #[must_use]
    pub fn calg(&mut self) -> CALG_W<8> {
        CALG_W::new(self)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<9> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ces(&mut self) -> CES_W<10> {
        CES_W::new(self)
    }
    #[doc = "Bit 11 - Update Selection"]
    #[inline(always)]
    #[must_use]
    pub fn upds(&mut self) -> UPDS_W<11> {
        UPDS_W::new(self)
    }
    #[doc = "Bit 12 - Disabled Polarity Inverted"]
    #[inline(always)]
    #[must_use]
    pub fn dpoli(&mut self) -> DPOLI_W<12> {
        DPOLI_W::new(self)
    }
    #[doc = "Bit 13 - Timer Counter Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tcts(&mut self) -> TCTS_W<13> {
        TCTS_W::new(self)
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dte(&mut self) -> DTE_W<16> {
        DTE_W::new(self)
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    #[must_use]
    pub fn dthi(&mut self) -> DTHI_W<17> {
        DTHI_W::new(self)
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    #[must_use]
    pub fn dtli(&mut self) -> DTLI_W<18> {
        DTLI_W::new(self)
    }
    #[doc = "Bit 19 - Push-Pull Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ppm(&mut self) -> PPM_W<19> {
        PPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr](index.html) module"]
pub struct CMR_SPEC;
impl crate::RegisterSpec for CMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmr::R](R) reader structure"]
impl crate::Readable for CMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmr::W](W) writer structure"]
impl crate::Writable for CMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMR to value 0"]
impl crate::Resettable for CMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
